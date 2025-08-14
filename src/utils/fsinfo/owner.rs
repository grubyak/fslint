use users::get_user_by_uid;

pub fn lookup(uid: u32) -> String {
  match get_user_by_uid(uid) {
    Some(user) => user.name().to_string_lossy().into_owned(),
    None => uid.to_string(),
  }
}
