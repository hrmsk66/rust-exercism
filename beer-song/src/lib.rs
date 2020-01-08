pub fn verse(n: u32) -> String {
    let result = match n {
        0 => concat!(
            "No more bottles of beer on the wall, ",
            "no more bottles of beer.\n",
            "Go to the store and buy some more, ",
            "99 bottles of beer on the wall.\n"
        ).to_string(),
        1 => concat!(
            "1 bottle of beer on the wall, ",
            "1 bottle of beer.\n",
            "Take it down and pass it around, ",
            "no more bottles of beer on the wall.\n"
        ).to_string(),
        2 => concat!(
            "2 bottles of beer on the wall, ",
            "2 bottles of beer.\n",
            "Take one down and pass it around, ",
            "1 bottle of beer on the wall.\n"
        ).to_string(),
        _ => [
                format!("{} bottles of beer on the wall, ", n),
                format!("{} bottles of beer.\n", n),
                format!("Take one down and pass it around, "),
                format!("{} bottles of beer on the wall.\n", n-1),
            ].join("")
    };
    result
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(|n| verse(n)).collect::<Vec<String>>().join("\n")
}
