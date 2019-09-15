
use crate::scan_term::ScanTerm;
use crate::captures::{Captures, NoneCapture};

pub struct CaptureResult<'a, T> {
    pub captures: Captures<'a, T>,
    pub rest: &'a str,
}

impl<'a, T> CaptureResult<'a, T> {
    pub fn new(captures: Captures<'a, T>, rest: &'a str) -> CaptureResult<'a, T> {
        CaptureResult {
            captures,
            rest,
        }
    }
}

impl<'a, T> Into<(Captures<'a, T>, &'a str)> for CaptureResult<'a, T> {
    fn into(self) -> (Captures<'a, T>, &'a str) {
        (self.captures, self.rest)
    }
}

pub trait Capture<'a> {
    type Inner: Clone;
    fn capture(&self, text: &'a str) -> Option<CaptureResult<'a, Self::Inner>>;
    fn capture_empty(&self, text: &'a str) -> CaptureResult<'a, Self::Inner>;
}

impl<'a, S: ScanTerm> Capture<'a> for S {
    type Inner = NoneCapture;

    fn capture(&self, text: &'a str) -> Option<CaptureResult<'a, Self::Inner>> {
        let (left, rest) = self.scan_split(text)?;
        let caps = Captures::without_capture(left);
        Some(CaptureResult::new(caps, rest))
    }

    fn capture_empty(&self, text: &'a str) -> CaptureResult<'a, Self::Inner> {
        let caps = Captures::without_capture("");
        CaptureResult::new(caps, text)
    }
}
