use crate::fs::metadata::Value;
use std::path::Path;

#[derive(Debug)]
pub struct Id3Metadata {
  pub title: Option<String>,
  pub artist: Option<String>,
  pub album: Option<String>,
  pub year: Option<u32>,
}

pub fn resolve(path: &Path) -> Value {
  Value::Id3(Id3Metadata {
    title: None,
    artist: None,
    album: None,
    year: None,
  })
}
