mod broken_symlink;
mod permission;

use crate::fs::{
  domain::fs::provider::Provider,
  rule::{prefix_rules, Rule},
};

pub fn get_rules() -> Vec<(String, Box<dyn Rule>)> {
  let rules: Vec<Box<dyn Rule>> = vec![
    Box::new(broken_symlink::Rule {}),
    Box::new(permission::Rule {}),
  ];

  prefix_rules(rules, &Provider)
}
