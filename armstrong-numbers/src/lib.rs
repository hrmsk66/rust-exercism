pub fn is_armstrong_number(num: u32) -> bool {
    let nums : Vec<u32> = num
        .to_string()
        .chars()
        .map(|n| n.to_digit(10).unwrap())
        .collect();

        num == nums.iter().fold(0, |sum, x| sum + x.pow(nums.len() as u32))
}
