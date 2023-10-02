use join_string::{StringJoin, StringJoiner};

#[test]
fn basic() {

    assert_eq!(["foo", "bar", "baz"].iter().join(", ").into_string(), "foo, bar, baz");
    assert_eq!(['a', 'b', 'c'].iter().join(", ").into_string(), "a, b, c");
    assert_eq!(['a', 'b', 'c'].iter().join("").into_string(), "abc");
    assert_eq!(["foo".to_owned(), "bar".to_owned(), "baz".to_owned()].iter().join(", ").into_string(), "foo, bar, baz");
    let str: String = vec![1, 2, 3].iter().join(", ").into();
    assert_eq!(str, "1, 2, 3");
    assert_eq!(<StringJoiner<_, _> as Into<String>>::into("äüö".chars().join(" & ")), "ä & ü & ö");
}
