pub fn square(s: u32) -> u64 {
    match s {
        s if s < 1 || 64 < s => panic!("Square must be between 1 and 64"),
        _ => 1<<(s - 1)
    }
}

pub fn total() -> u64 {
    (1..=64).fold(0, |acc, x| acc + square(x))
}
