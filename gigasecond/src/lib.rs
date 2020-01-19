use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let r: DateTime<Utc> = start + Duration::seconds(1_000_000_000);

    // cargo test -- --nocapture --test-threads=1
    println!("{} + Gigaseconds = {}",
        start.format("%Y-%m-%d %H:%M:%S").to_string(),
        r.format("%Y-%m-%d %H:%M:%S").to_string());

    r
}
