use std::time::SystemTime;

pub fn epoch() -> u128 {
    let t = SystemTime::now();
    t.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
}