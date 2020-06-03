use crate::*;
use tempfile::TempDir;

#[test]
fn issue_1_1ch() {
  // ch = 1, t = 50ms, sr = 2000Hz, tone = sin 880Hz
  static DATA: &'static [u8] = b"RIFF\x88\x00\x00\x00WAVEfmt \x10\x00\x00\x00\x01\x00\x01\x00\xd0\x07\x00\x00\xd0\x07\x00\x00\x01\x00\x08\x00datad\x00\x00\x00\x80\xa59\xdc\x19\xe11\xb1sf\xbc)\xe4\x1b\xd6C\x99\x8cN\xce\x1e\xe6#\xc6Z\x80\xa59\xdc\x19\xe11\xb1sf\xbc)\xe4\x1b\xd6C\x99\x8cN\xce\x1e\xe6#\xc6Z\x7f\xa59\xdc\x19\xe11\xb1sf\xbc)\xe4\x1b\xd6C\x99\x8cN\xce\x1e\xe6#\xc6Z\x7f\xa59\xdc\x19\xe11\xb1sf\xbc)\xe4\x1b\xd6C\x99\x8cN\xce\x1e\xe6#\xc6Z";
  let tmp_dir = TempDir::new().unwrap();
  let tmp_path = tmp_dir.as_ref().join("issue_1_1ch.wav");
  std::fs::write(&tmp_path, DATA).unwrap();

  let mut snd = OpenOptions::ReadOnly(ReadOptions::Auto).from_path(&tmp_path).unwrap();
  assert!(snd.is_seekable());
  assert_eq!(snd.get_major_format(), MajorFormat::WAV);
  assert_eq!(snd.get_subtype_format(), SubtypeFormat::PCM_U8);
  assert_eq!(snd.get_channels(), 1);
  assert_eq!(snd.get_samplerate(), 2000);
  assert_eq!(snd.len().unwrap(), 100);
  let buf: Vec<i16> = snd.read_all_to_vec().unwrap();
  assert_eq!(buf.len(), 100);
}

#[test]
fn issue_1_2ch() {
  // ch = 2, t = 25ms, sr = 2000Hz, tone = sin 880Hz
  static DATA: &'static [u8] = b"RIFF\x88\x00\x00\x00WAVEfmt \x10\x00\x00\x00\x01\x00\x02\x00\xd0\x07\x00\x00\xa0\x0f\x00\x00\x02\x00\x08\x00datad\x00\x00\x00\x80\x80\xa5\xa599\xdc\xdc\x19\x19\xe1\xe111\xb1\xb1ssff\xbc\xbc))\xe4\xe4\x1b\x1b\xd6\xd6CC\x99\x99\x8c\x8cNN\xce\xce\x1e\x1e\xe6\xe6##\xc6\xc6ZZ\x7f\x80\xa5\xa599\xdc\xdc\x19\x19\xe1\xe111\xb1\xb1ssff\xbc\xbc))\xe4\xe4\x1b\x1b\xd6\xd6CC\x99\x99\x8c\x8cNN\xce\xce\x1e\x1e\xe6\xe6##\xc6\xc6ZZ";
  let tmp_dir = TempDir::new().unwrap();
  let tmp_path = tmp_dir.as_ref().join("issue_1_1ch.wav");
  std::fs::write(&tmp_path, DATA).unwrap();

  let mut snd = OpenOptions::ReadOnly(ReadOptions::Auto).from_path(&tmp_path).unwrap();
  assert!(snd.is_seekable());
  assert_eq!(snd.get_major_format(), MajorFormat::WAV);
  assert_eq!(snd.get_subtype_format(), SubtypeFormat::PCM_U8);
  assert_eq!(snd.get_channels(), 2);
  assert_eq!(snd.get_samplerate(), 2000);
  assert_eq!(snd.len().unwrap(), 50);
  let buf: Vec<i16> = snd.read_all_to_vec().unwrap();
  assert_eq!(buf.len(), 100);
}
