
use super::none_capture::NoneCapture;

#[derive(Copy, Clone, Debug)]
enum CaptureIteratorState {
    CapturedStr,
    Inner,
}

#[derive(Copy, Clone, Debug)]
pub struct Captures<'a, T> {
    iter_state: CaptureIteratorState,
    pub captured_str: &'a str,
    pub inner: T,
}

impl<'a, T> Captures<'a, T> {
    pub fn new(matched_str: &str, inner: T) -> Captures<T> {
        Captures {
            iter_state: CaptureIteratorState::CapturedStr,
            captured_str: matched_str,
            inner,
        }
    }

    pub fn with_inner<K>(&self, inner: K) -> Captures<'a, K> {
        Captures {
            iter_state: CaptureIteratorState::CapturedStr,
            captured_str: self.captured_str,
            inner,
        }
    }

    pub fn captured_len(&self) -> usize {
        self.captured_str.len()
    }
}

impl Captures<'_, NoneCapture> {
    pub fn without_capture(matched_str: &str) -> Captures<NoneCapture> {
        Captures {
            iter_state: CaptureIteratorState::CapturedStr,
            captured_str: matched_str,
            inner: NoneCapture,
        }
    }
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for Captures<'a, T> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter_state {
            CaptureIteratorState::CapturedStr => {
                self.iter_state = CaptureIteratorState::Inner;
                Some(self.captured_str)
            },
            CaptureIteratorState::Inner => self.inner.next(),
        }
    }
}

impl<'a, T> Into<&'a str> for Captures<'a, T> {
    fn into(self) -> &'a str {
        self.captured_str
    }
}
