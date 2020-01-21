fn opener_of(c: char) -> char {
    match c {
        ']' => '[',
        '}' => '{',
        ')' => '(',
        _ => unreachable!(),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' | '}' | ')' => if stack.pop() != Some(opener_of(c)) { return false }
            _ => (),
        }
    }
    stack.is_empty()
}