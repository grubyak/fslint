use serde::Serialize;
use std::collections::HashMap;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use walkdir::DirEntry;

use crate::{fs::domain, utils::fsinfo};

#[derive(Serialize, Debug)]
pub struct Node {
  pub entry: PathBuf,
  pub directory: String,
  pub name: String,
  pub kind: fsinfo::kind::Kind,
  pub size: u64,
  pub owner: String,
  pub group: String,
  pub mode: u32,
  pub created: i64,
  pub accessed: i64,
  pub modified: i64,
  pub metadata: HashMap<String, domain::Value>,
}

impl Node {
  pub fn populate(entry: &DirEntry) -> Self {
    let path = entry.path();
    let name = entry.file_name().to_string_lossy().into_owned();

    match entry.metadata() {
      Ok(node) => Node {
        entry: path.to_path_buf(),
        directory: path.parent().unwrap().to_str().unwrap().to_string(),
        name,
        kind: fsinfo::kind::detect(&node),
        size: node.len(),
        owner: fsinfo::owner::lookup(node.uid()),
        group: fsinfo::group::lookup(node.gid()),
        mode: node.mode() & 0o777,
        created: node.created().map(fsinfo::timestamp::to_unix).unwrap_or(0),
        accessed: node.accessed().map(fsinfo::timestamp::to_unix).unwrap_or(0),
        modified: node.modified().map(fsinfo::timestamp::to_unix).unwrap_or(0),
        metadata: domain::collect(path),
      },
      Err(_) => Node {
        entry: path.to_path_buf(),
        directory: "".to_string(),
        name,
        kind: fsinfo::kind::Kind::Other,
        size: 0,
        owner: "unknown".to_string(),
        group: "unknown".to_string(),
        mode: 0,
        created: 0,
        accessed: 0,
        modified: 0,
        metadata: Default::default(),
      },
    }
  }
}
