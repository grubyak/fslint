#[macro_export]
macro_rules! declare_rule {
  ($name:expr, $rule_options:ty, |$options:ident, $node:ident, $messages:ident| $body:block) => {
    pub struct Rule;

    impl $crate::fs::rule::Rule for Rule {
      fn name(&self) -> &str {
        $name
      }

      fn options_schema(&self) -> serde_json::Value {
        serde_json::to_value(&schemars::schema_for!($rule_options)).unwrap()
      }

      fn check(
        &self,
        $node: &$crate::fs::node::Node,
        options_value: &serde_json::Value,
      ) -> Option<$crate::fs::rule::RuleReport> {
        let mut $messages = Vec::new();
        let (base, $options) = match $crate::fs::rule::resolve_rule_options::<$rule_options>(options_value) {
          Ok((base, _)) if base.level == $crate::fs::rule::Level::Off => return None,
          Ok((base, options)) => (base, options),
          Err(report) => return Some(report),
        };

        $body
        $crate::fs::rule::rule_return($messages, base.level)
      }
    }
  };
}
