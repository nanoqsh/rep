
/// Result of scan function in Scan trait
///
/// If string starts with some substring which is matching with pattern
/// then result is `Ok(position)`, where `position` is a pointer to match ending.
///
/// If string doesn't start with matched substring then result is `Err(())`
///
pub type ScanResult = Result<usize, ()>;

pub trait Scan {

    /// Pattern matching function
    ///
    /// `scan` takes a text and matching it with pattern.
    ///
    /// `scan` returns `Ok(len)` where range `0..len` (in bytes) is matched substring.
    /// Otherwise result is `Err(())`
    ///
    /// # Examples
    /// ```
    /// # use rep::Scan;
    /// let pattern = "abc";
    /// assert_eq!(pattern.scan("abcdef"), Ok(3));
    /// ```
    /// `scan` returns `Ok(3)` because `"abc"` is three bytes string
    /// and beginnig of `"abcdef"` string is matched with this pattern.
    ///
    /// ```
    /// # use rep::Scan;
    /// let pattern = "aaa";
    /// assert_eq!(pattern.scan("abc"), Err(()));
    /// ```
    /// In that case pattern is not matching `scan` returns `Err(())`.
    ///
    fn scan(&self, text: &str) -> ScanResult;

    fn test(&self, text: &str) -> bool {
        match self.scan(text) {
            Ok(len) => text.len() == len,
            _ => false,
        }
    }

    fn scan_split<'a>(&self, text: &'a str) -> Result<(&'a str, &'a str), ()> {
        match self.scan(text) {
            Ok(len) => Ok(text.split_at(len)),
            _ => Err(()),
        }
    }
}

impl Scan for &str {

    /// Scan implementation of string
    ///
    /// It checks a text starts with pattern string.
    /// If so then result is `Ok(len)` where `len` is pattern length.
    /// Otherwise `Err(())`.
    ///
    /// Pattern entry in text is a range `0..len` in bytes
    ///
    /// # Examples
    /// ```
    /// # use rep::Scan;
    /// let pattern = "012";
    /// assert_eq!(pattern.scan("01234"), Ok(3));
    /// ```
    ///
    /// An empty pattern always included in any text
    /// ```
    /// # use rep::Scan;
    /// let empty_pattern = "";
    /// assert_eq!(empty_pattern.scan("abc"), Ok(0));
    /// assert_eq!(empty_pattern.scan(""), Ok(0));
    /// ```
    ///
    fn scan(&self, text: &str) -> ScanResult {
        if text.starts_with(self) {
            Ok(self.len())
        }
        else {
            Err(())
        }
    }
}

impl Scan for String {
    fn scan(&self, text: &str) -> ScanResult {
        self.as_str().scan(text)
    }
}

impl Scan for char {
    fn scan(&self, text: &str) -> ScanResult {
        if text.starts_with(*self) {
            Ok(self.len_utf8())
        }
        else {
            Err(())
        }
    }
}

impl<F: Fn(char) -> bool> Scan for F {
    fn scan(&self, text: &str) -> ScanResult {
        match text.chars().next() {
            Some(ch) if self(ch) => Ok(ch.len_utf8()),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_str() {
        let pattern = "some text";
        let same = "some text";
        assert_eq!(pattern.scan(same), Ok(same.len()));

        let part = "some text more";
        assert_eq!(pattern.scan(part), Ok(pattern.len()));

        let another = "text";
        assert_eq!(pattern.scan(another), Err(()));
    }

    #[test]
    fn scan_str_utf8() {
        let pattern = "привет";
        assert_eq!(pattern.scan("привет"), Ok(pattern.len()));
        assert_eq!(pattern.scan("привет, мир!"), Ok(pattern.len()));
        assert_eq!(pattern.scan("прив"), Err(()));
    }

    #[test]
    fn scan_empty_str() {
        let empty_pattern = "";
        let text = "text";
        assert_eq!(empty_pattern.scan(text), Ok(0));
        assert_eq!(empty_pattern.scan(""), Ok(0));

        let pattern = "baz";
        assert_eq!(pattern.scan(""), Err(()));
    }

    #[test]
    fn scan_empty_string() {
        let pattern = String::from("some text");
        assert_eq!(pattern.scan("some text"), Ok(pattern.len()));
        assert_eq!(pattern.scan("some text more"), Ok(pattern.len()));
        assert_eq!(pattern.scan("text"), Err(()));
    }

    #[test]
    fn test_str() {
        let pattern = "baz";
        assert!(pattern.test("baz"));
        assert!(!pattern.test("bar"));

        let empty_pattern = "";
        assert!(empty_pattern.test(""));
    }

    #[test]
    fn scan_split_str() {
        let pattern = "foo";
        let text = "foobar";
        assert_eq!(pattern.scan_split(text), Ok(("foo", "bar")));

        let err_split = "bar";
        assert_eq!(pattern.scan_split(err_split), Err(()));
    }

    #[test]
    fn scan_char() {
        let pattern = 'a';
        assert_eq!(pattern.scan("a"), Ok(1));
        assert_eq!(pattern.scan("ab"), Ok(1));
        assert_eq!(pattern.scan("b"), Err(()));
        assert_eq!(pattern.scan(""), Err(()));
    }

    #[test]
    fn scan_char_utf8() {
        let pattern = 'ф';
        assert_eq!(pattern.scan("ф"), Ok(pattern.len_utf8()));
        assert_eq!(pattern.scan("фы"), Ok(pattern.len_utf8()));
        assert_eq!(pattern.scan("ы"), Err(()));
    }

    #[test]
    fn scan_fn() {
        let whitespace = char::is_whitespace;
        assert_eq!(whitespace.scan(" "), Ok(1));
        assert_eq!(whitespace.scan(""), Err(()));
        assert_eq!(whitespace.scan("."), Err(()));

        let alpha = char::is_alphabetic;
        assert_eq!(alpha.scan("a"), Ok(1));
        assert_eq!(alpha.scan("."), Err(()));

        let a_or_b = |c: char| c == 'a' || c == 'b';
        assert_eq!(a_or_b.scan("a"), Ok(1));
        assert_eq!(a_or_b.scan("b"), Ok(1));
        assert_eq!(a_or_b.scan("c"), Err(()));
    }

    #[test]
    fn test_fn() {
        let whitespace = |c: char| c.is_whitespace();
        assert!(whitespace.test(" "));
        assert!(!whitespace.test("-"));

        let not_a = |c: char| c != 'a';
        assert!(not_a.test("x"));
        assert!(!not_a.test("a"));
    }
}
