pub fn pig_latin(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            let first_char = chars.next().unwrap();

            if ['a', 'e', 'i', 'o', 'u'].contains(&first_char) {
                format!("{}-hay", word)
            } else {
                format!("{}-{}ay", chars.collect::<String>(), &first_char)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
