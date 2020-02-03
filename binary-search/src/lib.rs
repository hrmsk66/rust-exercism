use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }

    let mid = array.len() / 2;
    match array[mid].cmp(&key) {
        Ordering::Equal => Some(mid),
        Ordering::Greater => find(&array[..mid], key),
        Ordering::Less => find(&array[mid + 1..], key).map(|n| n + mid + 1),
    }
}
