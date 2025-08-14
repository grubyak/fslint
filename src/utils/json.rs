use serde_json::Value;

pub trait Pretty {
  fn pretty(&self) -> String;
}

impl Pretty for Value {
  fn pretty(&self) -> String {
    serde_json::to_string_pretty(self).unwrap()
  }
}
