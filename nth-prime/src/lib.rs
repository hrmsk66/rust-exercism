pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        _ => {
            let mut cnt = 2;
            let mut x = 5;

            while cnt < n {
                x += 2;
                if is_prime(x) {
                    cnt += 1;
                }
            }
            x
        }
    }
}

fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false
    }

    let upto = ((n as f64).sqrt() as u32) + 1;
    for i in 2..upto {
        if n % i == 0 {
            return false
        }
    }

    true
}