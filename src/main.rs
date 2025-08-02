mod utils;
use utils::args::Args;

fn main() {
  println!("hello, world");

  let args = Args::parse_args();
  println!("scanning {}", args.root);

  utils::dummy::hey();
}
