use std::time::{SystemTime, UNIX_EPOCH};

pub fn to_unix(system_time: SystemTime) -> i64 {
  match system_time.duration_since(UNIX_EPOCH) {
    Ok(duration) => (duration.as_secs() * 1000 + u64::from(duration.subsec_millis())) as i64,
    Err(_) => 0,
  }
}
