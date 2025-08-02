mod utils;

use utils::args::Args;
use utils::walk::walk_paths;

fn main() {
  let args = Args::parse_args();
  for entry in walk_paths(&args.root) {
    println!("{}", entry.path().display());
  }
}
