
use crate::capture::{Capture, CaptureResult};
use crate::captures::{EitherCaptures, Captures};

#[derive(Copy, Clone)]
pub struct OrPattern<A, B>(pub A, pub B);

impl<'a, A: Capture<'a>, B: Capture<'a>> Capture<'a> for OrPattern<A, B> {
    type Inner = EitherCaptures<A::Inner, B::Inner>;

    fn capture(&self, text: &'a str) -> Option<CaptureResult<'a, Self::Inner>> {
        let (caps, rest) = if let Some(res) = self.0.capture(text) {
            let inner = EitherCaptures::Left(res.captures.inner);
            let caps = Captures::new(res.captures.captured_str, inner);
            (caps, res.rest)
        } else {
            let res = self.1.capture(text)?;
            let inner = EitherCaptures::Right(res.captures.inner);
            let caps = Captures::new(res.captures.captured_str, inner);
            (caps, res.rest)
        };

        Some(CaptureResult::new(caps, rest))
    }

    fn capture_empty(&self, text: &'a str) -> CaptureResult<'a, Self::Inner> {
        let inner = EitherCaptures::Left(self.0.capture_empty(text).captures.inner);
        let caps = Captures::new("", inner);
        CaptureResult::new(caps, text)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Pattern;

    #[test]
    fn or_pattern() {
        let pattern = Pattern("foo") | "bar";
        assert!(pattern.test("foo"));
        assert!(pattern.test("bar"));
        assert!(!pattern.test("baz"));
        assert!(!pattern.test(""));

        let a = Pattern("a") | "";
        assert!(a.test(""));
        assert!(a.test("a"));
        assert!(!a.test("b"));

        let b = Pattern("") | "b";
        assert!(b.test(""));

        // This pattern will match "" first
        // thus "b" will never match
        assert!(!b.test("b"));
        assert!(!b.test("a"));

        let empty_pattern = Pattern("") | "";
        assert!(empty_pattern.test(""));
        assert!(!empty_pattern.test("x"));
    }
}
