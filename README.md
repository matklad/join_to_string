# join

[![Build Status](https://travis-ci.org/matklad/join_to_string.svg?branch=master)](https://travis-ci.org/matklad/join_to_string)
[![Crates.io](https://img.shields.io/crates/v/join_to_string.svg)](https://crates.io/crates/join_to_string)
[![API reference](https://docs.rs/join_to_string/badge.svg)](https://docs.rs/join_to_string/)

Join a list of items to string:

```rust
extern crate join_to_string;
use join_to_string::join;

fn main() {
    let mut buf = String::new();
    join([1, 2, 3].iter())
        .separator(", ")
        .prefix("(")
        .suffix(")")
        .to_buf(&mut buf); // .to_string()
    assert_eq!(buf, "(1, 2, 3)");
}
```
