pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack : Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' => {
                if stack.is_empty() || stack.pop().unwrap() != '[' {
                    return false
                }
            },
            '}' => {
                if stack.is_empty() || stack.pop().unwrap() != '{' {
                    return false
                }
            },
            ')' => {
                if stack.is_empty() || stack.pop().unwrap() != '(' {
                    return false
                }
            },
            _ => (),
        }
    }

    if stack.is_empty() {
        true
    } else {
        false
    }
}
