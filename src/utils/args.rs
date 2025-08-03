use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
  #[arg(short, long, default_value = ".")]
  pub root: std::path::PathBuf,

  #[arg(short, long, action = clap::ArgAction::Count)]
  pub verbose: u8,
}

impl Args {
  pub fn parse_args() -> Self {
    let mut args = Self::parse();

    if let Ok(abs) = env::current_dir().and_then(|cwd| cwd.join(&args.root).canonicalize()) {
      args.root = abs;
    }

    args
  }
}
