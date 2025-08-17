use serde_json::Value;

use crate::fs::config::schema;

pub fn out() {
  let schema = schema::get_config_schema();
  let all_rules = schema
    .pointer("/$defs/ConfigEntry/properties/rules/properties")
    .and_then(Value::as_object)
    .unwrap();

  println!("# rules");

  for (rule_name, details) in all_rules {
    let description = details
      .get("description")
      .and_then(Value::as_str)
      .unwrap_or_default();

    println!("\n### {rule_name}\n\n{description}\n");

    if let Some(options) = details.get("properties").and_then(Value::as_object) {
      if options.is_empty() {
        continue;
      }

      let required = details.get("required").and_then(Value::as_array);
      println!("| option | type | required | description |");
      println!("|--------|------|----------|-------------|");

      for (name, properties) in options {
        let types = match properties.get("type") {
          Some(Value::String(text)) => vec![text.to_string()],
          Some(Value::Array(array)) => array
            .iter()
            .filter_map(Value::as_str)
            .map(String::from)
            .collect(),
          _ => Vec::new(),
        };

        let enums = properties
          .get("enum")
          .and_then(Value::as_array)
          .map(|array| {
            array
              .iter()
              .filter_map(Value::as_str)
              .map(|text| format!("\"{text}\""))
              .collect::<Vec<_>>()
          });

        let values = enums.unwrap_or(types).join(" \\| ");

        let is_required = ["no", "yes"][required
          .map(|array| array.iter().any(|value| value == name))
          .unwrap_or(false) as usize];

        let description = properties
          .get("description")
          .and_then(Value::as_str)
          .unwrap_or_default();

        println!("| `{name}` | `{values}` | `{is_required}` | {description} |");
      }
    }
  }
}
