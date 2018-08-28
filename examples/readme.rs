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
