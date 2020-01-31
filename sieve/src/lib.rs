use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut non_primes: HashSet<u64> = HashSet::new();
    (2..=upper_bound).filter(|x| {
        for y in ((x * 2)..=upper_bound).step_by(*x as usize) {
            non_primes.insert(y);
        }
        !non_primes.contains(&x)
    }).collect()
}
