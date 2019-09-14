
use std::ops::{RangeBounds, Bound};
use crate::capture::{CaptureResult, Capture};

/// Result of range pattern
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let pattern = Pattern("a") * (1..3);
/// assert!(pattern.test("a"));
/// assert!(pattern.test("aa"));
/// ```
///
#[derive(Copy, Clone)]
pub struct RangePattern<S, R>(pub S, pub R);

impl<'a, S: Capture<'a>, R: RangeBounds<u32>> Capture<'a> for RangePattern<S, R> {
    type Inner = S::Inner;

    fn capture(&self, text: &'a str) -> Option<CaptureResult<'a, Self::Inner>> {

        if self.0.capture("").is_some() && self.1.end_bound() == Bound::Unbounded {
            panic!("Infinity loop")
        }

        let mut count = 0;
        let mut len = 0;
        let mut cap = self.0.capture_empty(text);

        loop {
            match self.0.capture(cap.rest) {
                Some(mut c) => {
                    count += 1;
                    len += c.captures.captured_len();
                    c.captures.captured_str = &text[..len];
                    cap = c;

                    match self.1.end_bound() {
                        Bound::Included(b) if *b == count     => break Some(cap),
                        Bound::Excluded(b) if *b == count + 1 => break Some(cap),
                        _ => {},
                    }
                },
                None if self.1.contains(&count) => break Some(cap),
                None => break None,
            }
        }
    }

    fn capture_empty(&self, text: &'a str) -> CaptureResult<'a, Self::Inner> {
        self.0.capture_empty(text)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Pattern;

    #[test]
    fn range_pattern() {
        let full = Pattern("a") * ..;
        assert!(full.test(""));
        assert!(full.test("a"));
        assert!(full.test("aa"));
        assert!(full.test("aaa"));

        let from = Pattern("b") * (2..);
        assert!(!from.test(""));
        assert!(!from.test("b"));
        assert!(from.test("bb"));
        assert!(from.test("bbb"));

        let to = Pattern("c") * ..2;
        assert!(to.test(""));
        assert!(to.test("c"));
        assert!(!to.test("cc"));

        let to_inclusive = Pattern("d") * ..=2;
        assert!(to_inclusive.test(""));
        assert!(to_inclusive.test("d"));
        assert!(to_inclusive.test("dd"));
        assert!(!to_inclusive.test("ddd"));

        let range = Pattern("e") * (1..3);
        assert!(!range.test(""));
        assert!(range.test("e"));
        assert!(range.test("ee"));
        assert!(!range.test("eee"));

        let range_inclusive = Pattern("f") * (1..=2);
        assert!(!range_inclusive.test(""));
        assert!(range_inclusive.test("f"));
        assert!(range_inclusive.test("ff"));
        assert!(!range_inclusive.test("fff"));
    }
}
