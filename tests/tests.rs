use join_string::{StringJoin, StringJoiner};

#[test]
fn basic() {
    let empty: [&str; 0] = [];
    assert_eq!(empty.iter().join(", ").into_string(), "");
    assert_eq!([""].iter().join(", ").into_string(), "");
    assert_eq!(["foo"].iter().join(", ").into_string(), "foo");
    assert_eq!(["", ""].iter().join(", ").into_string(), ", ");
    assert_eq!(["foo", "bar", "baz"].iter().join(", ").into_string(), "foo, bar, baz");
}

#[test]
fn types() {
    assert_eq!(['a', 'b', 'c'].iter().join(", ".to_owned()).into_string(), "a, b, c");
    assert_eq!([1, 2, 3].iter().join("").into_string(), "123");
    assert_eq!(["foo".to_owned(), "bar".to_owned(), "baz".to_owned()].iter().join(", ").into_string(), "foo, bar, baz");
    assert_eq!(vec![format_args!("{:02}", 1), format_args!("{:.1} {}", 3.0, 4)].iter().join(' ').into_string(), "01 3.0 4");
}

#[test]
fn into_impl() {
    let str: String = [1, 2, 3].iter().join(" + ").into();
    assert_eq!(str, "1 + 2 + 3");
    assert_eq!(<StringJoiner<_, _> as Into<String>>::into("äüö".chars().join(' ')), "ä ü ö");
}

#[test]
fn display_impl() {
    assert_eq!(format!("<{}>", ["foo", "bar", "baz"].iter().join(", ")), "<foo, bar, baz>");
}
