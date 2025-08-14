use globset::{Glob, GlobSet, GlobSetBuilder};
use schemars::{schema_for, JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Map, Value};

use crate::fs;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[schemars(deny_unknown_fields)]
pub struct ConfigEntry {
  #[serde(default)]
  #[schemars(
    default,
    description = "glob patterns for files to apply configuration entry"
  )]
  pub include: Vec<String>,

  #[serde(default)]
  #[schemars(
    default,
    description = "glob patterns for files to exclude from configuration entry"
  )]
  pub exclude: Vec<String>,

  #[serde(skip)]
  #[schemars(skip)]
  pub include_set: GlobSet,

  #[serde(skip)]
  #[schemars(skip)]
  pub exclude_set: GlobSet,

  #[schemars(
    schema_with = "rules_schema",
    description = "rules to apply to matching files"
  )]
  pub rules: serde_json::Value,
}

pub type ConfigEntries = Vec<ConfigEntry>;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[schemars(deny_unknown_fields)]
#[schemars(
  title = "fslint configuration file schema",
  description = "see https://crates.io/crates/fslint"
)]
pub struct ConfigFile {
  #[serde(default, rename = "$schema")]
  #[schemars(
    default,
    rename = "$schema",
    description = "json schema uri for tooling and editor integration"
  )]
  pub schema: Option<String>,

  #[serde(default)]
  #[schemars(
    default,
    description = "array of configuration entries with their associated rules"
  )]
  pub entries: ConfigEntries,
}

fn rules_schema(_: &mut SchemaGenerator) -> Schema {
  let mut map = Map::new();
  let base = serde_json::to_value(schema_for!(fs::rule::BaseOptions)).unwrap();

  for (name, rule) in fs::get_rules() {
    let schema = rule.options_schema();
    let mut properties = Map::new();
    let mut required = Vec::new();

    if let Some(value) = base.get("properties").and_then(|value| value.as_object()) {
      properties.extend(value.clone());
    }

    if let Some(value) = base.get("required").and_then(|value| value.as_array()) {
      required.extend(value.clone());
    }

    if let Some(value) = schema.get("properties").and_then(|value| value.as_object()) {
      properties.extend(value.clone());
    }

    if let Some(value) = schema.get("required").and_then(|value| value.as_array()) {
      required.extend(value.clone());
    }

    let mut rule_object = json!({
      "type": "object",
      "properties": properties,
      "required": required,
      "additionalProperties": false
    });

    if let Some(description) = schema.get("description").and_then(|value| value.as_str()) {
      rule_object
        .as_object_mut()
        .unwrap()
        .insert("description".to_string(), json!(description));
    }

    map.insert(name, rule_object);
  }

  let root = json!({
    "type": "object",
    "properties": map,
    "additionalProperties": false
  });

  serde_json::from_value(root).unwrap()
}

pub fn get_config_schema() -> Value {
  let mut schema = to_value(schema_for!(ConfigFile)).unwrap();

  if let Value::Object(ref mut data) = schema {
    data.insert(
      "$id".to_string(),
      Value::String("https://www.schemastore.org/fslint.json".to_string()),
    );
  }

  schema
}

pub fn has_match(config: &[ConfigEntry], path: &std::path::Path) -> bool {
  config.iter().any(|config_entry| config_entry.matches(path))
}

impl ConfigEntry {
  pub fn compile_patterns(&mut self) {
    let mut include_builder = GlobSetBuilder::new();
    let mut exclude_builder = GlobSetBuilder::new();

    for pattern in &self.include {
      if let Ok(glob) = Glob::new(pattern) {
        include_builder.add(glob);
      }
    }

    for pattern in &self.exclude {
      if let Ok(glob) = Glob::new(pattern) {
        exclude_builder.add(glob);
      }
    }

    self.include_set = include_builder.build().unwrap();
    self.exclude_set = exclude_builder.build().unwrap();
  }

  pub fn matches(&self, path: &std::path::Path) -> bool {
    if self.include.is_empty() || !self.include_set.is_match(path) {
      return false;
    }

    !self.exclude_set.is_match(path)
  }
}
