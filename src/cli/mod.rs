pub mod render;

use std::process::exit;

use crate::{cli::render::summary, fs::linter, utils::args};

pub fn run(args: &args::Args) {
  if args.schema > 0 {
    match args.schema {
      1 => render::schema::out(),
      _ => render::markdown::out(),
    }

    exit(0);
  }

  let stats = linter::run(args);

  summary::out(stats);
}
