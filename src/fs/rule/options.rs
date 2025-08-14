use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::fs::{
  config::validate::validate_value_as,
  rule::{BaseOptions, RuleReport},
};

pub fn resolve_rule_options<RuleOptions: DeserializeOwned>(
  options_value: &Value,
) -> Result<(BaseOptions, RuleOptions), RuleReport> {
  let base = validate_value_as::<BaseOptions>(options_value)?;
  let mut reduced = options_value.clone();

  if let Some(options_map) = reduced.as_object_mut() {
    if let Some(base_map) = serde_json::to_value(&base).unwrap().as_object() {
      for base_key in base_map.keys() {
        options_map.remove(base_key);
      }
    }
  }

  Ok((base, validate_value_as::<RuleOptions>(&reduced)?))
}
