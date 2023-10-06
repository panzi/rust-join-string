use join_string::Join;

fn main() -> std::io::Result<()> {
    println!("{}", ["foo", "bar", "baz"].join(", "));
    println!("{}", ['a', 'b', 'c'].join(", "));
    println!(
        "{}",
        ["foo".to_owned(), "bar".to_owned(), "baz".to_owned()].join(", ")
    );
    println!("{}", vec![1, 2, 3].iter().cycle().take(5).join(", "));
    println!("{}", "äüö".chars().join(' '));
    std::env::args().join(", ").write_io(std::io::stdout())?;
    println!();

    // inefficient temporary string
    let str: String = std::env::args().join(", ").into();
    println!("{}", str);

    Ok(())
}
