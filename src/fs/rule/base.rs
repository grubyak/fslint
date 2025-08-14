use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug, PartialEq, Eq, Default)]
#[schemars(inline)]
#[serde(rename_all = "lowercase")]
pub enum Level {
  Off,
  #[default]
  Warn,
  Error,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug, Default)]
#[serde(default)]
pub struct BaseOptions {
  #[schemars(description = "severity level (default: warn)")]
  pub level: Level,
}
