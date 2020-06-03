use crate::*;
use tempfile::TempDir;
mod test_issue_1;

#[test]
fn supported_format() {
  let maj_d = get_supported_major_format_dict();
  let sub_d = get_supported_subtype_format_dict();
  let maj_wav = maj_d.get(&MajorFormat::WAV);
  let sub_pcm_16 = sub_d.get(&SubtypeFormat::PCM_16);
  dbg!(maj_d);
  dbg!(sub_d);
  match maj_wav {
    None => panic!(),
    Some(x) => {
      assert_eq!(x.name, "WAV (Microsoft)");
      assert_eq!(x.extension, "wav");
    }
  }
  match sub_pcm_16 {
    None => panic!(),
    Some(x) => {
      assert_eq!(x.name, "Signed 16 bit PCM");
    }
  }

  assert!(check_format(
    3,
    44100,
    MajorFormat::FLAC,
    SubtypeFormat::PCM_24,
    Endian::File
  ));
}

#[test]
fn file_io_ok_0() {
  const DESIRED_BUF: [i16; 34] = [
    -32768, -32768, -28672, -28672, -24576, -24576, -20480, -20480, -16384, -16384, -12288,
    -12288, -8192, -8192, -4096, -4096, 0, 0, 4096, 4096, 8192, 8192, 12288, 12288, 16384, 16384,
    20480, 20480, 24576, 24576, 28672, 28672, 32767, 32767,
  ];
  const TAG_STR: &str = "fxxking test tone";
  let tmp_dir = TempDir::new().unwrap();
  let tmp_path = tmp_dir.as_ref().join("file_io_ok_0.wav");

  {
    let mut snd = OpenOptions::WriteOnly(WriteOptions::new(
      MajorFormat::WAV,
      SubtypeFormat::PCM_24,
      Endian::File,
      8000,
      2,
    ))
    .from_path(&tmp_path)
    .unwrap();
    for _ in 0..4096 {
      snd.write_from_slice(&DESIRED_BUF).unwrap();
    }
    snd.set_tag(TagType::Title, TAG_STR).unwrap();
  }
  {
    let mut snd = OpenOptions::ReadOnly(ReadOptions::Auto)
      .from_path(&tmp_path)
      .unwrap();
    assert!(snd.is_seekable());
    assert_eq!(snd.get_major_format(), MajorFormat::WAV);
    assert_eq!(snd.get_subtype_format(), SubtypeFormat::PCM_24);
    assert_eq!(snd.len().unwrap(), 4096 * 17);
    for _ in 0..2 {
      snd.seek(SeekFrom::Start(0)).unwrap();
      for _ in 0..4096 {
        let mut buf = [0i16; DESIRED_BUF.len()];
        snd.read_to_slice(&mut buf).unwrap();
        assert_eq!(buf[..], DESIRED_BUF[..]);
      }
    }
    let buf: Vec<i16> = snd.read_all_to_vec().unwrap();
    for chunk in buf.chunks(DESIRED_BUF.len()) {
      assert_eq!(chunk[..], DESIRED_BUF[..]);
    }
    assert_eq!(snd.get_tag(TagType::Title), TAG_STR);
  }
  std::fs::remove_file(&tmp_path).unwrap();
}

#[cfg(feature = "ndarray_features")]
#[test]
fn file_io_ok_1() {
  use ndarray::{Array1, Array2, Axis};
  let desired_buf = Array1::<i16>::from_iter(
    [
      -32768, -32768, -28672, -28672, -24576, -24576, -20480, -20480, -16384, -16384, -12288,
      -12288, -8192, -8192, -4096, -4096, 0, 0, 4096, 4096, 8192, 8192, 12288, 12288, 16384,
      16384, 20480, 20480, 24576, 24576, 28672, 28672, 32767, 32767,
    ]
    .iter()
    .map(|x| *x),
  )
  .into_shape((17, 2))
  .unwrap();
  let tmp_dir = TempDir::new().unwrap();
  let tmp_path = tmp_dir.as_ref().join("file_io_ok_1.wav");

  {
    let mut snd = OpenOptions::WriteOnly(WriteOptions::new(
      MajorFormat::WAV,
      SubtypeFormat::PCM_24,
      Endian::File,
      8000,
      2,
    ))
    .from_path(&tmp_path)
    .unwrap();
    for _ in 0..4096 {
      snd.write_from_ndarray(desired_buf.view()).unwrap();
    }
  }
  {
    let mut snd = OpenOptions::ReadOnly(ReadOptions::Auto)
      .from_path(&tmp_path)
      .unwrap();
    assert!(snd.is_seekable());
    assert_eq!(snd.get_major_format(), MajorFormat::WAV);
    assert_eq!(snd.get_subtype_format(), SubtypeFormat::PCM_24);
    assert_eq!(snd.len().unwrap(), 4096 * 17);
    for _ in 0..2 {
      snd.seek(SeekFrom::Start(0)).unwrap();
      for _ in 0..4096 {
        let mut buf: Array2<i16> = Array2::zeros(desired_buf.raw_dim());
        snd.read_to_ndarray(buf.view_mut()).unwrap();
        assert_eq!(buf, desired_buf);
      }
    }
    let buf: Array2<i16> = snd.read_all_to_ndarray().unwrap();
    for chunk in buf.axis_chunks_iter(Axis(0), desired_buf.shape()[0]) {
      assert_eq!(chunk, desired_buf);
    }
  }
  std::fs::remove_file(&tmp_path).unwrap();
}
