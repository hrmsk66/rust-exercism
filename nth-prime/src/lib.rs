pub fn nth(n: u32) -> u32 {
    let mut x = 2;
    let mut cnt = 0;

    while n != cnt {
        x += 1;

        if is_prime(x) {
            cnt += 1;
        }
    }
    x
}

fn is_prime(n: u32) -> bool {
    for v in 2..n {
        if n == v {
            continue
        }
        if n % v == 0 {
            return false
        }
    }
    true
}
