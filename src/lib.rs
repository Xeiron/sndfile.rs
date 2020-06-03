// Copyright 2020 tuxzz
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. This file may not be copied, modified,
// or distributed except according to those terms.

/*!
A safe rust wrapper of [libsndfile](http://www.mega-nerd.com/libsndfile/).  
With this crate, you can read or save audio files.

# Getting started
With minimal features:
````toml
[dependencies]
sndfile = "0.0"
````

With ndarray supports:
````toml
[dependencies.sndfile]
version = "0.0"
features = ["ndarray_features"]
````

# Example
```ignore
extern crate sndfile;
extern crate ndarray;

fn main() {
  use sndfile::*;
  let mut snd = sndfile::OpenOptions::ReadOnly(ReadOptions::Auto).from_path(
    "./sample_song.flac"
  ).unwrap();
  let data: ndarray::Array2<f32> = snd.read_all_to_ndarray().unwrap();

  let samplerate = snd.get_samplerate();
  let n_frame = snd.len().unwrap();
  let n_channels = snd.get_channels();
  let title = snd.get_tag(TagType::Title);
  println!("Loaded song `{}`:", title);
  println!("  Length: {:.2} seconds", n_frame as f64 / samplerate as f64);
  println!("  Sample rate: {} Hz", samplerate);
  println!("  Channel count: {}", n_channels);
  println!("  DC offset = {}", data.mean().unwrap());
}

/*
== Expected output ==
Loaded song `Loow`:
  Length: 277.06 seconds
  Sample rate: 44100 Hz
  Channel count: 2
  DC offset = 0.00018921464
*/
```

*/

#[macro_use]
extern crate lazy_static;

use sndfile_sys::sf_count_t;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::iter::FromIterator;
use std::os::raw::{c_int, c_void};
use std::path::Path;
use std::sync::Mutex;

mod format;
mod test;
pub use format::{
  check_format, default_subtype, get_supported_major_format_dict,
  get_supported_subtype_format_dict, Endian, MajorFormat, MajorInfo, SubtypeFormat, SubtypeInfo,
};

#[cfg(feature = "ndarray_features")]
mod ndarray_support;
#[cfg(feature = "ndarray_features")]
pub use ndarray_support::*;

lazy_static! {
  static ref SF_GLOBAL_LOCK: Mutex<()> = Mutex::new(());
}

#[derive(Debug)]
pub struct VIOFile {
  f: File,
}

extern "C" fn vio_get_filelen(user_data: *mut c_void) -> sf_count_t {
  let vio_file = unsafe { (user_data as *mut VIOFile).as_mut().unwrap() };
  vio_file.f.metadata().unwrap().len() as sf_count_t
}

extern "C" fn vio_seek(offset: sf_count_t, whence: c_int, user_data: *mut c_void) -> sf_count_t {
  let vio_file = unsafe { (user_data as *mut VIOFile).as_mut().unwrap() };
  let seek_from = match whence {
    sndfile_sys::SF_SEEK_SET => SeekFrom::Start(offset as u64),
    sndfile_sys::SF_SEEK_CUR => SeekFrom::Current(offset),
    sndfile_sys::SF_SEEK_END => SeekFrom::End(offset),
    _ => unreachable!(),
  };
  vio_file.f.seek(seek_from).unwrap() as sf_count_t
}

extern "C" fn vio_read(dst: *mut c_void, count: sf_count_t, user_data: *mut c_void) -> sf_count_t {
  let vio_file = unsafe { (user_data as *mut VIOFile).as_mut().unwrap() };
  let dst_buf = unsafe { std::slice::from_raw_parts_mut(dst as *mut u8, count as usize) };
  vio_file.f.read(dst_buf).unwrap() as sf_count_t
}

extern "C" fn vio_write(
  src: *const c_void,
  count: sf_count_t,
  user_data: *mut c_void,
) -> sf_count_t {
  let vio_file = unsafe { (user_data as *mut VIOFile).as_mut().unwrap() };
  let src_buf = unsafe { std::slice::from_raw_parts(src as *const u8, count as usize) };
  vio_file.f.write(src_buf).unwrap() as sf_count_t
}

extern "C" fn vio_tell(user_data: *mut c_void) -> sf_count_t {
  let vio_file = unsafe { (user_data as *mut VIOFile).as_mut().unwrap() };
  vio_file.f.seek(SeekFrom::Current(0)).unwrap() as sf_count_t
}

/// Options for reading audio files.
#[derive(Debug)]
pub enum ReadOptions {
  /// Auto detect format  
  Auto,
  /// `Raw(samplerate, channels)`: read as raw file.
  Raw(usize, usize),
}

/// Options for writing audio files.
#[derive(Debug)]
pub struct WriteOptions {
  major_format: format::MajorFormat,
  subtype_format: format::SubtypeFormat,
  endian: format::Endian,
  samplerate: usize,
  channels: usize,
}

impl WriteOptions {
  /// Create new WriteOptions.
  ///
  /// * `major_format`: Audio container format, e.g., `SubtypeFormat::WAV`, `SubtypeFormat::FLAC`, etc  
  /// * `subtype_format`: Audio encoding format, e.g., `SubtypeFormat::PCM_S16`, `SubtypeFormat::VORBIS`, etc  
  /// * `endian`: Usually `Endian::File`  
  /// * `samplerate`: A positive number  
  /// * `channels`: A positive number  
  pub fn new(
    major_format: format::MajorFormat,
    subtype_format: format::SubtypeFormat,
    endian: format::Endian,
    samplerate: usize,
    channels: usize,
  ) -> Self {
    assert!(samplerate > 0);
    assert!(channels > 0);
    WriteOptions {
      major_format,
      subtype_format,
      endian,
      samplerate,
      channels,
    }
  }

  /// This function allows the caller to check if a set of parameters in the WriteOptions is valid.
  ///
  /// Returns `Some(Self)` if the parameters are valid and `None` otherwise.
  pub fn validate(self) -> Option<Self> {
    if check_format(
      self.channels,
      self.samplerate,
      self.major_format,
      self.subtype_format,
      self.endian,
    ) {
      Some(self)
    } else {
      None
    }
  }
}

/// Struct to specify options when opening a audio file.  
#[derive(Debug)]
pub enum OpenOptions {
  /// Open an audio file read only.  
  ReadOnly(ReadOptions),
  /// Open an audio file write only.  
  ///
  /// In `OpenOptions::from_path` function, a new file will be created if the file does not yet already exist.  
  WriteOnly(WriteOptions),
  /// Open an audio file for reading and writing.  
  ///
  /// In `OpenOptions::from_path` function, a `SndFileError::IOError` will be returned if the file does not yet already exist.  
  ReadWrite(ReadOptions),
  /// Open an audio file for reading and writing.  
  ///
  /// In `OpenOptions::from_path` function, a new file will be created if the file does not yet already exist.  
  /// If specified file already exists, format may differ from `WriteOptions`  
  WriteRead(WriteOptions),
}

/// This struct is unstable.
#[derive(Debug)]
pub struct UnsafeSndFile {
  pub vio_ptr: *mut sndfile_sys::SF_VIRTUAL_IO,
  pub vio_user_ptr: *mut VIOFile,
  pub sndfile_ptr: *mut sndfile_sys::SNDFILE,
}

/// Main struct of this crate.
#[derive(Debug)]
pub struct SndFile {
  unsafe_fields: UnsafeSndFile,
  samplerate: usize,
  channels: usize,
  major_format: MajorFormat,
  subtype_format: SubtypeFormat,
  endian: Endian,
  seekable: bool,
}

/// Do I/O operation on slice or iterator.
pub trait SndFileIO<T>
where
  T: 'static + Default + Copy,
{
  /// Read frames from current I/O cursor, returns the number of frames read if success.
  ///
  /// This function may affect the I/O cursor.
  fn read_to_slice(&mut self, dst: &mut [T]) -> Result<usize, ()>;
  /// Read frames from file, returns the number of frames written if success.
  ///
  /// This function may affect the I/O cursor.
  fn write_from_slice(&mut self, src: &[T]) -> Result<usize, ()>;
  /// Read all frames into a `Vec<_>` if success.
  ///
  /// This function may affect the I/O cursor.
  fn read_all_to_vec(&mut self) -> Result<Vec<T>, ()>;

  /// Read frames from current I/O cursor, returns the number of frames read if success.
  ///
  /// This function may affect the I/O cursor.
  fn read_to_iter<'a, I>(&mut self, dst: I) -> Result<usize, ()>
  where
    I: ExactSizeIterator<Item = &'a mut T>,
  {
    let mut buf = vec![Default::default(); dst.len()];
    self.read_to_slice(&mut buf).map(|r| {
      dst.zip(buf.into_iter()).for_each(|(x, y)| *x = y);
      r
    })
  }

  /// Read frames from file, returns the number of frames written if success.
  ///
  /// This function may affect the I/O cursor.
  fn write_from_iter<'a, I>(&mut self, src: I) -> Result<usize, ()>
  where
    I: ExactSizeIterator<Item = T>,
  {
    let buf = Vec::<T>::from_iter(src);
    self.write_from_slice(&buf[..])
  }
}

#[derive(Debug)]
pub enum SndFileError {
  UnrecognisedFormat(String),
  SystemError(String),
  MalformedFile(String),
  UnsupportedEncoding(String),
  InvalidParameter(String),
  InternalError(String),
  IOError(std::io::Error),
}

#[derive(Debug)]
/// Type of tags
pub enum TagType {
  Title,
  Copyright,
  Software,
  Artist,
  Comment,
  Date,
  Album,
  License,
  Tracknumber,
  Genre,
}

/// Lock it before interacting with a few raw `libsndfile` functions in multithread context.
///
/// Affected functions:
/// * `sf_open(...)`
/// * `sf_error(nullptr)`
/// * `sf_strerror(nullptr)`
/// * `sf_perror(nullptr)`
/// * `sf_error_str(nullptr, ...)`
pub fn get_sf_global_lock() -> &'static Mutex<()> {
  &SF_GLOBAL_LOCK
}

fn sf_err_code_to_enum(err_code: c_int) -> SndFileError {
  match err_code {
    sndfile_sys::SF_ERR_NO_ERROR => panic!("Errrrrrr"),
    _ => {
      let err_msg = unsafe {
        std::ffi::CStr::from_ptr(sndfile_sys::sf_error_number(err_code))
          .to_str()
          .unwrap()
      }
      .to_string();
      match err_code {
        sndfile_sys::SF_ERR_UNRECOGNISED_FORMAT => SndFileError::UnrecognisedFormat(err_msg),
        sndfile_sys::SF_ERR_SYSTEM => SndFileError::SystemError(err_msg),
        sndfile_sys::SF_ERR_MALFORMED_FILE => SndFileError::MalformedFile(err_msg),
        sndfile_sys::SF_ERR_UNSUPPORTED_ENCODING => SndFileError::UnsupportedEncoding(err_msg),
        _ => SndFileError::InternalError(err_msg),
      }
    }
  }
}

impl OpenOptions {
  /// Open from path
  pub fn from_path<P: AsRef<Path>>(&self, path: P) -> Result<SndFile, SndFileError> {
    let file_obj = match self {
      Self::ReadOnly(_) => std::fs::OpenOptions::new().read(true).open(path),
      Self::WriteOnly(_) => std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path),
      Self::ReadWrite(_) => std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path),
      Self::WriteRead(_) => std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path),
    }
    .map_err(|e| SndFileError::IOError(e))?;
    self.from_file(file_obj)
  }

  /// Open from file
  pub fn from_file(&self, f: File) -> Result<SndFile, SndFileError> {
    let sf_open_mode = match self {
      Self::ReadOnly(_) => sndfile_sys::SFM_READ,
      Self::WriteOnly(_) => sndfile_sys::SFM_WRITE,
      Self::ReadWrite(_) | Self::WriteRead(_) => sndfile_sys::SFM_RDWR,
    };
    let mut sf_info = match self {
      OpenOptions::ReadOnly(ReadOptions::Auto) | OpenOptions::ReadWrite(ReadOptions::Auto) => {
        sndfile_sys::SF_INFO {
          frames: 0,
          samplerate: 0,
          channels: 0,
          format: 0,
          sections: 0,
          seekable: 0,
        }
      }
      OpenOptions::ReadOnly(ReadOptions::Raw(samplerate, channels))
      | OpenOptions::ReadWrite(ReadOptions::Raw(samplerate, channels)) => sndfile_sys::SF_INFO {
        frames: 0,
        samplerate: *samplerate as c_int,
        channels: *channels as c_int,
        format: sndfile_sys::SF_FORMAT_RAW,
        sections: 0,
        seekable: 0,
      },
      OpenOptions::WriteOnly(x) | OpenOptions::WriteRead(x) => sndfile_sys::SF_INFO {
        frames: 0,
        samplerate: x.samplerate as c_int,
        channels: x.channels as c_int,
        format: format::assembly_format_flags(x.major_format, x.subtype_format, x.endian),
        sections: 0,
        seekable: 0,
      },
    };
    let vio_ptr = Box::into_raw(Box::new(sndfile_sys::SF_VIRTUAL_IO {
      get_filelen: vio_get_filelen,
      seek: vio_seek,
      read: vio_read,
      write: vio_write,
      tell: vio_tell,
    }));
    let vio_user_ptr = Box::into_raw(Box::new(VIOFile { f }));
    {
      let _sf_global_lock_guard = SF_GLOBAL_LOCK.lock();
      let sndfile_ptr = unsafe {
        sndfile_sys::sf_open_virtual(
          vio_ptr,
          sf_open_mode,
          &mut sf_info as *mut sndfile_sys::SF_INFO,
          vio_user_ptr as *mut c_void,
        )
      };
      if sndfile_ptr.is_null() {
        unsafe {
          Box::from_raw(vio_user_ptr);
          Box::from_raw(vio_ptr);
        }
        Err(sf_err_code_to_enum(unsafe {
          sndfile_sys::sf_error(sndfile_ptr)
        }))
      } else {
        let u = UnsafeSndFile {
          vio_ptr,
          vio_user_ptr,
          sndfile_ptr,
        };

        if sf_info.frames < 0 {
          Err(SndFileError::InvalidParameter(
            "Got invalid frame count, expect a non-negative number.".to_string(),
          ))
        } else if sf_info.samplerate <= 0 {
          Err(SndFileError::InvalidParameter(
            "Got invalid samplerate, expect a positive number.".to_string(),
          ))
        } else if sf_info.channels <= 0 {
          Err(SndFileError::InvalidParameter(
            "Got invalid channels, expect a positive number.".to_string(),
          ))
        } else {
          let major_format = format::flags_to_major_format(sf_info.format);
          let subtype_format = format::flags_to_subtype_format(sf_info.format);
          let endian_format = format::flags_to_endian(sf_info.format);
          if major_format.is_none() || subtype_format.is_none() || endian_format.is_none() {
            Err(SndFileError::InvalidParameter(
              "Got invalid format flags.".to_string(),
            ))
          } else {
            unsafe {
              sndfile_sys::sf_command(
                u.sndfile_ptr,
                sndfile_sys::SFC_SET_SCALE_FLOAT_INT_READ,
                std::ptr::null_mut(),
                sndfile_sys::SF_TRUE,
              )
            };
            unsafe {
              sndfile_sys::sf_command(
                u.sndfile_ptr,
                sndfile_sys::SFC_SET_SCALE_INT_FLOAT_WRITE,
                std::ptr::null_mut(),
                sndfile_sys::SF_TRUE,
              )
            };
            Ok(SndFile {
              unsafe_fields: u,
              samplerate: sf_info.samplerate as usize,
              channels: sf_info.channels as usize,
              major_format: major_format.unwrap(),
              subtype_format: subtype_format.unwrap(),
              endian: endian_format.unwrap(),
              seekable: sf_info.seekable != sndfile_sys::SF_FALSE,
            })
          }
        }
      }
    }
  }
}

impl Drop for UnsafeSndFile {
  fn drop(&mut self) {
    let err_code = unsafe { sndfile_sys::sf_close(self.sndfile_ptr) };
    unsafe {
      Box::from_raw(self.vio_user_ptr);
      Box::from_raw(self.vio_ptr);
    }
    if err_code != 0 {
      let err_msg = unsafe {
        std::ffi::CStr::from_ptr(sndfile_sys::sf_error_number(err_code))
          .to_str()
          .unwrap()
      };
      panic!(format!("Failed to call `sf_close`: {}", err_msg));
    }
  }
}

impl SndFileIO<i16> for SndFile {
  fn read_to_slice(&mut self, dst: &mut [i16]) -> Result<usize, ()> {
    let len = dst.len();
    let n_ch = self.channels as usize;
    let n_elem = len / n_ch;
    assert!(len % n_ch == 0);
    let n = unsafe {
      sndfile_sys::sf_readf_short(
        self.unsafe_fields.sndfile_ptr,
        dst.as_mut_ptr(),
        n_elem as sf_count_t,
      )
    };
    if n >= 0 {
      Ok(n as usize)
    } else {
      Err(())
    }
  }

  fn write_from_slice(&mut self, src: &[i16]) -> Result<usize, ()> {
    let len = src.len();
    let n_ch = self.channels as usize;
    let n_elem = len / n_ch;
    let n = unsafe {
      sndfile_sys::sf_writef_short(
        self.unsafe_fields.sndfile_ptr,
        src.as_ptr(),
        n_elem as sf_count_t,
      )
    };
    if n >= 0 {
      Ok(n as usize)
    } else {
      Err(())
    }
  }

  fn read_all_to_vec(&mut self) -> Result<Vec<i16>, ()> {
    let n = self.len()? as usize * self.channels;
    self.seek(SeekFrom::Start(0))?;
    let mut buf = vec![0; n];
    self.read_to_slice(&mut buf).map(|_| buf)
  }
}

impl SndFileIO<i32> for SndFile {
  fn read_to_slice(&mut self, dst: &mut [i32]) -> Result<usize, ()> {
    let len = dst.len();
    let n_ch = self.channels as usize;
    let n_elem = len / n_ch;
    let n = unsafe {
      sndfile_sys::sf_readf_int(
        self.unsafe_fields.sndfile_ptr,
        dst.as_mut_ptr(),
        n_elem as sf_count_t,
      )
    };
    if n >= 0 {
      Ok(n as usize)
    } else {
      Err(())
    }
  }

  fn write_from_slice(&mut self, src: &[i32]) -> Result<usize, ()> {
    let len = src.len();
    let n_ch = self.channels as usize;
    let n_elem = len / n_ch;
    let n = unsafe {
      sndfile_sys::sf_writef_int(
        self.unsafe_fields.sndfile_ptr,
        src.as_ptr(),
        n_elem as sf_count_t,
      )
    };
    if n >= 0 {
      Ok(n as usize)
    } else {
      Err(())
    }
  }

  fn read_all_to_vec(&mut self) -> Result<Vec<i32>, ()> {
    let n = self.len()? as usize * self.channels;
    self.seek(SeekFrom::Start(0))?;
    let mut buf = vec![0; n];
    self.read_to_slice(&mut buf).map(|_| buf)
  }
}

impl SndFileIO<f32> for SndFile {
  fn read_to_slice(&mut self, dst: &mut [f32]) -> Result<usize, ()> {
    let len = dst.len();
    let n_ch = self.channels as usize;
    let n_elem = len / n_ch;
    let n = unsafe {
      sndfile_sys::sf_readf_float(
        self.unsafe_fields.sndfile_ptr,
        dst.as_mut_ptr(),
        n_elem as sf_count_t,
      )
    };
    if n >= 0 {
      Ok(n as usize)
    } else {
      Err(())
    }
  }

  fn write_from_slice(&mut self, src: &[f32]) -> Result<usize, ()> {
    let len = src.len();
    let n_ch = self.channels as usize;
    let n_elem = len / n_ch;
    let n = unsafe {
      sndfile_sys::sf_writef_float(
        self.unsafe_fields.sndfile_ptr,
        src.as_ptr(),
        n_elem as sf_count_t,
      )
    };
    if n >= 0 {
      Ok(n as usize)
    } else {
      Err(())
    }
  }

  fn read_all_to_vec(&mut self) -> Result<Vec<f32>, ()> {
    let n = self.len()? as usize * self.channels;
    self.seek(SeekFrom::Start(0))?;
    let mut buf = vec![0.0; n];
    self.read_to_slice(&mut buf).map(|_| buf)
  }
}

impl SndFileIO<f64> for SndFile {
  fn read_to_slice(&mut self, dst: &mut [f64]) -> Result<usize, ()> {
    let len = dst.len();
    let n_ch = self.channels as usize;
    let n_elem = len / n_ch;
    let n = unsafe {
      sndfile_sys::sf_readf_double(
        self.unsafe_fields.sndfile_ptr,
        dst.as_mut_ptr(),
        n_elem as sf_count_t,
      )
    };
    if n >= 0 {
      Ok(n as usize)
    } else {
      Err(())
    }
  }

  fn write_from_slice(&mut self, src: &[f64]) -> Result<usize, ()> {
    let len = src.len();
    let n_ch = self.channels as usize;
    let n_elem = len / n_ch;
    let n = unsafe {
      sndfile_sys::sf_writef_double(
        self.unsafe_fields.sndfile_ptr,
        src.as_ptr(),
        n_elem as sf_count_t,
      )
    };
    if n >= 0 {
      Ok(n as usize)
    } else {
      Err(())
    }
  }

  fn read_all_to_vec(&mut self) -> Result<Vec<f64>, ()> {
    let n = self.len()? as usize * self.channels;
    self.seek(SeekFrom::Start(0))?;
    let mut buf = vec![0.0; n];
    self.read_to_slice(&mut buf).map(|_| buf)
  }
}

fn tag_type_to_flags(t: TagType) -> c_int {
  match t {
    TagType::Title => sndfile_sys::SF_STR_TITLE,
    TagType::Copyright => sndfile_sys::SF_STR_COPYRIGHT,
    TagType::Software => sndfile_sys::SF_STR_SOFTWARE,
    TagType::Artist => sndfile_sys::SF_STR_ARTIST,
    TagType::Comment => sndfile_sys::SF_STR_COMMENT,
    TagType::Date => sndfile_sys::SF_STR_DATE,
    TagType::Album => sndfile_sys::SF_STR_ALBUM,
    TagType::License => sndfile_sys::SF_STR_LICENSE,
    TagType::Tracknumber => sndfile_sys::SF_STR_TRACKNUMBER,
    TagType::Genre => sndfile_sys::SF_STR_GENRE,
  }
}

impl SndFile {
  /// Get sample rate.
  ///
  /// Return values should be greater than zero.
  pub fn get_samplerate(&self) -> usize {
    self.samplerate
  }

  /// Get channel count.
  ///
  /// Return values should be greater than zero.
  pub fn get_channels(&self) -> usize {
    self.channels
  }

  /// Get audio container format
  pub fn get_major_format(&self) -> MajorFormat {
    self.major_format
  }

  /// Get audio encoding format
  pub fn get_subtype_format(&self) -> SubtypeFormat {
    self.subtype_format
  }

  /// Get audio file endian
  ///
  /// Usually returns `Endian::File`
  pub fn get_endian(&self) -> Endian {
    self.endian
  }

  /// Check if this file seekable
  ///
  /// If not, many functions like `len` or `read_all_to_vec` will return an error.
  pub fn is_seekable(&self) -> bool {
    self.seekable
  }

  /// Useful if you want to do something unsafe.
  pub fn get_raw_struct(&self) -> &UnsafeSndFile {
    &self.unsafe_fields
  }

  /// Get tag string, e.g., artist, album, etc.
  pub fn get_tag(&self, t: TagType) -> String {
    let s_ptr =
      unsafe { sndfile_sys::sf_get_string(self.unsafe_fields.sndfile_ptr, tag_type_to_flags(t)) };
    let c_str = unsafe { std::ffi::CStr::from_ptr(s_ptr) };
    c_str.to_string_lossy().into_owned()
  }

  /// Set tag string
  pub fn set_tag(&mut self, t: TagType, v: &str) -> Result<(), SndFileError> {
    let c_str = std::ffi::CString::new(v).unwrap();
    let ret_code = unsafe {
      sndfile_sys::sf_set_string(
        self.unsafe_fields.sndfile_ptr,
        tag_type_to_flags(t),
        c_str.as_ptr(),
      )
    };
    if ret_code == 0 {
      Ok(())
    } else {
      Err(sf_err_code_to_enum(ret_code))
    }
  }

  /// Modify the I/O cursor.
  pub fn seek(&mut self, pos: SeekFrom) -> Result<u64, ()> {
    if self.is_seekable() {
      let r = unsafe {
        match pos {
          SeekFrom::Start(x) => sndfile_sys::sf_seek(
            self.unsafe_fields.sndfile_ptr,
            x as sf_count_t,
            sndfile_sys::SF_SEEK_SET,
          ),
          SeekFrom::Current(x) => sndfile_sys::sf_seek(
            self.unsafe_fields.sndfile_ptr,
            x as sf_count_t,
            sndfile_sys::SF_SEEK_CUR,
          ),
          SeekFrom::End(x) => sndfile_sys::sf_seek(
            self.unsafe_fields.sndfile_ptr,
            x as sf_count_t,
            sndfile_sys::SF_SEEK_END,
          ),
        }
      };
      if r >= 0 {
        Ok(r as u64)
      } else {
        Err(())
      }
    } else {
      Err(())
    }
  }

  /// Get the length of audio file.
  ///
  /// This function may affect the I/O cursor.
  pub fn len(&mut self) -> Result<u64, ()> {
    self.seek(SeekFrom::End(0))
  }
}

unsafe impl std::marker::Send for SndFile {}
