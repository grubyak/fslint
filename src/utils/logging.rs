use std::env;

pub fn init(verbose: u8) {
  let log_level = match verbose {
    0 => "warn",
    1 => "info",
    2 => "debug",
    _ => "trace",
  };

  env::set_var("RUST_LOG", log_level);
  env_logger::init();
}
