use serde_json::to_string_pretty;
use std::{
  env::current_dir,
  path::{Path, PathBuf},
  process::exit,
  time::{Duration, Instant},
};
use walkdir::DirEntry;

pub struct LinterStats {
  pub total_errors: usize,
  pub total_warnings: usize,
  pub total_inspected: usize,
  pub total_skipped: usize,
  pub elapsed: Duration,
}

use crate::{
  fs::{
    self,
    config::{
      loader,
      schema::{self, ConfigEntry},
    },
    node::Node,
    rule::Rule,
  },
  utils::{args, walk},
};

fn resolve_directory(args: &args::Args) -> PathBuf {
  if args.path.as_os_str().is_empty() {
    current_dir().unwrap()
  } else {
    args.path.clone()
  }
}

fn load_config_or_quit(directory: &Path) -> Vec<ConfigEntry> {
  let config = loader::read_config(directory.to_str().unwrap());

  if config.is_empty() {
    log::info!("empty config - quitting");
    exit(0);
  }

  config
}

fn should_skip(entry: &DirEntry, config: &[ConfigEntry]) -> bool {
  let no_match = !schema::has_match(config, entry.path());

  if no_match {
    log::trace!(
      "skipping: {} in {} (no matching config)",
      entry.file_name().to_string_lossy(),
      entry
        .path()
        .parent()
        .map(|path| path.to_string_lossy())
        .unwrap_or_default()
    );
  }

  no_match
}

fn inspect_for(
  config_entry: &ConfigEntry,
  node: &Node,
  registered_rules: &[(String, Box<dyn Rule>)],
) -> Vec<(log::Level, String)> {
  let mut findings = Vec::new();

  if let Some(config_entry_rules) = config_entry.rules.as_object() {
    for (rule_name, rule_options) in config_entry_rules {
      if let Some((_, rule)) = registered_rules.iter().find(|(name, _)| name == rule_name) {
        if let Some(report) = rule.check(node, rule_options) {
          findings.push((report.level, format!("{rule_name}: {report}")));
        }
      }
    }
  }

  findings
}

fn report_findings(node: &Node, findings: &[(log::Level, String)]) -> (usize, usize) {
  let mut errors = 0;
  let mut warnings = 0;

  println!("{}", node.entry.to_str().unwrap());

  for (level, message) in findings {
    log::log!(target: "lint", *level, "{message}");
    match level {
      log::Level::Error => errors += 1,
      log::Level::Warn => warnings += 1,
      _ => {}
    }
  }

  println!();
  (errors, warnings)
}

pub fn run(args: &args::Args) -> LinterStats {
  let directory = resolve_directory(args);
  let config = load_config_or_quit(&directory);
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
    if should_skip(&entry, &config) {
      total_skipped += 1;
      continue;
    }

    let node = Node::populate(&entry);

    total_inspected += 1;
    log::info!("inspecting: {} in {}", node.name, node.directory);
    log::debug!("node:\n{}", to_string_pretty(&node).unwrap());

    for config_entry in &config {
      let matches = config_entry.matches(entry.path());
      let status = ["no match → skipping", "matches → checking"][matches as usize];

      log::debug!("{status}:\n{}", to_string_pretty(&config_entry).unwrap());

      if !matches {
        continue;
      }

      let findings = inspect_for(config_entry, &node, &registered_rules);

      if !findings.is_empty() {
        let (errors, warnings) = report_findings(&node, &findings);

        total_errors += errors;
        total_warnings += warnings;
      }
    }
  }

  LinterStats {
    total_errors,
    total_warnings,
    total_inspected,
    total_skipped,
    elapsed: start_time.elapsed(),
  }
}
