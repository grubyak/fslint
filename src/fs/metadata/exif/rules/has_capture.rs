use chrono::{Local, NaiveDate};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{declare_rule, fs::metadata::exif::utils::get_exif};

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(description = "guards presence and validity of capture date and time")]
pub struct Options {
  #[schemars(description = "require capture date")]
  pub date: Option<bool>,

  #[schemars(description = "require capture time")]
  pub time: Option<bool>,

  #[schemars(description = "minimum allowed year for capture date")]
  pub min_year: Option<u16>,

  #[schemars(description = "reject capture dates later than the current date")]
  pub reject_future: Option<bool>,
}

declare_rule!("has-capture-datetime", Options, |options, node, msgs| {
  let exif = match get_exif(node) {
    Ok(metadata) => metadata,
    Err(report) => return Some(report),
  };

  if options.date.unwrap_or(false) && exif.capture_date.is_none() {
    msgs.push("missing capture date".to_string());
  }

  if options.time.unwrap_or(false) && exif.capture_time.is_none() {
    msgs.push("missing capture time".to_string());
  }

  if let Some(date) = exif.capture_date.as_ref() {
    if let Some(min_year) = options.min_year {
      if let Some(year) = date.get(0..4).and_then(|value| value.parse::<u16>().ok()) {
        if year < min_year {
          msgs.push(format!("capture date {date} is below minimum {min_year}"));
        }
      }
    }

    if options.reject_future.unwrap_or(false) {
      if let Ok(capture_date) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        let today = Local::now().date_naive();

        if capture_date > today {
          msgs.push(format!("capture date {capture_date} is in the future"));
        }
      }
    }
  }
});
