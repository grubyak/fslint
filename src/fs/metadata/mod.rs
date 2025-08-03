pub mod exif;
pub mod id3;
pub mod text;
pub mod video;

use std::{collections::HashMap, ffi::OsStr, path::Path};

pub use exif::ExifMetadata;
pub use id3::Id3Metadata;
pub use text::TextMetadata;
pub use video::VideoMetadata;

#[derive(Debug)]
pub enum Value {
  Exif(ExifMetadata),
  Id3(Id3Metadata),
  Text(TextMetadata),
  Video(VideoMetadata),
}

pub trait MetadataProvider {
  fn key(&self) -> &'static str;
  fn extensions(&self) -> &'static [&'static str];
  fn collect(&self, path: &Path) -> Value;

  // fn rules(&self) -> Option<&'static [Rule]> {
  //   None
  // }

  fn supports(&self, ext: &str) -> bool {
    self
      .extensions()
      .iter()
      .any(|item| item.eq_ignore_ascii_case(ext))
  }
}

pub fn all_providers() -> Vec<Box<dyn MetadataProvider>> {
  vec![
    Box::new(exif::Provider),
    // Box::new(id3::Resolver),
    // Box::new(txt::Resolver),
  ]
}

pub fn collect(path: &Path) -> HashMap<String, Value> {
  let mut map = HashMap::new();

  let ext = match path.extension().and_then(OsStr::to_str) {
    Some(ext) => ext.to_lowercase(),
    None => return map,
  };

  for provider in all_providers() {
    if provider.supports(&ext) {
      map.insert(provider.key().to_string(), provider.collect(path));
    }
  }

  map
}
