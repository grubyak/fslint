pub mod config;
pub mod domain;
pub mod linter;
pub mod node;
pub mod rule;

use serde_json::Value;
use std::collections::BTreeMap;

use crate::fs::{
  domain::{exif, fs},
  rule::Rule,
};

type RuleProvider = fn() -> Vec<(String, Box<dyn Rule>)>;

pub fn get_rules() -> Vec<(String, Box<dyn Rule>)> {
  let providers: Vec<RuleProvider> = vec![
    exif::rules::get_rules,
    fs::rules::get_rules,
    // id3::get_rules,
  ];

  providers
    .into_iter()
    .flat_map(|provider| provider())
    .collect()
}

pub fn describe_rules() -> BTreeMap<String, BTreeMap<String, Value>> {
  get_rules()
    .into_iter()
    .map(|(rule_name, rule)| (rule_name, rule.expected_options()))
    .collect()
}
