use std::collections::HashMap;
use std::os::raw::{c_int, c_void};

#[derive(Debug)]
pub struct MajorInfo {
  pub name: String,
  pub extension: String,
}

#[derive(Debug)]
pub struct SubtypeInfo {
  pub name: String,
}

lazy_static! {
  static ref MAJOR_FORMAT_LIST: HashMap<MajorFormat, MajorInfo> = {
    let mut n: c_int = 0;
    unsafe {
      sndfile_sys::sf_command(
        std::ptr::null_mut(),
        sndfile_sys::SFC_GET_FORMAT_MAJOR_COUNT,
        &mut n as *mut c_int as *mut c_void,
        std::mem::size_of::<c_int>() as c_int,
      )
    };
    assert!(n >= 0);

    let mut fmt_info = sndfile_sys::SF_FORMAT_INFO {
      format: 0,
      name: std::ptr::null(),
      extension: std::ptr::null(),
    };
    let mut out = HashMap::new();
    for i in 0..n {
      fmt_info.format = i;
      unsafe {
        sndfile_sys::sf_command(
          std::ptr::null_mut(),
          sndfile_sys::SFC_GET_FORMAT_MAJOR,
          &mut fmt_info as *mut sndfile_sys::SF_FORMAT_INFO as *mut c_void,
          std::mem::size_of::<sndfile_sys::SF_FORMAT_INFO>() as c_int,
        )
      };
      match flags_to_major_format(fmt_info.format) {
        Some(major_format) => {
          let name = unsafe { std::ffi::CStr::from_ptr(fmt_info.name) }
            .to_str()
            .unwrap();
          let extension = unsafe { std::ffi::CStr::from_ptr(fmt_info.extension) }
            .to_str()
            .unwrap();
          out.insert(
            major_format,
            MajorInfo {
              name: name.to_string(),
              extension: extension.to_string(),
            },
          );
        }
        _ => {}
      }
    }
    out
  };
  static ref SUBTYPE_FORMAT_LIST: HashMap<SubtypeFormat, SubtypeInfo> = {
    let mut n: c_int = 0;
    unsafe {
      sndfile_sys::sf_command(
        std::ptr::null_mut(),
        sndfile_sys::SFC_GET_FORMAT_SUBTYPE_COUNT,
        &mut n as *mut c_int as *mut c_void,
        std::mem::size_of::<c_int>() as c_int,
      )
    };
    assert!(n >= 0);

    let mut fmt_info = sndfile_sys::SF_FORMAT_INFO {
      format: 0,
      name: std::ptr::null(),
      extension: std::ptr::null(),
    };
    let mut out = HashMap::new();
    for i in 0..n {
      fmt_info.format = i;
      unsafe {
        sndfile_sys::sf_command(
          std::ptr::null_mut(),
          sndfile_sys::SFC_GET_FORMAT_SUBTYPE,
          &mut fmt_info as *mut sndfile_sys::SF_FORMAT_INFO as *mut c_void,
          std::mem::size_of::<sndfile_sys::SF_FORMAT_INFO>() as c_int,
        )
      };
      match flags_to_subtype_format(fmt_info.format) {
        Some(subtype_format) => {
          let name = unsafe { std::ffi::CStr::from_ptr(fmt_info.name) }
            .to_str()
            .unwrap();
          out.insert(
            subtype_format,
            SubtypeInfo {
              name: name.to_string(),
            },
          );
        }
        _ => {}
      }
    }
    out
  };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MajorFormat {
  WAV,
  AIFF,
  AU,
  RAW,
  PAF,
  SVX,
  NIST,
  VOC,
  IRCAM,
  W64,
  MAT4,
  MAT5,
  PVF,
  XI,
  HTK,
  SDS,
  AVR,
  WAVEX,
  SD2,
  FLAC,
  CAF,
  WVE,
  OGG,
  MPC2K,
  RF64,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SubtypeFormat {
  PCM_S8,
  PCM_16,
  PCM_24,
  PCM_32,
  PCM_U8,
  FLOAT,
  DOUBLE,
  ULAW,
  ALAW,
  IMA_ADPCM,
  MS_ADPCM,
  GSM610,
  VOX_ADPCM,
  G721_32,
  G723_24,
  G723_40,
  DWVW_12,
  DWVW_16,
  DWVW_24,
  DWVW_N,
  DPCM_8,
  DPCM_16,
  VORBIS,
  ALAC_16,
  ALAC_20,
  ALAC_24,
  ALAC_32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Endian {
  File,
  Little,
  Big,
  CPU,
}

pub fn flags_to_major_format(flags: c_int) -> Option<MajorFormat> {
  match flags & sndfile_sys::SF_FORMAT_TYPEMASK {
    sndfile_sys::SF_FORMAT_WAV => Some(MajorFormat::WAV),
    sndfile_sys::SF_FORMAT_AIFF => Some(MajorFormat::AIFF),
    sndfile_sys::SF_FORMAT_AU => Some(MajorFormat::AU),
    sndfile_sys::SF_FORMAT_RAW => Some(MajorFormat::RAW),
    sndfile_sys::SF_FORMAT_PAF => Some(MajorFormat::PAF),
    sndfile_sys::SF_FORMAT_SVX => Some(MajorFormat::SVX),
    sndfile_sys::SF_FORMAT_NIST => Some(MajorFormat::NIST),
    sndfile_sys::SF_FORMAT_VOC => Some(MajorFormat::VOC),
    sndfile_sys::SF_FORMAT_IRCAM => Some(MajorFormat::IRCAM),
    sndfile_sys::SF_FORMAT_W64 => Some(MajorFormat::W64),
    sndfile_sys::SF_FORMAT_MAT4 => Some(MajorFormat::MAT4),
    sndfile_sys::SF_FORMAT_MAT5 => Some(MajorFormat::MAT5),
    sndfile_sys::SF_FORMAT_PVF => Some(MajorFormat::PVF),
    sndfile_sys::SF_FORMAT_XI => Some(MajorFormat::XI),
    sndfile_sys::SF_FORMAT_HTK => Some(MajorFormat::HTK),
    sndfile_sys::SF_FORMAT_SDS => Some(MajorFormat::SDS),
    sndfile_sys::SF_FORMAT_AVR => Some(MajorFormat::AVR),
    sndfile_sys::SF_FORMAT_WAVEX => Some(MajorFormat::WAVEX),
    sndfile_sys::SF_FORMAT_SD2 => Some(MajorFormat::SD2),
    sndfile_sys::SF_FORMAT_FLAC => Some(MajorFormat::FLAC),
    sndfile_sys::SF_FORMAT_CAF => Some(MajorFormat::CAF),
    sndfile_sys::SF_FORMAT_WVE => Some(MajorFormat::WVE),
    sndfile_sys::SF_FORMAT_OGG => Some(MajorFormat::OGG),
    sndfile_sys::SF_FORMAT_MPC2K => Some(MajorFormat::MPC2K),
    sndfile_sys::SF_FORMAT_RF64 => Some(MajorFormat::RF64),
    _ => None,
  }
}

pub fn flags_to_subtype_format(flags: c_int) -> Option<SubtypeFormat> {
  match flags & sndfile_sys::SF_FORMAT_SUBMASK {
    sndfile_sys::SF_FORMAT_PCM_S8 => Some(SubtypeFormat::PCM_S8),
    sndfile_sys::SF_FORMAT_PCM_16 => Some(SubtypeFormat::PCM_16),
    sndfile_sys::SF_FORMAT_PCM_24 => Some(SubtypeFormat::PCM_24),
    sndfile_sys::SF_FORMAT_PCM_32 => Some(SubtypeFormat::PCM_32),
    sndfile_sys::SF_FORMAT_PCM_U8 => Some(SubtypeFormat::PCM_U8),
    sndfile_sys::SF_FORMAT_FLOAT => Some(SubtypeFormat::FLOAT),
    sndfile_sys::SF_FORMAT_DOUBLE => Some(SubtypeFormat::DOUBLE),
    sndfile_sys::SF_FORMAT_ULAW => Some(SubtypeFormat::ULAW),
    sndfile_sys::SF_FORMAT_ALAW => Some(SubtypeFormat::ALAW),
    sndfile_sys::SF_FORMAT_IMA_ADPCM => Some(SubtypeFormat::IMA_ADPCM),
    sndfile_sys::SF_FORMAT_MS_ADPCM => Some(SubtypeFormat::MS_ADPCM),
    sndfile_sys::SF_FORMAT_GSM610 => Some(SubtypeFormat::GSM610),
    sndfile_sys::SF_FORMAT_VOX_ADPCM => Some(SubtypeFormat::VOX_ADPCM),
    sndfile_sys::SF_FORMAT_G721_32 => Some(SubtypeFormat::G721_32),
    sndfile_sys::SF_FORMAT_G723_24 => Some(SubtypeFormat::G723_24),
    sndfile_sys::SF_FORMAT_G723_40 => Some(SubtypeFormat::G723_40),
    sndfile_sys::SF_FORMAT_DWVW_12 => Some(SubtypeFormat::DWVW_12),
    sndfile_sys::SF_FORMAT_DWVW_16 => Some(SubtypeFormat::DWVW_16),
    sndfile_sys::SF_FORMAT_DWVW_24 => Some(SubtypeFormat::DWVW_24),
    sndfile_sys::SF_FORMAT_DWVW_N => Some(SubtypeFormat::DWVW_N),
    sndfile_sys::SF_FORMAT_DPCM_8 => Some(SubtypeFormat::DPCM_8),
    sndfile_sys::SF_FORMAT_DPCM_16 => Some(SubtypeFormat::DPCM_16),
    sndfile_sys::SF_FORMAT_VORBIS => Some(SubtypeFormat::VORBIS),
    sndfile_sys::SF_FORMAT_ALAC_16 => Some(SubtypeFormat::ALAC_16),
    sndfile_sys::SF_FORMAT_ALAC_20 => Some(SubtypeFormat::ALAC_20),
    sndfile_sys::SF_FORMAT_ALAC_24 => Some(SubtypeFormat::ALAC_24),
    sndfile_sys::SF_FORMAT_ALAC_32 => Some(SubtypeFormat::ALAC_32),
    _ => None,
  }
}

pub fn flags_to_endian(flags: c_int) -> Option<Endian> {
  match flags & sndfile_sys::SF_FORMAT_ENDMASK {
    sndfile_sys::SF_ENDIAN_FILE => Some(Endian::File),
    sndfile_sys::SF_ENDIAN_LITTLE => Some(Endian::Little),
    sndfile_sys::SF_ENDIAN_BIG => Some(Endian::Big),
    sndfile_sys::SF_ENDIAN_CPU => Some(Endian::CPU),
    _ => None,
  }
}

pub fn major_format_to_flags(major_format: MajorFormat) -> c_int {
  match major_format {
    MajorFormat::WAV => sndfile_sys::SF_FORMAT_WAV,
    MajorFormat::AIFF => sndfile_sys::SF_FORMAT_AIFF,
    MajorFormat::AU => sndfile_sys::SF_FORMAT_AU,
    MajorFormat::RAW => sndfile_sys::SF_FORMAT_RAW,
    MajorFormat::PAF => sndfile_sys::SF_FORMAT_PAF,
    MajorFormat::SVX => sndfile_sys::SF_FORMAT_SVX,
    MajorFormat::NIST => sndfile_sys::SF_FORMAT_NIST,
    MajorFormat::VOC => sndfile_sys::SF_FORMAT_VOC,
    MajorFormat::IRCAM => sndfile_sys::SF_FORMAT_IRCAM,
    MajorFormat::W64 => sndfile_sys::SF_FORMAT_W64,
    MajorFormat::MAT4 => sndfile_sys::SF_FORMAT_MAT4,
    MajorFormat::MAT5 => sndfile_sys::SF_FORMAT_MAT5,
    MajorFormat::PVF => sndfile_sys::SF_FORMAT_PVF,
    MajorFormat::XI => sndfile_sys::SF_FORMAT_XI,
    MajorFormat::HTK => sndfile_sys::SF_FORMAT_HTK,
    MajorFormat::SDS => sndfile_sys::SF_FORMAT_SDS,
    MajorFormat::AVR => sndfile_sys::SF_FORMAT_AVR,
    MajorFormat::WAVEX => sndfile_sys::SF_FORMAT_WAVEX,
    MajorFormat::SD2 => sndfile_sys::SF_FORMAT_SD2,
    MajorFormat::FLAC => sndfile_sys::SF_FORMAT_FLAC,
    MajorFormat::CAF => sndfile_sys::SF_FORMAT_CAF,
    MajorFormat::WVE => sndfile_sys::SF_FORMAT_WVE,
    MajorFormat::OGG => sndfile_sys::SF_FORMAT_OGG,
    MajorFormat::MPC2K => sndfile_sys::SF_FORMAT_MPC2K,
    MajorFormat::RF64 => sndfile_sys::SF_FORMAT_RF64,
  }
}

pub fn subtype_format_to_flags(subtype_format: SubtypeFormat) -> c_int {
  match subtype_format {
    SubtypeFormat::PCM_S8 => sndfile_sys::SF_FORMAT_PCM_S8,
    SubtypeFormat::PCM_16 => sndfile_sys::SF_FORMAT_PCM_16,
    SubtypeFormat::PCM_24 => sndfile_sys::SF_FORMAT_PCM_24,
    SubtypeFormat::PCM_32 => sndfile_sys::SF_FORMAT_PCM_32,
    SubtypeFormat::PCM_U8 => sndfile_sys::SF_FORMAT_PCM_U8,
    SubtypeFormat::FLOAT => sndfile_sys::SF_FORMAT_FLOAT,
    SubtypeFormat::DOUBLE => sndfile_sys::SF_FORMAT_DOUBLE,
    SubtypeFormat::ULAW => sndfile_sys::SF_FORMAT_ULAW,
    SubtypeFormat::ALAW => sndfile_sys::SF_FORMAT_ALAW,
    SubtypeFormat::IMA_ADPCM => sndfile_sys::SF_FORMAT_IMA_ADPCM,
    SubtypeFormat::MS_ADPCM => sndfile_sys::SF_FORMAT_MS_ADPCM,
    SubtypeFormat::GSM610 => sndfile_sys::SF_FORMAT_GSM610,
    SubtypeFormat::VOX_ADPCM => sndfile_sys::SF_FORMAT_VOX_ADPCM,
    SubtypeFormat::G721_32 => sndfile_sys::SF_FORMAT_G721_32,
    SubtypeFormat::G723_24 => sndfile_sys::SF_FORMAT_G723_24,
    SubtypeFormat::G723_40 => sndfile_sys::SF_FORMAT_G723_40,
    SubtypeFormat::DWVW_12 => sndfile_sys::SF_FORMAT_DWVW_12,
    SubtypeFormat::DWVW_16 => sndfile_sys::SF_FORMAT_DWVW_16,
    SubtypeFormat::DWVW_24 => sndfile_sys::SF_FORMAT_DWVW_24,
    SubtypeFormat::DWVW_N => sndfile_sys::SF_FORMAT_DWVW_N,
    SubtypeFormat::DPCM_8 => sndfile_sys::SF_FORMAT_DPCM_8,
    SubtypeFormat::DPCM_16 => sndfile_sys::SF_FORMAT_DPCM_16,
    SubtypeFormat::VORBIS => sndfile_sys::SF_FORMAT_VORBIS,
    SubtypeFormat::ALAC_16 => sndfile_sys::SF_FORMAT_ALAC_16,
    SubtypeFormat::ALAC_20 => sndfile_sys::SF_FORMAT_ALAC_20,
    SubtypeFormat::ALAC_24 => sndfile_sys::SF_FORMAT_ALAC_24,
    SubtypeFormat::ALAC_32 => sndfile_sys::SF_FORMAT_ALAC_32,
  }
}

pub fn endian_to_flags(endian: Endian) -> c_int {
  match endian {
    Endian::File => sndfile_sys::SF_ENDIAN_FILE,
    Endian::Little => sndfile_sys::SF_ENDIAN_LITTLE,
    Endian::Big => sndfile_sys::SF_ENDIAN_BIG,
    Endian::CPU => sndfile_sys::SF_ENDIAN_CPU,
  }
}

pub fn assembly_format_flags(
  major_format: MajorFormat,
  subtype_format: SubtypeFormat,
  endian: Endian,
) -> c_int {
  let major_format_flag = major_format_to_flags(major_format);
  let subtype_format_flag = subtype_format_to_flags(subtype_format);
  let endian_flag = endian_to_flags(endian);
  major_format_flag | subtype_format_flag | endian_flag
}

/// Get all supported audio container format
pub fn get_supported_major_format_dict() -> &'static HashMap<MajorFormat, MajorInfo> {
  &*MAJOR_FORMAT_LIST
}

/// Get all supported audio encoding format
pub fn get_supported_subtype_format_dict() -> &'static HashMap<SubtypeFormat, SubtypeInfo> {
  &*SUBTYPE_FORMAT_LIST
}

/// This function allows the caller to check if a set of parameters before opening a file in write mode.
pub fn check_format(
  channels: usize,
  samplerate: usize,
  major_format: MajorFormat,
  subtype_format: SubtypeFormat,
  endian: Endian,
) -> bool {
  let info = sndfile_sys::SF_INFO {
    frames: 0,
    samplerate: samplerate as c_int,
    channels: channels as c_int,
    format: assembly_format_flags(major_format, subtype_format, endian),
    sections: 0,
    seekable: 0,
  };
  match unsafe { sndfile_sys::sf_format_check(&info as *const sndfile_sys::SF_INFO) } {
    sndfile_sys::SF_TRUE => true,
    _ => false,
  }
}

/// Returns default audio encoding format for given audio container format
pub fn default_subtype(major_format: MajorFormat) -> Option<SubtypeFormat> {
  match major_format {
    MajorFormat::WAV => Some(SubtypeFormat::PCM_16),
    MajorFormat::AIFF => Some(SubtypeFormat::PCM_16),
    MajorFormat::AU => Some(SubtypeFormat::PCM_16),
    MajorFormat::RAW => None,
    MajorFormat::PAF => Some(SubtypeFormat::PCM_16),
    MajorFormat::SVX => Some(SubtypeFormat::PCM_16),
    MajorFormat::NIST => Some(SubtypeFormat::PCM_16),
    MajorFormat::VOC => Some(SubtypeFormat::PCM_16),
    MajorFormat::IRCAM => Some(SubtypeFormat::PCM_16),
    MajorFormat::W64 => Some(SubtypeFormat::PCM_16),
    MajorFormat::MAT4 => Some(SubtypeFormat::DOUBLE),
    MajorFormat::MAT5 => Some(SubtypeFormat::DOUBLE),
    MajorFormat::PVF => Some(SubtypeFormat::PCM_16),
    MajorFormat::XI => Some(SubtypeFormat::DPCM_16),
    MajorFormat::HTK => Some(SubtypeFormat::PCM_16),
    MajorFormat::SDS => Some(SubtypeFormat::PCM_16),
    MajorFormat::AVR => Some(SubtypeFormat::PCM_16),
    MajorFormat::WAVEX => Some(SubtypeFormat::PCM_16),
    MajorFormat::SD2 => Some(SubtypeFormat::PCM_16),
    MajorFormat::FLAC => Some(SubtypeFormat::PCM_16),
    MajorFormat::CAF => Some(SubtypeFormat::PCM_16),
    MajorFormat::WVE => Some(SubtypeFormat::ALAW),
    MajorFormat::OGG => Some(SubtypeFormat::VORBIS),
    MajorFormat::MPC2K => Some(SubtypeFormat::PCM_16),
    MajorFormat::RF64 => Some(SubtypeFormat::PCM_16),
  }
}
