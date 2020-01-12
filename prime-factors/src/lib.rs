pub fn factors(n: u64) -> Vec<u64> {
    let mut r = Vec::new();

    let mut curr = n;
    while 1 < curr {
        for x in 2..=curr {
            if curr % x == 0 {
                r.push(x);
                curr = curr / x;
                break
            }

            if curr == x {
                r.push(x);
                return r;
            }
        }
    }

    r
}
