use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let length = candidate.to_ascii_lowercase().chars().filter(|&c| c != ' ' && c != '-').count();
    let set : HashSet<char> = candidate.to_ascii_lowercase().chars().filter(|&c| c != ' ' && c != '-').collect();
    set.len() == length
}
