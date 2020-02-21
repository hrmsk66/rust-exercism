use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    for c in candidate.to_ascii_lowercase().chars().filter(|&c| c != ' ' && c != '-') {
        if !set.insert(c) {
            return false;
        }
    }
    return true
}
