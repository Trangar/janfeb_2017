use __time::OffsetDateTime;

pub fn get() -> u64 {
    (OffsetDateTime::now_utc() - OffsetDateTime::unix_epoch()).whole_milliseconds() as u64
}

pub fn has_elapsed(time: &mut u64, interval_in_ms: u64) -> bool {
    let compare_time = get();
    if *time + interval_in_ms < compare_time {
        *time += interval_in_ms;
        true
    } else {
        false
    }
}

pub fn since(time: &mut u64) -> u64 {
    let time_now = get();
    let diff = time_now - *time;
    *time = time_now;
    diff
}
