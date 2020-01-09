pub fn build_proverb(list: &[&str]) -> String {
    let mut r = String::new();

    match list.len() {
        0 => { return r; },
        1 => {},
        _ => {
            for (i, item) in list.iter().enumerate() {
                if i + 1 == list.len() {
                    break;
                }
                r.push_str(&format!("For want of a {} the {} was lost.\n", item, list[i+1]));
            }
        }
    }

    r.push_str(&format!("And all for the want of a {}.", list[0]));
    r
}
