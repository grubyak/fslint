pub mod exif;
pub mod id3;

use serde::Serialize;
use std::{collections::HashMap, ffi::OsStr, path::Path};

pub use id3::Id3Metadata;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Value {
  Exif(exif::provider::Metadata),
  Id3(Id3Metadata),
}

pub trait MetadataProvider {
  fn key(&self) -> &'static str;
  fn extensions(&self) -> &'static [&'static str];
  fn collect(&self, path: &Path) -> Option<Value>;

  fn supports(&self, ext: &str) -> bool {
    self
      .extensions()
      .iter()
      .any(|item| item.eq_ignore_ascii_case(ext))
  }
}

pub fn get_providers() -> Vec<Box<dyn MetadataProvider>> {
  vec![
    Box::new(exif::provider::Provider),
    // Box::new(id3::Resolver),
    // Box::new(txt::Resolver),
  ]
}

pub fn collect(path: &Path) -> HashMap<String, Value> {
  let mut map = HashMap::new();

  let file_extension = match path.extension().and_then(OsStr::to_str) {
    Some(extension) => extension.to_lowercase(),
    None => return map,
  };

  for provider in get_providers() {
    if provider.supports(&file_extension) {
      if let Some(metadata) = provider.collect(path) {
        map.insert(provider.key().to_string(), metadata);
      }
    }
  }

  map
}
