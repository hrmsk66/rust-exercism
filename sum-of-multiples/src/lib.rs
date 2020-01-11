use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set = HashSet::new();
    
    for &x in factors.iter() {
        if x == 0 { continue; }

        (x..limit).step_by(x as usize).for_each(|n| {
            set.insert(n);
            ()
        });
    }

    set.iter().fold(0, |sum, x| sum + x)
}
