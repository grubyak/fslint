mod fs;
mod utils;

use serde_json::to_string_pretty;
use std::{env::current_dir, process::exit, time::Instant};

use crate::fs::{config::loader, config::schema, node::Node};
use crate::utils::{args, json::Pretty, logging, walk};

fn main() {
  let args = args::parse();

  logging::init(args.verbose);
  log::debug!("args: {args:?}");

  if args.schema {
    println!("{}", schema::get_config_schema().pretty());
    exit(0);
  }

  let directory = if args.path.as_os_str().is_empty() {
    current_dir().unwrap()
  } else {
    args.path.clone()
  };

  let config = match loader::read_config(directory.to_str().unwrap()) {
    cfg if cfg.is_empty() => {
      log::info!("empty config - quitting");
      exit(0);
    }
    cfg => cfg,
  };

  let registered_rules = fs::get_rules();
  let mut total_errors: usize = 0;
  let mut total_warnings: usize = 0;
  let mut total_inspected: usize = 0;
  let mut total_skipped: usize = 0;
  let start_time = Instant::now();

  log::info!("scanning directory: {:?}", args.path);
  log::debug!(
    "available rules:\n{}",
    to_string_pretty(&fs::describe_rules()).unwrap()
  );

  for entry in walk::paths(&args.path) {
    if !schema::has_match(&config, entry.path()) {
      total_skipped += 1;

      log::info!(
        "skipping: {} in {} (no matching config)",
        entry.file_name().to_string_lossy(),
        entry
          .path()
          .parent()
          .map(|path| path.to_string_lossy())
          .unwrap_or_default()
      );

      continue;
    }

    let node = Node::populate(&entry);

    total_inspected += 1;
    log::info!("inspecting: {} in {}", node.name, node.directory);
    log::debug!("node:\n{}", to_string_pretty(&node).unwrap());

    for config_entry in &config {
      let matches = config_entry.matches(entry.path());

      log::debug!(
        "{}:\n{}",
        if matches {
          "matches → checking"
        } else {
          "no match → skipping"
        },
        to_string_pretty(&config_entry).unwrap()
      );

      if !matches {
        continue;
      }

      let mut findings: Vec<(log::Level, String)> = Vec::new();

      if let Some(config_entry_rules) = config_entry.rules.as_object() {
        for (rule_name, rule_options) in config_entry_rules {
          if let Some((_, rule)) = registered_rules.iter().find(|(name, _)| name == rule_name) {
            if let Some(report) = rule.check(&node, rule_options) {
              findings.push((report.level, format!("{rule_name}: {report}")));
            }
          }
        }
      }

      if !findings.is_empty() {
        println!("{}", node.entry.to_str().unwrap());

        for (level, message) in findings {
          log::log!(target: "lint", level, "{message}");

          match level {
            log::Level::Error => total_errors += 1,
            log::Level::Warn => total_warnings += 1,
            _ => {}
          }
        }

        println!();
      }
    }
  }

  let total_problems = total_errors + total_warnings;
  let elapsed = start_time.elapsed();
  let summary = format!(
    "inspected {total_inspected} file{}, skipped {total_skipped} [{elapsed:.2?}]",
    if total_inspected == 1 { "" } else { "s" },
  );

  if total_problems > 0 {
    println!(
        "✖ {total_problems} problem{} ({total_errors} error{}, {total_warnings} warning{}) -- {summary}",
        if total_problems == 1 { "" } else { "s" },
        if total_errors == 1 { "" } else { "s" },
        if total_warnings == 1 { "" } else { "s" },
    );
  }

  log::info!("{summary}");
}
