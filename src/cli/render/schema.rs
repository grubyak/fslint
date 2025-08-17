use crate::{fs::config::schema, utils::json::Pretty};

pub fn out() {
  println!("{}", schema::get_config_schema().pretty());
}
