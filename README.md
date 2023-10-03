join-string
===========

[![Build status](https://img.shields.io/github/actions/workflow/status/panzi/rust-join-string/test.yml?branch=main)](https://github.com/panzi/rust-join-string/actions/workflows/test.yml)
[![Release](https://img.shields.io/github/v/release/panzi/rust-join-string)](https://github.com/panzi/rust-join-string/releases)
[![License](https://img.shields.io/github/license/panzi/rust-join-string)](https://github.com/panzi/rust-join-string/blob/main/LICENSE)

A simple crate to join the elements of iterators, interspersing a separator between all elements.

This is done somewhat efficiently, if possible, meaning if the iterator is cheaply clonable you can
directly print the result of `Join::join()` without creating a temporary `String` in memory.

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

Note that the standard library already provides a similar [`Join`](https://doc.rust-lang.org/std/slice/trait.Join.html)
trait on slices, but not on interators, and the standard library version always directly returns a
new `String`. And then there are multiple other similar crates that however work a bit differently,
e.g. having more restrictions on element and separator types or always returning a `String`.
