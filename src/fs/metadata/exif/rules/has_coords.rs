use schemars::JsonSchema;
use serde::Deserialize;

use crate::{declare_rule, fs::metadata::exif::utils::get_exif};

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(description = "guards presence of gps coordinates")]
pub struct Options {
  #[schemars(description = "require gps latitude")]
  pub latitude: Option<bool>,

  #[schemars(description = "require gps longitude")]
  pub longitude: Option<bool>,
}

declare_rule!("has-coords", Options, |options, node, msgs| {
  let exif = match get_exif(node) {
    Ok(metadata) => metadata,
    Err(report) => return Some(report),
  };

  if options.latitude.unwrap_or(false) && exif.latitude.is_none() {
    msgs.push("missing latitude".to_string());
  }

  if options.longitude.unwrap_or(false) && exif.longitude.is_none() {
    msgs.push("missing longitude".to_string());
  }
});
