fn main() {
    let mut s = String::from("me");

    let slice = slice(&s);

    s.clear();

    println!("{} {}", s, slice);
}

fn slice(s: &str) -> String {
    String::from(&s[..5])
}
