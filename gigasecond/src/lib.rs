use time::PrimitiveDateTime as DateTime;
use core::time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGASECOND: u64 = 1_000_000_000;
    start + Duration::from_secs(GIGASECOND)
}