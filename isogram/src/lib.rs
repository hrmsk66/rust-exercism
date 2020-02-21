use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate.chars().all(|c| !c.is_ascii_alphabetic() || set.insert(c.to_ascii_lowercase()))
}
