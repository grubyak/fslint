use std::path::Path;
use walkdir::WalkDir;

pub fn walk_paths(root: &Path) -> impl Iterator<Item = walkdir::DirEntry> {
  WalkDir::new(root).into_iter().filter_map(Result::ok)
}
