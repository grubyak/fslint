use colored::Colorize;
use env_logger::fmt::Formatter;
use log::{Level, LevelFilter, Record};
use std::io::Write;

pub fn init(verbose: u8) {
  let level = match verbose {
    0 => LevelFilter::Warn,
    1 => LevelFilter::Info,
    2 => LevelFilter::Debug,
    _ => LevelFilter::Trace,
  };

  env_logger::Builder::new()
    .format(|buf: &mut Formatter, record: &Record| {
      let color = match record.level() {
        Level::Error => "error".bright_red(),
        Level::Warn => "warn".bright_yellow(),
        Level::Info => "info".green(),
        Level::Debug => "debug".bright_purple(),
        Level::Trace => "trace".bright_cyan(),
      };

      if record.target() == "lint" {
        writeln!(buf, "    {:<5} {}", color, record.args())
      } else {
        writeln!(buf, "{} [{}] {}", buf.timestamp(), color, record.args())
      }
    })
    .filter_level(level)
    .init();
}
