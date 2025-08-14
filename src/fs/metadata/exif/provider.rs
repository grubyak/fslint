use rexiv2;
use serde::Serialize;
use std::path::Path;

use crate::fs::metadata::{MetadataProvider, Value};

#[derive(Serialize, Debug, Default)]
pub struct Metadata {
  pub longitude: Option<String>,
  pub latitude: Option<String>,
  pub capture_date: Option<String>,
  pub capture_time: Option<String>,
  pub lens: Option<String>,
  pub camera: Option<String>,
}

pub struct Provider;

fn lookup(metadata: &rexiv2::Metadata, tag: &str) -> Option<String> {
  metadata
    .get_tag_string(tag)
    .ok()
    .map(|value| value.trim().to_string())
    .filter(|value| !value.is_empty())
}

impl MetadataProvider for Provider {
  fn key(&self) -> &'static str {
    "exif"
  }

  fn extensions(&self) -> &'static [&'static str] {
    &["jpg", "jpeg", "tiff", "dng", "cr2", "cr3"]
  }

  fn collect(&self, path: &Path) -> Option<Value> {
    let raw = match rexiv2::Metadata::new_from_path(path) {
      Ok(value) => value,
      Err(_) => return None,
    };

    let longitude = lookup(&raw, "Xmp.exif.GPSLongitude");
    let latitude = lookup(&raw, "Xmp.exif.GPSLatitude");
    let lens = lookup(&raw, "Xmp.exifEX.LensModel");

    let (capture_date, capture_time) = raw
      .get_tag_string("Xmp.exif.DateTimeOriginal")
      .ok()
      .and_then(|value| {
        value
          .trim()
          .split_once('T')
          .map(|(date, time)| (Some(date.to_string()), Some(time.to_string())))
      })
      .unwrap_or((None, None));

    let camera = match (
      lookup(&raw, "Xmp.tiff.Make"),
      lookup(&raw, "Xmp.tiff.Model"),
    ) {
      (Some(make), Some(model)) => Some(format!("{make} {model}")),
      (Some(make), None) => Some(make),
      (None, Some(model)) => Some(model),
      _ => None,
    };

    Some(Value::Exif(Metadata {
      longitude,
      latitude,
      capture_date,
      capture_time,
      lens,
      camera,
    }))
  }
}
