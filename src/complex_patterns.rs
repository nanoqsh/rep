
use crate::Scan;

/// Result of alteration two patterns
///
/// # Examples
/// ```
/// # use rep::{Pattern, Scan};
/// let pattern = Pattern("ab") | Pattern("xy");
/// assert!(pattern.test("ab"));
/// assert!(pattern.test("xy"));
/// ```
///
pub struct OrPattern<A, B>(pub A, pub B);

impl<A: Scan, B: Scan> Scan for OrPattern<A, B> {
    fn scan(&self, text: &str) -> Result<usize, ()> {
        match self.0.scan(text) {
            Err(()) => self.1.scan(text),
            ok => ok,
        }
    }
}

/// Result of concatenation two patterns
///
/// # Examples
/// ```
/// # use rep::{Pattern, Scan};
/// let pattern = Pattern("ab") & Pattern("xy");
/// assert!(pattern.test("abxy"));
/// ```
///
pub struct AndPattern<A, B>(pub A, pub B);

impl<A: Scan, B: Scan> Scan for AndPattern<A, B> {
    fn scan(&self, text: &str) -> Result<usize, ()> {
        let len_a = self.0.scan(text)?;
        let len_b = self.1.scan(&text[len_a..])?;
        Ok(len_a + len_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::Pattern;

    #[test]
    fn or_pattern() {
        let pattern = Pattern("foo") | "bar";
        assert!(pattern.test("foo"));
        assert!(pattern.test("bar"));
        assert!(!pattern.test("baz"));
    }

    #[test]
    fn and_pattern() {
        let pattern = Pattern("foo") & "bar";
        assert!(pattern.test("foobar"));
        assert!(!pattern.test("foo"));
        assert!(!pattern.test("bar"));
    }
}
