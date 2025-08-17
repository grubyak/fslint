mod cli;
mod fs;
mod utils;

use crate::utils::{args, logging};

fn main() {
  let args = args::parse();

  logging::init(args.verbose);
  cli::run(&args);
}
