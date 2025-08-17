use clap::Parser;
use std::{env, path};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
  #[arg(short, long, default_value = ".")]
  pub path: path::PathBuf,

  #[arg(short, long, action = clap::ArgAction::Count)]
  pub verbose: u8,

  #[arg(short, long, action = clap::ArgAction::Count)]
  pub schema: u8,
}

pub fn parse() -> Args {
  let mut args = Args::parse();

  if let Ok(abs) = env::current_dir().and_then(|cwd| cwd.join(&args.path).canonicalize()) {
    args.path = abs;
  }

  log::debug!("args: {args:?}");
  args
}
