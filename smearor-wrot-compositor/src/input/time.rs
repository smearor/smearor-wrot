use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn get_time() -> u32 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_millis() as u32
}

#[cfg(test)]
mod tests {
    use super::get_time;

    #[test]
    fn test_get_time_returns_valid_timestamp() {
        let time = get_time();
        assert!(time > 0);
        assert!(time < 4_294_967_295);
    }
}
