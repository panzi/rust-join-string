join-string
===========

Simple library to join strings somewhat efficiently if possible.

Example:

```Rust
println!("{}",
    "foo bar baz".split_whitespace()
        .map(|s| s.chars().rev().join(""))
        .join(" "));
// Output: oof rab zab
```
