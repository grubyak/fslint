mod base;
mod macros;
mod options;
mod rule;

pub use crate::fs::rule::{
  base::{BaseOptions, Level},
  options::resolve_rule_options,
  rule::{prefix_rules, rule_return, Rule, RuleReport},
};
