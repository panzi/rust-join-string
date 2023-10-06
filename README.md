join-string
===========

[![Build status](https://img.shields.io/github/actions/workflow/status/panzi/rust-join-string/test.yml?branch=main)](https://github.com/panzi/rust-join-string/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/panzi/rust-join-string)](https://github.com/panzi/rust-join-string/releases)
[![License](https://img.shields.io/github/license/panzi/rust-join-string)](https://github.com/panzi/rust-join-string/blob/main/LICENSE)
[Reference](https://docs.rs/join-string/latest/join_string/)

A simple crate to join elements as a string, interspersing a separator between
all elements.

This is done somewhat efficiently, if possible, meaning if the iterator is cheaply
clonable you can directly print the result of `Join::join()` without creating a temporary
`String` in memory. The `Join::join()` method will appear on anything that implements
[`std::iter::IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html),
meaning on all iterators and collections. The elements and the separator need to implement
[`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html). Alternatively
the `Join::join_str()` method can be used to join elements that only implement
[`AsRef<str>`](https://doc.rust-lang.org/std/convert/trait.AsRef.html).

```Rust
use join_string::Join;

assert_eq!(
    "foo bar baz".split_whitespace().join(", ").into_string(),
    "foo, bar, baz");

println!("{}",
    "foo bar baz".split_whitespace()
        .map(|s| s.chars().rev().join(""))
        .join(' '));
// Output: oof rab zab
```

You can also write the result more directly to a
[`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) or
[`std::fmt::Write`](https://doc.rust-lang.org/std/fmt/trait.Write.html)
even if the backing iterator doesn't implement
[`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html).

```Rust
use join_string::Join;

["foo", "bar", "baz"].join(", ").write_io(std::io::stdout())?;

let mut str = String::new();
["foo", "bar", "baz"].join(", ").write_fmt(&mut str)?;
```

Note that the standard library already provides a similar
[`std::slice::Join`](https://doc.rust-lang.org/std/slice/trait.Join.html)
trait on slices, but not on iterators, and the standard library version always directly returns a
new `String`. Further there are multiple other similar crates that however work a bit differently,
e.g. having more restrictions on element and separator types or always returning a `String`.
