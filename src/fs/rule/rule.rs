use serde_json::Value;
use std::{collections::BTreeMap, fmt};

use crate::fs::{metadata::MetadataProvider, node, rule};

#[derive(Debug)]
pub struct RuleReport {
  pub messages: Vec<String>,
  pub level: log::Level,
}

impl fmt::Display for RuleReport {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "{}", self.messages.join(", "))
  }
}

pub trait Rule {
  fn name(&self) -> &str;
  fn options_schema(&self) -> Value;
  fn check(&self, node: &node::Node, options: &Value) -> Option<RuleReport>;

  fn expected_options(&self) -> BTreeMap<String, Value> {
    self
      .options_schema()
      .get("properties")
      .and_then(Value::as_object)
      .cloned()
      .unwrap_or_default()
      .into_iter()
      .map(|(name, options)| (name, options.get("type").cloned().unwrap_or(Value::Null)))
      .collect()
  }
}

pub fn prefix_rules(
  rules: Vec<Box<dyn Rule>>,
  metadata_provider: &impl MetadataProvider,
) -> Vec<(String, Box<dyn Rule>)> {
  rules
    .into_iter()
    .map(|rule| (format!("{}-{}", metadata_provider.key(), rule.name()), rule))
    .collect()
}

pub fn rule_return(messages: Vec<String>, rule_level: rule::Level) -> Option<RuleReport> {
  if messages.is_empty() {
    return None;
  }

  let level = match rule_level {
    rule::Level::Error => log::Level::Error,
    rule::Level::Warn => log::Level::Warn,
    _ => return None,
  };

  Some(RuleReport { messages, level })
}
