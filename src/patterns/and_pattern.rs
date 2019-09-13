
use crate::capture::{Capture, CaptureResult};
use crate::captures::{DoubleCaptures, Captures};

#[derive(Copy, Clone)]
pub struct AndPattern<A, B>(pub A, pub B);

impl<'a, A: Capture<'a>, B: Capture<'a>> Capture<'a> for AndPattern<A, B> {
    type Inner = DoubleCaptures<A::Inner, B::Inner>;

    fn capture(&self, text: &'a str) -> Option<CaptureResult<'a, Self::Inner>> {
        let (caps_a, rest) = self.0.capture(text)?.into();
        let (caps_b, rest) = self.1.capture(rest)?.into();

        let captured_str = &text[..caps_a.captured_len() + caps_b.captured_len()];
        let inner = DoubleCaptures::new(caps_a.inner, caps_b.inner);
        let caps = Captures::new(captured_str, inner);

        Some(CaptureResult::new(caps, rest))
    }

    fn capture_empty(&self, text: &'a str) -> CaptureResult<'a, Self::Inner> {
        let inner_a = self.0.capture_empty(text).captures.inner;
        let inner_b = self.1.capture_empty(text).captures.inner;
        let inner = DoubleCaptures::new(inner_a, inner_b);
        let caps = Captures::new("", inner);
        CaptureResult::new(caps, text)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Pattern;

    #[test]
    fn and_pattern() {
        let pattern = Pattern("foo") & "bar";
        assert!(pattern.test("foobar"));
        assert!(!pattern.test("foo"));
        assert!(!pattern.test("bar"));
        assert!(!pattern.test(""));

        let a = Pattern("") & "a";
        assert!(a.test("a"));

        let b = Pattern("") & "b";
        assert!(b.test("b"));

        let empty_pattern = Pattern("") & "";
        assert!(empty_pattern.test(""));
        assert!(!empty_pattern.test("a"));
        assert!(!empty_pattern.test("b"));
    }
}
