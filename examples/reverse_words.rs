use join_string::StringJoin;

pub fn reverse_words(s: impl AsRef<str>) -> String {
    s.as_ref().split_whitespace()
         .map(|s| s.chars().rev().join(""))
         .join(" ")
         .into_string()
}

fn main() {
    println!("{}", reverse_words("foo bar baz"));

    println!("{}",
        "foo bar baz".split_whitespace()
            .map(|s| s.chars().rev().join(""))
            .join(" "));

    println!("{}",
        "foo bar baz".split_whitespace()
            .map(|s| s.chars().rev().map(|c| char::from_u32(c as u32 + 1u32).unwrap_or('?')).join(""))
            .join(" "));

    // inefficient temporary strings
    println!("{}",
        "foo bar baz".split_whitespace()
            .map(|s| s.chars().rev().map(|c| format!("<{c}>")).join(""))
            .join(" "));

    // inefficient temporary strings
    println!("{}", std::env::args().map(|s| s.chars().rev().collect::<String>()).join(" ").into_string());
}
