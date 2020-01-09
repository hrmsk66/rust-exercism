pub fn build_proverb(list: &[&str]) -> String {
    let mut proverbs = vec![];

    for (left, right) in list.iter().zip(list.iter().skip(1)) {
        proverbs.push(format!("For want of a {} the {} was lost.", left, right));
    }

    if let Some(first) = list.first() {
        proverbs.push(format!("And all for the want of a {}.", first));
    }

    proverbs.join("\n")
}