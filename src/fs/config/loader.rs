use jsonschema::validator_for;
use serde_json::Value;
use std::{fs, io, path::Path};

use crate::fs::config::schema::{get_config_schema, ConfigEntries, ConfigFile};

const CONFIG_FILENAME: &str = "fslint.json";

fn validate_config(value: &Value) -> Result<(), String> {
  let mut schema = get_config_schema();

  if let Some(properties) = schema
    .pointer_mut("/$defs/ConfigEntry/properties")
    .and_then(|value| value.as_object_mut())
  {
    properties.insert(
      "rules".to_string(),
      serde_json::json!({
        "type": "object",
        "additionalProperties": true
      }),
    );
  }

  let validator = validator_for(&schema).map_err(|error| format!("invalid schema: {error}"))?;

  match validator.validate(value) {
    Ok(_) => Ok(()),
    Err(error) => Err(error.to_string()),
  }
}

fn evaluate_config(directory: &str) -> Result<ConfigEntries, String> {
  let config_path = Path::new(directory).join(CONFIG_FILENAME);

  let text = match fs::read_to_string(&config_path) {
    Ok(text) => text,
    Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(ConfigEntries::default()),
    Err(error) => return Err(error.to_string()),
  };

  let value: Value = serde_json::from_str(&text).map_err(|error| error.to_string())?;

  validate_config(&value)?;

  serde_json::from_value::<ConfigFile>(value)
    .map(|config| config.entries)
    .map_err(|error| error.to_string())
}

pub fn read_config(directory: &str) -> ConfigEntries {
  log::info!("reading config: {directory}/{CONFIG_FILENAME}");

  match evaluate_config(directory) {
    Ok(mut config) => {
      log::debug!(
        "applying config:\n{}",
        serde_json::to_string_pretty(&config).unwrap()
      );

      for entry in &mut config {
        entry.compile_patterns();
      }

      config
    }
    Err(error) => {
      eprintln!("invalid config: {error}");
      std::process::exit(1);
    }
  }
}
