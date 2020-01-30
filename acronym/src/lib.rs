pub fn abbreviate(phrase: &str) -> String {
    let mut abbr = String::new();
    for word in phrase.split(|c: char| !c.is_alphabetic() && c != '\'') {
        if word.is_empty() {

        }
        else if word.chars().all(char::is_uppercase) {
            abbr.push(word.chars().next().unwrap());
        }
        else if word.chars().all(char::is_lowercase) {
            abbr.push_str(&word.chars().next().unwrap().to_uppercase().collect::<String>());
        }
        else {
            abbr.push_str(&word.chars().filter(|c| c.is_uppercase()).collect::<String>());
        }
    }
    abbr
}
