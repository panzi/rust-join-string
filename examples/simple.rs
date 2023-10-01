use join_string::StringJoin;

fn main() {
    println!("{}", ["foo", "bar", "baz"].iter().join(", "));
    println!("{}", ["foo", "bar", "baz"].join(", "));
    println!("{}", ["foo".to_owned(), "bar".to_owned(), "baz".to_owned()].join(", "));
    println!("{}", vec!["foo", "bar", "baz"].join(", "));

    // inefficient temporary string
    println!("{}", std::env::args().join(", ").into_string());
}
