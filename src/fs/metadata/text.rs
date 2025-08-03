use crate::fs::metadata::Value;
use std::path::Path;

#[derive(Debug)]
pub struct TextMetadata {
  pub encoding: Option<String>,
  pub line_count: Option<u64>,
}

pub fn resolve(_path: &Path) -> Value {
  Value::Text(TextMetadata {
    encoding: None,
    line_count: None,
  })
}
