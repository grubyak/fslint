use crate::fs::linter::LinterStats;

pub fn out(stats: LinterStats) {
  let total_problems = stats.total_errors + stats.total_warnings;
  let summary = format!(
    "inspected {} file{}, skipped {} [{:.2?}]",
    stats.total_inspected,
    if stats.total_inspected == 1 { "" } else { "s" },
    stats.total_skipped,
    stats.elapsed,
  );

  if total_problems > 0 {
    println!(
      "✖ {total_problems} problem{} ({} error{}, {} warning{}) -- {summary}",
      if total_problems == 1 { "" } else { "s" },
      stats.total_errors,
      if stats.total_errors == 1 { "" } else { "s" },
      stats.total_warnings,
      if stats.total_warnings == 1 { "" } else { "s" },
    );
  }

  log::info!("{summary}");
}
