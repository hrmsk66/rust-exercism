use std::cmp::Ordering;

pub fn find<T, S>(array: S, key: T) -> Option<usize>
where
    T: Ord,
    S: AsRef<[T]>,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    let half = array.len() / 2;
    match array[half].cmp(&key) {
        Ordering::Equal => Some(half),
        Ordering::Greater => find(&array[..half], key),
        Ordering::Less => find(&array[half + 1..], key).map(|p| p + half + 1),
    }
}