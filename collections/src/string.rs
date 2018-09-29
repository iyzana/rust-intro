pub fn pig_latin(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| convert_word(&word))
        .collect::<Vec<_>>()
        .join(" ")
}

fn convert_word(word: &str) -> String {
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    if ['a', 'e', 'i', 'o', 'u'].contains(&first_char) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay", chars.collect::<String>(), &first_char)
    }
}
