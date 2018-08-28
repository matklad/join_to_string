# join

[![Build Status](https://travis-ci.org/matklad/join.svg?branch=master)](https://travis-ci.org/matklad/join)
[![Crates.io](https://img.shields.io/crates/v/join.svg)](https://crates.io/crates/join)
[![API reference](https://docs.rs/join/badge.svg)](https://docs.rs/join/)

Join a list of items to string:

```rust
let mut buf = String::new();
join(iterable)
    .sep(", ")
    .preifx("(")
    .suffix(")")
    .to_buf(&mut buf); // .to_string()
```
