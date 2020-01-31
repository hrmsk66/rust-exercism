use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut non_primes: HashSet<u64> = HashSet::new();
    (2..=upper_bound).filter_map(|x| {
        if non_primes.contains(&x) {
            return None;
        }

        for y in ((x * 2)..=upper_bound).step_by(x as usize) {
            non_primes.insert(y);
        }

        Some(x)
    }).collect()
}
