fn main() {
    let s = String::from("hello world");

    let first = first_word(&s[..]);

    println!("the first word in '{}' is '{}'", s, first);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
