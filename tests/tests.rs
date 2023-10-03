use join_string::{Join, Joiner, join, join_str, DisplayWrapper};

#[test]
fn basic() {
    let empty: [&str; 0] = [];
    assert_eq!(empty.iter().join(", ").into_string(), "");
    assert_eq!([""].iter().join(", ").into_string(), "");
    assert_eq!(["foo"].iter().join(", ").into_string(), "foo");
    assert_eq!(["", ""].iter().join(", ").into_string(), ", ");
    assert_eq!(["foo", "bar", "baz"].iter().join(", ").into_string(), "foo, bar, baz");
}

fn join_as_ref_str(elements: &[impl AsRef<str>], sep: impl AsRef<str>) -> String {
    join_str(elements, sep).into_string()
}

fn join_map_as_ref_str(elements: &[impl AsRef<str>], sep: impl std::fmt::Display) -> String {
    DisplayWrapper::map(elements).join(sep).into_string()
}

fn join_sep_as_ref_str(elements: &[impl std::fmt::Display], sep: impl AsRef<str>) -> String {
    elements.iter().join(DisplayWrapper::new(sep)).into_string()
}

#[test]
fn types() {
    assert_eq!(['a', 'b', 'c'].iter().join(", ".to_owned()).to_string(), "a, b, c");
    assert_eq!([1, 2, 3].iter().join("").into_string(), "123");
    assert_eq!([
            "foo".to_owned(),
            "bar".to_owned(),
            "baz".to_owned()
        ].iter().join(", ").into_string(),
        "foo, bar, baz");
    assert_eq!(vec![
            format_args!("{:02}", 1),
            format_args!("{:.1} {}", 3.0, 4)
        ].iter().join(' ').into_string(),
        "01 3.0 4");
    let items: [&dyn std::fmt::Display; 4] = [
        &Box::new("foo"), &"bar", &'z', &"bla".to_owned()
    ];
    assert_eq!(items.iter().join(", ").into_string(), "foo, bar, z, bla");
    assert_eq!([
            "".chars().join(""),
            "".chars().join("bla")
        ].iter().join("ab".chars().join(',')).into_string(),
        "a,b");
    assert_eq!(join_as_ref_str(&["foo", "bar", "baz"], ", "), "foo, bar, baz");
    assert_eq!(join_map_as_ref_str(&["foo", "bar", "baz"], ", "), "foo, bar, baz");
    assert_eq!(join_sep_as_ref_str(&["foo", "bar", "baz"], ", "), "foo, bar, baz");
}

#[test]
fn joinable() {
    assert_eq!(join(["foo", "bar", "baz"].as_slice(), ", ").into_string(), "foo, bar, baz");
    let mut iter = ["foo", "bar", "baz"].iter();
    assert_eq!(join(&mut iter, ", ").into_string(), "foo, bar, baz");
    assert_eq!(join(["foo", "bar", "baz"].iter(), ", ").into_string(), "foo, bar, baz");
    assert_eq!(join(["foo", "bar", "baz"].iter().rev(), ", ").into_string(), "baz, bar, foo");
    assert_eq!(join(&["foo", "bar", "baz"], ", ").into_string(), "foo, bar, baz");
    assert_eq!(join(vec!["foo", "bar", "baz"].as_slice(), ", ").into_string(), "foo, bar, baz");
    assert_eq!(join(&vec!["foo", "bar", "baz"], ", ").into_string(), "foo, bar, baz");
}

#[test]
fn complex_exprs() {
    assert_eq!("a bcd ef".split_whitespace()
        .map(str::len)
        .join(", ")
        .into_string(), "1, 3, 2");

    assert_eq!("foo bar baz".split_whitespace()
        .map(|s| s.chars().rev().join(""))
        .join(" ")
        .into_string(), "oof rab zab");
}

#[test]
fn into_impl() {
    let str: String = [1, 2, 3].iter().join(" + ").into();
    assert_eq!(str, "1 + 2 + 3");
    assert_eq!(<Joiner<_, _> as Into<String>>::into("äüö".chars().join(' ')), "ä ü ö");
}

#[test]
fn display_impl() {
    assert_eq!(format!("<{}>", ["foo", "bar", "baz"].iter().join(", ")), "<foo, bar, baz>");
    assert_eq!(format!("<{}>", [1, 2, 3].iter().cycle().take(5).join(", ")), "<1, 2, 3, 1, 2>");
}

#[test]
fn write_fmt() -> std::fmt::Result {
    let mut buffer = String::new();
    ["foo", "bar", "baz"].iter().join(", ").write_fmt(&mut buffer)?;
    assert_eq!(buffer, "foo, bar, baz");

    Ok(())
}

#[test]
fn write_io() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = Vec::new();
    ["foo", "bar", "baz"].iter().join(", ").write_io(&mut buffer)?;
    assert_eq!(buffer, b"foo, bar, baz");

    Ok(())
}
