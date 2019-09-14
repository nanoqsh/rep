
use std::ops::{
    BitOr,
    BitAnd,
    Mul,
    RangeFull,
    RangeFrom,
    RangeTo,
    RangeToInclusive,
    Range,
    RangeInclusive,
};

use crate::capture::{Capture, CaptureResult};
use crate::match_iterator::MatchIterator;
use crate::patterns::{OrPattern, AndPattern, Cap, ManyPattern, RangePattern};

#[derive(Copy, Clone)]
pub struct Pattern<S>(pub S);

impl<'a, S: Capture<'a>> Pattern<S> {
    pub fn matches(self, text: &'a str) -> MatchIterator<'a, S> {
        MatchIterator::new(self.0, text)
    }

    pub fn matched_strs(self, text: &'a str) -> impl Iterator<Item=&'a str> {
        self.matches(text).map(|m| m.captured_str)
    }

    pub fn test(&self, text: &'a str) -> bool {
        match self.0.capture(text) {
            Some(res) => res.rest.is_empty(),
            None => false,
        }
    }
}

impl<'a, S: Capture<'a>> Capture<'a> for Pattern<S> {
    type Inner = S::Inner;

    fn capture(&self, text: &'a str) -> Option<CaptureResult<'a, Self::Inner>> {
        self.0.capture(text)
    }

    fn capture_empty(&self, text: &'a str) -> CaptureResult<'a, Self::Inner> {
        self.0.capture_empty(text)
    }
}

pub fn cap<'a, P: Capture<'a>>(pattern: P) -> Pattern<Cap<P>> {
    Pattern(Cap(pattern))
}

/// Combination of patterns with `|` operator (alteration)
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let a_or_b = Pattern("a") | Pattern("b");
/// assert!(a_or_b.test("a"));
/// assert!(a_or_b.test("b"));
/// ```
///
impl<'a, A: Capture<'a>, B: Capture<'a>> BitOr<B> for Pattern<A> {
    type Output = Pattern<OrPattern<A, B>>;

    fn bitor(self, rhs: B) -> Self::Output {
        Pattern(OrPattern(self.0, rhs))
    }
}

/// Combination of patterns with `&` operator (concatination)
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let ab = Pattern("a") & Pattern("b");
/// assert!(ab.test("ab"));
/// ```
///
impl<'a, A: Capture<'a>, B: Capture<'a>> BitAnd<B> for Pattern<A> {
    type Output = Pattern<AndPattern<A, B>>;

    fn bitand(self, rhs: B) -> Self::Output {
        Pattern(AndPattern(self.0, rhs))
    }
}

/// Multiplication (repetition) of patterns
///
/// Specifies the number of repetitions of pattern
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let aaa = Pattern("a") * 3;
/// assert!(aaa.test("aaa"));
/// ```
///
impl<'a, A: Capture<'a>> Mul<u32> for Pattern<A> {
    type Output = Pattern<ManyPattern<A>>;

    fn mul(self, rhs: u32) -> Self::Output {
        Pattern(ManyPattern(self.0, rhs))
    }
}

/// Pattern range (RangeFull)
///
/// Using pattern range as `pattern * ..`
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let full = Pattern("a") * ..;
/// assert!(full.test(""));
/// assert!(full.test("a"));
/// assert!(full.test("aa"));
/// ```
///
impl<'a, A: Capture<'a>> Mul<RangeFull> for Pattern<A> {
    type Output = Pattern<RangePattern<A, RangeFull>>;

    fn mul(self, rhs: RangeFull) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

/// Pattern range (RangeFrom)
///
/// Using pattern range as `pattern * (n..)`
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let from = Pattern("a") * (2..);
/// assert!(from.test("aa"));
/// assert!(from.test("aaa"));
/// ```
///
impl<'a, A: Capture<'a>> Mul<RangeFrom<u32>> for Pattern<A> {
    type Output = Pattern<RangePattern<A, RangeFrom<u32>>>;

    fn mul(self, rhs: RangeFrom<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

/// Pattern range (RangeTo)
///
/// Using pattern range as `pattern * (..n)`
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let to = Pattern("a") * (..3);
/// assert!(to.test(""));
/// assert!(to.test("a"));
/// assert!(to.test("aa"));
/// ```
///
impl<'a, A: Capture<'a>> Mul<RangeTo<u32>> for Pattern<A> {
    type Output = Pattern<RangePattern<A, RangeTo<u32>>>;

    fn mul(self, rhs: RangeTo<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

/// Pattern range (RangeToInclusive)
///
/// Using pattern range as `pattern * (..=n)`
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let to_inclusive = Pattern("a") * (..=2);
/// assert!(to_inclusive.test(""));
/// assert!(to_inclusive.test("a"));
/// assert!(to_inclusive.test("aa"));
/// ```
///
impl<'a, A: Capture<'a>> Mul<RangeToInclusive<u32>> for Pattern<A> {
    type Output = Pattern<RangePattern<A, RangeToInclusive<u32>>>;

    fn mul(self, rhs: RangeToInclusive<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

/// Pattern range (Range)
///
/// Using pattern range as `pattern * (n..m)`
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let range = Pattern("a") * (0..2);
/// assert!(range.test(""));
/// assert!(range.test("a"));
/// ```
///
impl<'a, A: Capture<'a>> Mul<Range<u32>> for Pattern<A> {
    type Output = Pattern<RangePattern<A, Range<u32>>>;

    fn mul(self, rhs: Range<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

/// Pattern range (RangeInclusive)
///
/// Using pattern range as `pattern * (n..=m)`
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let range_inclusive = Pattern("a") * (0..=2);
/// assert!(range_inclusive.test(""));
/// assert!(range_inclusive.test("a"));
/// assert!(range_inclusive.test("aa"));
/// ```
///
impl<'a, A: Capture<'a>> Mul<RangeInclusive<u32>> for Pattern<A> {
    type Output = Pattern<RangePattern<A, RangeInclusive<u32>>>;

    fn mul(self, rhs: RangeInclusive<u32>) -> Self::Output {
        Pattern(RangePattern(self.0, rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_str() {
        let pattern = Pattern("some text");
        assert!(pattern.test("some text"));
        assert!(!pattern.test("some text_"));
        assert!(!pattern.test("text"));
        assert!(!pattern.test(""));
    }

    #[test]
    fn pattern_char() {
        let pattern = Pattern('a');
        assert!(pattern.test("a"));
        assert!(!pattern.test("ab"));
        assert!(!pattern.test("b"));
    }

    #[test]
    fn pattern_matches() {
        let xy = Pattern("xy");
        let xy_matches: Vec<&str> = xy
            .matched_strs("ab cxy yx xy x xxy.")
            .collect();
        assert_eq!(xy_matches, ["xy", "xy", "xy"]);

        let x_y = Pattern('x') | 'y';
        let x_y_matches: Vec<&str> = x_y
            .matched_strs("ab cx y lol*yx")
            .collect();
        assert_eq!(x_y_matches, ["x", "y", "y", "x"]);
    }

    #[test]
    fn pattern_caps() {
        let x = Pattern('x');
        let y = Pattern('y');
        let xy = cap(x) & cap(y);
        let mut xy_matches = xy.matches("zxy.");

        let cap = xy_matches.next().unwrap();
        let captures: Vec<&str> = cap.collect();
        assert_eq!(captures, ["xy", "x", "y"]);

        assert!(xy_matches.next().is_none());
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

        let not_a_3 = Pattern(|c: char| c != 'a') * 3;
        assert!(not_a_3.test("zzz"));
        assert!(!not_a_3.test("zz"));
        assert!(!not_a_3.test("zzzz"));
        assert!(!not_a_3.test("aaa"));
        assert!(!not_a_3.test("zza"));

        let whitespace_or_alpha = Pattern(char::is_whitespace) | char::is_alphabetic;
        assert!(whitespace_or_alpha.test("a"));
        assert!(whitespace_or_alpha.test(" "));
        assert!(!whitespace_or_alpha.test("*"));

        let w_a_range = whitespace_or_alpha * (1..=2);
        assert!(w_a_range.test("a"));
        assert!(w_a_range.test("a "));
        assert!(!w_a_range.test(""));
        assert!(!w_a_range.test("z+"));
        assert!(!w_a_range.test(" f "));
    }

    #[test]
    #[should_panic(expected = "Infinity loop")]
    fn infinity_loop_panic() {
        let empty_pattern = Pattern("") * (1..);
        empty_pattern.test("a");
    }
}
