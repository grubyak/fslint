use std::fs::read_link;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::{declare_rule, utils::fsinfo::kind::Kind};

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(description = "guards that symbolic links resolve to existing targets")]
pub struct Options {}

declare_rule!("broken-symlink", Options, |_options, node, msgs| {
  if node.kind == Kind::Symlink {
    match read_link(&node.entry) {
      Ok(target) if !target.exists() => {
        msgs.push(format!("target `{}` does not exist", target.display()));
      }
      Ok(_) => {}
      Err(_) => {
        msgs.push("unable to resolve symlink".to_string());
      }
    }
  }
});
