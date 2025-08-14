use users::get_group_by_gid;

pub fn lookup(gid: u32) -> String {
  match get_group_by_gid(gid) {
    Some(group) => group.name().to_string_lossy().into_owned(),
    None => gid.to_string(),
  }
}
