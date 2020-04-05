use super::SndFileIO;
use ndarray::{Array2, ArrayView2, ArrayViewMut2};
use std::io::SeekFrom;

/// Do I/O operation on 2D ndarray.
///
/// The array shape must be (n_frames, n_channels).
pub trait SndFileNDArrayIO<T> {
  fn read_to_ndarray(&mut self, dst: ArrayViewMut2<T>) -> Result<usize, ()>;
  fn write_from_ndarray(&mut self, src: ArrayView2<T>) -> Result<usize, ()>;
  fn read_all_to_ndarray(&mut self) -> Result<Array2<T>, ()>;
}

impl SndFileNDArrayIO<i16> for super::SndFile {
  fn read_to_ndarray(&mut self, mut dst: ArrayViewMut2<i16>) -> Result<usize, ()> {
    assert_eq!(dst.ndim(), 2);
    assert_eq!(dst.shape()[1], self.get_channels());
    match dst.as_slice_mut() {
      Some(s) => self.read_to_slice(s),
      None => self.read_to_iter(dst.iter_mut()),
    }
  }

  fn write_from_ndarray(&mut self, src: ArrayView2<i16>) -> Result<usize, ()> {
    assert_eq!(src.ndim(), 2);
    assert_eq!(src.shape()[1], self.get_channels());
    match src.as_slice() {
      Some(s) => self.write_from_slice(s),
      None => self.write_from_iter(src.iter().map(|x| *x)),
    }
  }

  fn read_all_to_ndarray(&mut self) -> Result<Array2<i16>, ()> {
    let mut arr = Array2::<_>::zeros((self.len()? as usize, self.get_channels()));
    self.seek(SeekFrom::Start(0))?;
    self.read_to_ndarray(arr.view_mut()).map(|_| arr)
  }
}

impl SndFileNDArrayIO<i32> for super::SndFile {
  fn read_to_ndarray(&mut self, mut dst: ArrayViewMut2<i32>) -> Result<usize, ()> {
    assert_eq!(dst.ndim(), 2);
    assert_eq!(dst.shape()[1], self.get_channels());
    match dst.as_slice_mut() {
      Some(s) => self.read_to_slice(s),
      None => self.read_to_iter(dst.iter_mut()),
    }
  }

  fn write_from_ndarray(&mut self, src: ArrayView2<i32>) -> Result<usize, ()> {
    assert_eq!(src.ndim(), 2);
    assert_eq!(src.shape()[1], self.get_channels());
    match src.as_slice() {
      Some(s) => self.write_from_slice(s),
      None => self.write_from_iter(src.iter().map(|x| *x)),
    }
  }

  fn read_all_to_ndarray(&mut self) -> Result<Array2<i32>, ()> {
    let mut arr = Array2::<_>::zeros((self.len()? as usize, self.get_channels()));
    self.seek(SeekFrom::Start(0))?;
    self.read_to_ndarray(arr.view_mut()).map(|_| arr)
  }
}

impl SndFileNDArrayIO<f32> for super::SndFile {
  fn read_to_ndarray(&mut self, mut dst: ArrayViewMut2<f32>) -> Result<usize, ()> {
    assert_eq!(dst.ndim(), 2);
    assert_eq!(dst.shape()[1], self.get_channels());
    match dst.as_slice_mut() {
      Some(s) => self.read_to_slice(s),
      None => self.read_to_iter(dst.iter_mut()),
    }
  }

  fn write_from_ndarray(&mut self, src: ArrayView2<f32>) -> Result<usize, ()> {
    assert_eq!(src.ndim(), 2);
    assert_eq!(src.shape()[1], self.get_channels());
    match src.as_slice() {
      Some(s) => self.write_from_slice(s),
      None => self.write_from_iter(src.iter().map(|x| *x)),
    }
  }

  fn read_all_to_ndarray(&mut self) -> Result<Array2<f32>, ()> {
    let mut arr = Array2::<_>::zeros((self.len()? as usize, self.get_channels()));
    self.seek(SeekFrom::Start(0))?;
    self.read_to_ndarray(arr.view_mut()).map(|_| arr)
  }
}

impl SndFileNDArrayIO<f64> for super::SndFile {
  fn read_to_ndarray(&mut self, mut dst: ArrayViewMut2<f64>) -> Result<usize, ()> {
    assert_eq!(dst.ndim(), 2);
    assert_eq!(dst.shape()[1], self.get_channels());
    match dst.as_slice_mut() {
      Some(s) => self.read_to_slice(s),
      None => self.read_to_iter(dst.iter_mut()),
    }
  }

  fn write_from_ndarray(&mut self, src: ArrayView2<f64>) -> Result<usize, ()> {
    assert_eq!(src.ndim(), 2);
    assert_eq!(src.shape()[1], self.get_channels());
    match src.as_slice() {
      Some(s) => self.write_from_slice(s),
      None => self.write_from_iter(src.iter().map(|x| *x)),
    }
  }

  fn read_all_to_ndarray(&mut self) -> Result<Array2<f64>, ()> {
    let mut arr = Array2::<_>::zeros((self.len()? as usize, self.get_channels()));
    self.seek(SeekFrom::Start(0))?;
    self.read_to_ndarray(arr.view_mut()).map(|_| arr)
  }
}
