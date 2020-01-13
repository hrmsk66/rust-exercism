pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];

    match(2..=n).find(|x| n % x == 0) {
        Some(x) => {
            result.push(x);
            result.append(&mut factors(n / x));
        },
        None => {}
    }
    result
}