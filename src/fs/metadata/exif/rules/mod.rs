mod has_capture;
mod has_coords;

use crate::fs::{
  metadata::exif::provider::Provider,
  rule::{prefix_rules, Rule},
};

pub fn get_rules() -> Vec<(String, Box<dyn Rule>)> {
  let rules: Vec<Box<dyn Rule>> = vec![
    Box::new(has_coords::Rule {}),
    Box::new(has_capture::Rule {}),
  ];

  prefix_rules(rules, &Provider)
}
