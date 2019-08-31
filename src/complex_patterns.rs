
use crate::{Scan, ScanResult};
use std::ops::{RangeBounds, Bound};

/// Result of two patterns alteration
///
/// # Examples
/// ```
/// # use rep::{Pattern, Scan};
/// let pattern = Pattern("ab") | Pattern("xy");
/// assert!(pattern.test("ab"));
/// assert!(pattern.test("xy"));
/// ```
///
#[derive(Copy, Clone)]
pub struct OrPattern<A, B>(pub A, pub B);

impl<A: Scan, B: Scan> Scan for OrPattern<A, B> {
    fn scan(&self, text: &str) -> ScanResult {
        match self.0.scan(text) {
            Err(()) => self.1.scan(text),
            ok => ok,
        }
    }
}

/// Result of two patterns concatenation
///
/// # Examples
/// ```
/// # use rep::{Pattern, Scan};
/// let pattern = Pattern("ab") & Pattern("xy");
/// assert!(pattern.test("abxy"));
/// ```
///
#[derive(Copy, Clone)]
pub struct AndPattern<A, B>(pub A, pub B);

impl<A: Scan, B: Scan> Scan for AndPattern<A, B> {
    fn scan(&self, text: &str) -> ScanResult {
        let len_a = self.0.scan(text)?;
        let len_b = self.1.scan(&text[len_a..])?;
        Ok(len_a + len_b)
    }
}

/// Result of pattern repetition
///
/// # Examples
/// ```
/// # use rep::{Pattern, Scan};
/// let pattern = Pattern("a") * 3;
/// assert!(pattern.test("aaa"));
/// ```
///
#[derive(Copy, Clone)]
pub struct ManyPattern<T>(pub T, pub u32);

impl<T: Scan> Scan for ManyPattern<T> {
    fn scan(&self, text: &str) -> ScanResult {
        let mut len = 0;

        for _ in 0..self.1 {
            match self.0.scan(&text[len..]) {
                Ok(l) => len += l,
                Err(()) => return Err(()),
            }
        }

        Ok(len)
    }
}

/// Result of range pattern
///
/// # Examples
/// ```
/// # use rep::{Pattern, Scan};
/// let pattern = Pattern("a") * (1..3);
/// assert!(pattern.test("a"));
/// assert!(pattern.test("aa"));
/// ```
///
#[derive(Copy, Clone)]
pub struct RangePattern<T, R>(pub T, pub R);

impl<T: Scan, R: RangeBounds<u32>> Scan for RangePattern<T, R> {
    fn scan(&self, text: &str) -> ScanResult {

        // todo: check empty pattern + unbound range

        let mut len = 0;
        let mut count = 0;

        loop {
            match self.0.scan(&text[len..]) {
                Ok(l) => {
                    len += l;
                    count += 1;

                    match self.1.end_bound() {
                        Bound::Included(b) if *b == count     => return Ok(len),
                        Bound::Excluded(b) if *b == count + 1 => return Ok(len),
                        _ => {},
                    }
                },
                Err(_) if self.1.contains(&count) => return Ok(len),
                _ => return Err(()),
            }
        }
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
        assert!(!pattern.test(""));

        assert_eq!(pattern.scan_split("foo bar"), Ok(("foo", " bar")));
        assert_eq!(pattern.scan_split("bar"), Ok(("bar", "")));
        assert_eq!(pattern.scan_split(""), Err(()));
    }

    #[test]
    fn and_pattern() {
        let pattern = Pattern("foo") & "bar";
        assert!(pattern.test("foobar"));
        assert!(!pattern.test("foo"));
        assert!(!pattern.test("bar"));
        assert!(!pattern.test(""));

        assert_eq!(pattern.scan_split("foobar baz"), Ok(("foobar", " baz")));
        assert_eq!(pattern.scan_split("foobar"), Ok(("foobar", "")));
        assert_eq!(pattern.scan_split(""), Err(()));
    }

    #[test]
    fn many_pattern() {
        let pattern = Pattern("a") * 3;
        assert!(pattern.test("aaa"));
        assert!(!pattern.test("aa"));
        assert!(!pattern.test("aaaa"));
        assert!(!pattern.test(""));

        assert_eq!(pattern.scan_split("aaaa"), Ok(("aaa", "a")));
        assert_eq!(pattern.scan_split("aaa"), Ok(("aaa", "")));
        assert_eq!(pattern.scan_split(""), Err(()));

        let empty_pattern = Pattern("b") * 0;
        assert!(empty_pattern.test(""));
    }

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

    #[test]
    fn combine_patterns() {
        let a = Pattern("a") | "b" | "c";
        let b = Pattern("0") & "" & "1";

        let ab = a & b;
        assert!(ab.test("a01"));
        assert!(ab.test("b01"));
        assert!(ab.test("c01"));
        assert!(!ab.test("c0"));

        let a_b = a | b;
        assert!(a_b.test("a"));
        assert!(a_b.test("b"));
        assert!(a_b.test("c"));
        assert!(a_b.test("01"));
        assert!(!a_b.test("0"));

        let aaa = a * 3;
        assert!(aaa.test("aaa"));
        assert!(aaa.test("abc"));
        assert!(aaa.test("bac"));

        let bb = b * 2;
        assert!(bb.test("0101"));
        assert!(!bb.test("01"));
        assert!(!bb.test("010101"));

        let a_aaa = a * (1..=3);
        assert!(a_aaa.test("b"));
        assert!(a_aaa.test("cb"));
        assert!(a_aaa.test("bca"));
        assert!(!a_aaa.test("aabb"));
        assert!(!a_aaa.test(""));

        let b_bb = b * (1..3);
        assert!(b_bb.test("01"));
        assert!(b_bb.test("0101"));
        assert!(!b_bb.test("010101"));
        assert!(!b_bb.test(""));
    }
}
