use crate::*;
use tempfile::TempDir;

#[test]
fn issue_3_no_tags() {
  // ch = 1, t = 50ms, sr = 2000Hz, tone = sin 880Hz
  static DATA: &'static [u8] = b"RIFF\x88\x00\x00\x00WAVEfmt \x10\x00\x00\x00\x01\x00\x01\x00\xd0\x07\x00\x00\xd0\x07\x00\x00\x01\x00\x08\x00datad\x00\x00\x00\x80\xa59\xdc\x19\xe11\xb1sf\xbc)\xe4\x1b\xd6C\x99\x8cN\xce\x1e\xe6#\xc6Z\x80\xa59\xdc\x19\xe11\xb1sf\xbc)\xe4\x1b\xd6C\x99\x8cN\xce\x1e\xe6#\xc6Z\x7f\xa59\xdc\x19\xe11\xb1sf\xbc)\xe4\x1b\xd6C\x99\x8cN\xce\x1e\xe6#\xc6Z\x7f\xa59\xdc\x19\xe11\xb1sf\xbc)\xe4\x1b\xd6C\x99\x8cN\xce\x1e\xe6#\xc6Z";
  let tmp_dir = TempDir::new().unwrap();
  let tmp_path = tmp_dir.as_ref().join("issue_3_no_tags.wav");
  std::fs::write(&tmp_path, DATA).unwrap();

  let mut snd = OpenOptions::ReadOnly(ReadOptions::Auto).from_path(&tmp_path).unwrap();
  
  // Empty header, should not have any tag set
  assert_eq!(snd.get_tag(TagType::Title), None);
  assert_eq!(snd.get_tag(TagType::Copyright), None);
  assert_eq!(snd.get_tag(TagType::Software), None);
  assert_eq!(snd.get_tag(TagType::Artist), None);
  assert_eq!(snd.get_tag(TagType::Comment), None);
  assert_eq!(snd.get_tag(TagType::Date), None);
  assert_eq!(snd.get_tag(TagType::Album), None);
  assert_eq!(snd.get_tag(TagType::License), None);
  assert_eq!(snd.get_tag(TagType::Tracknumber), None);
  assert_eq!(snd.get_tag(TagType::Genre), None);
}

#[test]
fn issue_3_some_tags() {
  // Tags to set
  const TAG_TITLE_STR: &str = "some_title";
  const TAG_COPYRIGHT: &str = "dobby_is_free";
  const TAG_COMMENT: &str = "no comment";

  // Empty data vec
  const DEFAULT_BUF: [i16; 256] = [0i16; 256];

  let tmp_dir = TempDir::new().unwrap();
  let tmp_path = tmp_dir.as_ref().join("issue_3_some_tags.wav");

  // Write the file
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
    for _ in 0..256 {
      snd.write_from_slice(&DEFAULT_BUF).unwrap();
    }
    snd.set_tag(TagType::Title, TAG_TITLE_STR).unwrap();
    snd.set_tag(TagType::Copyright, TAG_COPYRIGHT).unwrap();
    snd.set_tag(TagType::Comment, TAG_COMMENT).unwrap();
  }

  // Check the file
  {
    let mut snd = OpenOptions::ReadOnly(ReadOptions::Auto)
      .from_path(&tmp_path)
      .unwrap();
    
    // Check the tags has been set
    assert_eq!(snd.get_tag(TagType::Title).unwrap(), TAG_TITLE_STR);
    assert_eq!(snd.get_tag(TagType::Copyright).unwrap(), TAG_COPYRIGHT);
    assert_eq!(snd.get_tag(TagType::Comment).unwrap(), TAG_COMMENT);

    // Check the missing tags returns None
    assert_eq!(snd.get_tag(TagType::Software), None);
    assert_eq!(snd.get_tag(TagType::Artist), None);
    assert_eq!(snd.get_tag(TagType::Date), None);
    assert_eq!(snd.get_tag(TagType::Album), None);
    assert_eq!(snd.get_tag(TagType::License), None);
    assert_eq!(snd.get_tag(TagType::Tracknumber), None);
    assert_eq!(snd.get_tag(TagType::Genre), None);
  }
  std::fs::remove_file(&tmp_path).unwrap();
}