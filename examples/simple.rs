use join_string::StringJoin;

fn main() {
    println!("{}", ["foo", "bar", "baz"].iter().join(", "));
    println!("{}", ['a', 'b', 'c'].iter().join(", "));
    println!("{}", ["foo".to_owned(), "bar".to_owned(), "baz".to_owned()].iter().join(", "));
    println!("{}", vec![1, 2, 3].iter().join(", "));
    println!("{}", "äüö".chars().join(' '));

    // inefficient temporary string
    println!("{}", std::env::args().join(", ").into_string());
    let str: String = std::env::args().join(", ").into();
    println!("{}", str);
}
