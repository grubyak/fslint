mod fs;
mod utils;

use utils::args::Args;
use utils::logging;
use utils::walk;

fn main() {
  let args = Args::parse_args();

  logging::init(args.verbose);

  log::debug!("args: {:?}", args);
  log::info!("scanning: {:?}", args.root);

  for entry in walk::paths(&args.root) {
    let node = fs::node::Node::populate(&entry);
    println!("{} - {} - {:?}", entry.path().display(), node.owner, node);
  }
}
