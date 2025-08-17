use serde::Serialize;
use std::fs::Metadata;

#[derive(Serialize, Debug, PartialEq)]
pub enum Kind {
  File,
  Dir,
  Symlink,
  Other,
}

pub fn detect(node: &Metadata) -> Kind {
  match () {
    _ if node.is_file() => Kind::File,
    _ if node.is_dir() => Kind::Dir,
    _ if node.is_symlink() => Kind::Symlink,
    _ => Kind::Other,
  }
}
