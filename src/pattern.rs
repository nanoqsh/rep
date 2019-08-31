
use crate::Scan;

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

use crate::complex_patterns::{
    OrPattern,
    AndPattern,
    ManyPattern,
    RangePattern,
};

/// Struct for `Scan` trait using.
/// Also the `Pattern<T: Scan>` itself implements `Scan` trait.
///
#[derive(Copy, Clone)]
pub struct Pattern<T>(pub T);

impl<T: Scan> Scan for Pattern<T> {
    fn scan(&self, text: &str) -> Result<usize, ()> {
        self.0.scan(text)
    }
}

/// Combination of patterns with `|` operator (alteration)
///
/// # Examples
/// ```
/// # use rep::{Pattern, Scan};
/// let a_or_b = Pattern("a") | Pattern("b");
/// assert!(a_or_b.test("a"));
/// assert!(a_or_b.test("b"));
/// ```
///
impl<A: Scan, B: Scan> BitOr<B> for Pattern<A> {
    type Output = Pattern<OrPattern<A, B>>;

    fn bitor(self, rhs: B) -> Self::Output {
        Pattern(OrPattern(self.0, rhs))
    }
}

/// Combination of patterns with `&` operator (concatination)
///
/// # Examples
/// ```
/// # use rep::{Pattern, Scan};
/// let ab = Pattern("a") & Pattern("b");
/// assert!(ab.test("ab"));
/// ```
///
impl<A: Scan, B: Scan> BitAnd<B> for Pattern<A> {
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
/// # use rep::{Pattern, Scan};
/// let aaa = Pattern("a") * 3;
/// assert!(aaa.test("aaa"));
/// ```
///
impl<A: Scan> Mul<u32> for Pattern<A> {
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
/// # use rep::{Pattern, Scan};
/// let full = Pattern("a") * ..;
/// assert!(full.test(""));
/// assert!(full.test("a"));
/// assert!(full.test("aa"));
/// ```
///
impl<A: Scan> Mul<RangeFull> for Pattern<A> {
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
/// # use rep::{Pattern, Scan};
/// let from = Pattern("a") * (2..);
/// assert!(from.test("aa"));
/// assert!(from.test("aaa"));
/// ```
///
impl<A: Scan> Mul<RangeFrom<u32>> for Pattern<A> {
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
/// # use rep::{Pattern, Scan};
/// let to = Pattern("a") * (..3);
/// assert!(to.test(""));
/// assert!(to.test("a"));
/// assert!(to.test("aa"));
/// ```
///
impl<A: Scan> Mul<RangeTo<u32>> for Pattern<A> {
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
/// # use rep::{Pattern, Scan};
/// let to_inclusive = Pattern("a") * (..=2);
/// assert!(to_inclusive.test(""));
/// assert!(to_inclusive.test("a"));
/// assert!(to_inclusive.test("aa"));
/// ```
///
impl<A: Scan> Mul<RangeToInclusive<u32>> for Pattern<A> {
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
/// # use rep::{Pattern, Scan};
/// let range = Pattern("a") * (0..2);
/// assert!(range.test(""));
/// assert!(range.test("a"));
/// ```
///
impl<A: Scan> Mul<Range<u32>> for Pattern<A> {
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
/// # use rep::{Pattern, Scan};
/// let range_inclusive = Pattern("a") * (0..=2);
/// assert!(range_inclusive.test(""));
/// assert!(range_inclusive.test("a"));
/// assert!(range_inclusive.test("aa"));
/// ```
///
impl<A: Scan> Mul<RangeInclusive<u32>> for Pattern<A> {
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
        let same = "some text";
        assert_eq!(pattern.scan(same), Ok(same.len()));
        assert_eq!(pattern.scan("some text more"), Ok(pattern.0.len()));
        assert_eq!(pattern.scan("text"), Err(()));
    }

    #[test]
    fn pattern_char() {
        let pattern = Pattern('a');
        assert_eq!(pattern.scan("a"), Ok(1));
        assert_eq!(pattern.scan("ab"), Ok(1));
        assert_eq!(pattern.scan("b"), Err(()));
    }
}
