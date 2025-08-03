use crate::fs::metadata::{MetadataProvider, Value};
use std::path::Path;

#[derive(Debug)]
pub struct ExifMetadata {
  pub camera_model: Option<String>,
  pub datetime: Option<String>,
}

pub struct Provider;

impl MetadataProvider for Provider {
  fn key(&self) -> &'static str {
    "exif"
  }

  fn extensions(&self) -> &'static [&'static str] {
    &["jpg", "jpeg", "png"]
  }

  fn collect(&self, _path: &Path) -> Value {
    Value::Exif(ExifMetadata {
      camera_model: None,
      datetime: None,
    })
  }
}
