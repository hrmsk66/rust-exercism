use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut r = BTreeMap::new();

    for (k, v) in h {
        for c in v {
            r.insert(c.to_ascii_lowercase(), *k);
        }
    }

    r
}
