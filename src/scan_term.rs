
/// Object which implements ScanTerm trait
/// is terminal element of pattern expression tree
///
pub trait ScanTerm {

    /// Pattern matching function
    ///
    /// `scan_term` takes a text and matching it with pattern.
    ///
    /// `scan_term` returns `Some(len)` where range `0..len` (in bytes) is matched substring.
    /// Otherwise result is `None`
    ///
    /// # Examples
    /// ```
    /// # use rep::ScanTerm;
    /// let pattern = "abc";
    /// assert_eq!(pattern.scan_term("abcdef"), Some(3));
    /// ```
    /// `scan` returns `Some(3)` because `"abc"` is three bytes string
    /// and beginnig of `"abcdef"` string is matched with this pattern.
    ///
    /// ```
    /// # use rep::ScanTerm;
    /// let pattern = "aaa";
    /// assert_eq!(pattern.scan_term("abc"), None);
    /// ```
    /// In that case pattern is not matching `scan` returns `None`.
    ///
    fn scan_term(&self, text: &str) -> Option<usize>;

    fn scan_split<'a>(&'_ self, text: &'a str) -> Option<(&'a str, &'a str)> {
        Some(text.split_at(self.scan_term(text)?))
    }
}

impl ScanTerm for &str {

    /// ScanTerm implementation of string
    ///
    /// It checks a text starts with pattern string.
    /// If so then result is `Some(len)` where `len` is pattern length.
    /// Otherwise `None`.
    ///
    /// Pattern entry in text is a range `0..len` in bytes
    ///
    /// # Examples
    /// ```
    /// # use rep::ScanTerm;
    /// let pattern = "012";
    /// assert_eq!(pattern.scan_term("01234"), Some(3));
    /// ```
    ///
    /// An empty pattern always included in any text
    /// ```
    /// # use rep::ScanTerm;
    /// let empty_pattern = "";
    /// assert_eq!(empty_pattern.scan_term("abc"), Some(0));
    /// assert_eq!(empty_pattern.scan_term(""), Some(0));
    /// ```
    ///
    fn scan_term(&self, text: &str) -> Option<usize> {
        if text.starts_with(self) {
            Some(self.len())
        } else {
            None
        }
    }
}

impl ScanTerm for String {
    fn scan_term(&self, text: &str) -> Option<usize> {
        self.as_str().scan_term(text)
    }
}

impl ScanTerm for char {
    fn scan_term(&self, text: &str) -> Option<usize> {
        if text.starts_with(*self) {
            Some(self.len_utf8())
        }
        else {
            None
        }
    }
}

impl<F: Fn(char) -> bool> ScanTerm for F {
    fn scan_term(&self, text: &str) -> Option<usize> {
        match text.chars().next() {
            Some(ch) if self(ch) => Some(ch.len_utf8()),
            _ => None,
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
        assert_eq!(pattern.scan_term(same), Some(same.len()));

        let part = "some text more";
        assert_eq!(pattern.scan_term(part), Some(pattern.len()));

        let another = "text";
        assert_eq!(pattern.scan_term(another), None);
    }

    #[test]
    fn scan_str_utf8() {
        let pattern = "привет";
        assert_eq!(pattern.scan_term("привет"), Some(pattern.len()));
        assert_eq!(pattern.scan_term("привет, мир!"), Some(pattern.len()));
        assert_eq!(pattern.scan_term("прив"), None);
    }

    #[test]
    fn scan_empty_str() {
        let empty_pattern = "";
        let text = "text";
        assert_eq!(empty_pattern.scan_term(text), Some(0));
        assert_eq!(empty_pattern.scan_term(""), Some(0));

        let pattern = "baz";
        assert_eq!(pattern.scan_term(""), None);
    }

    #[test]
    fn scan_empty_string() {
        let pattern = String::from("some text");
        assert_eq!(pattern.scan_term("some text"), Some(pattern.len()));
        assert_eq!(pattern.scan_term("some text more"), Some(pattern.len()));
        assert_eq!(pattern.scan_term("text"), None);
    }

    #[test]
    fn scan_split_str() {
        let pattern = "foo";
        let text = "foobar";
        assert_eq!(pattern.scan_split(text), Some(("foo", "bar")));

        let err_split = "bar";
        assert_eq!(pattern.scan_split(err_split), None);
    }

    #[test]
    fn scan_char() {
        let pattern = 'a';
        assert_eq!(pattern.scan_term("a"), Some(1));
        assert_eq!(pattern.scan_term("ab"), Some(1));
        assert_eq!(pattern.scan_term("b"), None);
        assert_eq!(pattern.scan_term(""), None);
    }

    #[test]
    fn scan_char_utf8() {
        let pattern = 'ф';
        assert_eq!(pattern.scan_term("ф"), Some(pattern.len_utf8()));
        assert_eq!(pattern.scan_term("фы"), Some(pattern.len_utf8()));
        assert_eq!(pattern.scan_term("ы"), None);
    }

    #[test]
    fn scan_fn() {
        let whitespace = char::is_whitespace;
        assert_eq!(whitespace.scan_term(" "), Some(1));
        assert_eq!(whitespace.scan_term(""), None);
        assert_eq!(whitespace.scan_term("."), None);

        let alpha = char::is_alphabetic;
        assert_eq!(alpha.scan_term("a"), Some(1));
        assert_eq!(alpha.scan_term("."), None);

        let a_or_b = |c: char| c == 'a' || c == 'b';
        assert_eq!(a_or_b.scan_term("a"), Some(1));
        assert_eq!(a_or_b.scan_term("b"), Some(1));
        assert_eq!(a_or_b.scan_term("c"), None);

        let not_a = |c: char| c != 'a';
        assert_eq!(not_a.scan_term("x"), Some(1));
        assert_eq!(not_a.scan_term("a"), None);
    }
}
