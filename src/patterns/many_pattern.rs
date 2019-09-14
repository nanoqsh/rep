
use crate::capture::{CaptureResult, Capture};

/// Result of pattern repetition
///
/// # Examples
/// ```
/// # use rep::{Pattern, ScanTerm};
/// let pattern = Pattern("a") * 3;
/// assert!(pattern.test("aaa"));
/// ```
///
#[derive(Copy, Clone)]
pub struct ManyPattern<S>(pub S, pub u32);

impl<'a, S: Capture<'a>> Capture<'a> for ManyPattern<S> {
    type Inner = S::Inner;

    fn capture(&self, text: &'a str) -> Option<CaptureResult<'a, Self::Inner>> {
        let mut cap = self.0.capture_empty(text);
        let mut len = 0;

        for _ in 0..self.1 {
            cap = self.0.capture(cap.rest)?;
            len += cap.captures.captured_len();
            cap.captures.captured_str = &text[..len];
        }

        Some(cap)
    }

    fn capture_empty(&self, text: &'a str) -> CaptureResult<'a, Self::Inner> {
        self.0.capture_empty(text)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Pattern;

    #[test]
    fn many_pattern() {
        let pattern = Pattern("ab") * 3;
        assert!(pattern.test("ababab"));
        assert!(!pattern.test("aba"));
        assert!(!pattern.test("a"));
        assert!(!pattern.test("abab"));
        assert!(!pattern.test("abababab"));
        assert!(!pattern.test(""));

        let empty_pattern = Pattern("b") * 0;
        assert!(empty_pattern.test(""));

        let pattern = Pattern("z") * 4;
        assert!(!pattern.test(""));
        assert!(!pattern.test("z"));
        assert!(!pattern.test("zz"));
        assert!(!pattern.test("zzz"));
        assert!(pattern.test("zzzz"));
        assert!(!pattern.test("zzzzz"));
    }
}
