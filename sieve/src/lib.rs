use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut set = HashSet::<u64>::new();
    let mut r = Vec::<u64>::new();

    for n in 2..=upper_bound {
        if !set.contains(&n) {
            r.push(n);

            let mut m = n * 2;
            while m <= upper_bound {
               set.insert(m);
               m += n;
            }
        }
    }
    r
}
