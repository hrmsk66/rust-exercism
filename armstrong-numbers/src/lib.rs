pub fn is_armstrong_number(num: u32) -> bool {
    let numstr = num.to_string();
    let mut r : u32 = 0;

    for c in numstr.chars() {
        r += (c as u32 - 48).pow(numstr.len() as u32);
    }


    r == num
}
