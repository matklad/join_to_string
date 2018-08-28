extern crate join_to_string;
use join_to_string::join;

#[test]
fn smoke() {
    assert_eq!(
        join([1, 2, 3].iter()).to_string(),
        "1, 2, 3",
    );
    assert_eq!(
        join([1, 2, 3].iter())
            .surround_with("(", ")")
            .to_string(),
        "(1, 2, 3)",
    );
    assert_eq!(
        join([1, 2, 3].iter())
            .prefix("[")
            .suffix("]")
            .separator(" ")
            .to_string(),
        "[1 2 3]",
    );
    assert_eq!(
        join(None::<i32>.into_iter()).to_string(),
        "",
    );
}
