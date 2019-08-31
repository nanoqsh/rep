
use crate::Scan;
use std::ops::{BitOr, BitAnd};
use crate::complex_patterns::{OrPattern, AndPattern};

/// Struct for `Scan` trait using.
/// Thus usually it is `Pattern<T: Scan>(T)`.
/// Also the `Pattern<T: Scan>` itself implements `Scan` trait.
///
pub struct Pattern<T>(pub T);

impl<T: Scan> Scan for Pattern<T> {
    fn scan(&self, text: &str) -> Result<usize, ()> {
        self.0.scan(text)
    }
}

impl<A: Scan, B: Scan> BitOr<B> for Pattern<A> {
    type Output = Pattern<OrPattern<A, B>>;

    fn bitor(self, rhs: B) -> Self::Output {
        Pattern(OrPattern(self.0, rhs))
    }
}

impl<A: Scan, B: Scan> BitAnd<B> for Pattern<A> {
    type Output = Pattern<AndPattern<A, B>>;

    fn bitand(self, rhs: B) -> Self::Output {
        Pattern(AndPattern(self.0, rhs))
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
