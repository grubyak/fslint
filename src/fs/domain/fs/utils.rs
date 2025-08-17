use log::Level;

use crate::fs::rule::RuleReport;

pub fn parse_mode(text: &str) -> Result<u32, RuleReport> {
  let invalid_mode = || RuleReport {
    messages: vec![format!("invalid mode `{text}`")],
    level: Level::Error,
  };

  if let Ok(mode) = u32::from_str_radix(text.trim_start_matches('0'), 8) {
    return Ok(mode);
  }

  if text.len() == 9 {
    let mut mode = 0;

    for (index, letter) in text.chars().enumerate() {
      if letter == '-' {
        continue;
      }

      if ['r', 'w', 'x'][index % 3] != letter {
        return Err(invalid_mode());
      }

      mode |= 1 << (8 - index);
    }

    return Ok(mode);
  }

  Err(invalid_mode())
}
