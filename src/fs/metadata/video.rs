use crate::fs::metadata::Value;
use std::path::Path;

#[derive(Debug)]
pub struct VideoMetadata {
  pub codec: Option<String>,
  pub duration_seconds: Option<u64>,
}

pub fn resolve(_path: &Path) -> Value {
  Value::Video(VideoMetadata {
    codec: None,
    duration_seconds: None,
  })
}
