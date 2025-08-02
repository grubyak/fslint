use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "fslint")]
#[command(version, about = "lint your filesystem", long_about = None)]
pub struct Args {
  #[arg(short, long, default_value = ".")]
  pub root: String,
}

impl Args {
  pub fn parse_args() -> Self {
    Self::parse()
  }
}
