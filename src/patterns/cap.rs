
use crate::capture::{Capture, CaptureResult};
use crate::captures::Captures;

#[derive(Copy, Clone)]
pub struct Cap<S>(pub S);

impl<'a, S: Capture<'a>> Capture<'a> for Cap<S> {
    type Inner = Captures<'a, S::Inner>;

    fn capture(&self, text: &'a str) -> Option<CaptureResult<'a, Self::Inner>> {
        let (caps, rest) = self.0.capture(text)?.into();
        let inner = Captures::new(caps.captured_str, caps.inner);
        Some(CaptureResult::new(caps.with_inner(inner), rest))
    }

    fn capture_empty(&self, text: &'a str) -> CaptureResult<'a, Self::Inner> {
        let caps = self.0.capture_empty(text).captures;
        let inner = Captures::new("", caps.inner);
        CaptureResult::new(caps.with_inner(inner), text)
    }
}
