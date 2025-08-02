use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
  #[arg(short, long, default_value = ".")]
  pub root: std::path::PathBuf,
}

impl Args {
  pub fn parse_args() -> Self {
    Args::parse()
  }
}
