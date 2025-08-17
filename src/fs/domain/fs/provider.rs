use crate::fs::domain::MetadataProvider;

pub struct Provider;

impl MetadataProvider for Provider {
  fn key(&self) -> &'static str {
    "fs"
  }
}
