use schemars::JsonSchema;
use serde::Deserialize;

use crate::{declare_rule, fs::domain::fs::utils::parse_mode};

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(description = "guards file permission mode, owner, and group")]
pub struct Options {
  #[schemars(description = "enforce mode (octal or symbolic, e.g. \"0644'\" or \"rw-r--r--\")")]
  pub mode: Option<String>,

  #[schemars(description = "enforce specific owner")]
  pub owner: Option<String>,

  #[schemars(description = "enforce specific group")]
  pub group: Option<String>,
}

declare_rule!("permission", Options, |options, node, msgs| {
  if options.mode.is_some() {
    match parse_mode(&options.mode.unwrap()) {
      Ok(mode) if node.mode != mode => {
        msgs.push(format!(
          "mode must be `0{mode:o}` instead `0{:o}`",
          node.mode
        ));
      }
      Ok(_) => {}
      Err(report) => return Some(report),
    }
  }

  if let Some(owner) = &options.owner {
    if &node.owner != owner {
      msgs.push(format!("owner must be `{owner}` instead `{}`", node.owner));
    }
  }

  if let Some(group) = &options.group {
    if &node.group != group {
      msgs.push(format!("group must be `{group}` instead `{}`", node.group));
    }
  }
});
