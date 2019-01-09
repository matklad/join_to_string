use std::fmt::{self, Write};

/// Joins an interable of `fmt::Display` items to stirng.
/// Returns a builder struct to specify custom prefix (defaults to `""`),
/// suffix (defaults to `""`), separator (defaults to `", "`), and destination
/// (either a fresh `String`, or an existing `&mut String` buffer).
pub fn join<I: Iterator>(items: I) -> JoinBuilder<'static, I> {
    JoinBuilder {
        items,
        prefix: "",
        suffix: "",
        sep: ", ",
    }
}

/// Builder struct to specify separator, suffix and prefix of the
/// join operation.
pub struct JoinBuilder<'a, I> {
    items: I,
    prefix: &'a str,
    suffix: &'a str,
    sep: &'a str,
}

impl<'a, I> JoinBuilder<'a, I> {
    /// Set the prefix.
    pub fn prefix(self, prefix: &'a str) -> Self {
        JoinBuilder { prefix, .. self }
    }
    /// Set the suffix.
    pub fn suffix(self, suffix: &'a str) -> Self {
        JoinBuilder { suffix, .. self }
    }
    /// Set both suffix and prefix.
    pub fn surround_with(self, prefix: &'a str, suffix: &'a str) -> Self {
        self.prefix(prefix).suffix(suffix)
    }
    // Set
    pub fn separator(self, sep: &'a str) -> Self {
        JoinBuilder { sep, .. self }
    }
}

impl<'a, I> JoinBuilder<'a, I>
where
  I: Iterator,
  I::Item: ToBuf
{
    pub fn to_buf(self, buf: &mut String) {
        buf.push_str(self.prefix);
        let mut first = true;
        for item in self.items {
            if !first {
                buf.push_str(self.sep);
            }
            first = false;
            item.to_buf(buf);
        }
        buf.push_str(self.suffix);
    }

    pub fn to_string(self) -> String {
        let cap =
            self.prefix.len()
            + self.suffix.len()
            + self.items.size_hint().0 * self.sep.len();
        let mut buf = String::with_capacity(cap);
        self.to_buf(&mut buf);
        buf
    }
}

pub trait ToBuf {
    fn to_buf(&self, buf: &mut String);
}

impl<T: fmt::Display> ToBuf for T {
    fn to_buf(&self, buf: &mut String) {
        write!(buf, "{}", self)
            .unwrap()
    }
}
