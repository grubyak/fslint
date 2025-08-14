use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::fs::rule::RuleReport;

pub fn validate_value_as<Type: DeserializeOwned>(value: &Value) -> Result<Type, RuleReport> {
  let text = serde_json::to_string(value).unwrap();
  let mut deserializer = serde_json::Deserializer::from_str(&text);

  serde_path_to_error::deserialize::<_, Type>(&mut deserializer).map_err(|error| {
    let reason = error
      .inner()
      .to_string()
      .split(" at line ")
      .next()
      .unwrap_or_default()
      .to_string()
      .replace("invalid type: ", "invalid ")
      .replace("unknown field ", "unknown option ");

    let messages = vec![if reason.contains("unknown option") {
      reason
    } else {
      format!("option '{}' → {}", error.path(), reason)
    }];

    RuleReport {
      messages,
      level: log::Level::Error,
    }
  })
}
