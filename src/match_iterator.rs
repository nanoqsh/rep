
use crate::capture::Capture;
use crate::captures::Captures;

pub struct MatchIterator<'a, P> {
    pattern: P,
    rest: &'a str,
}

impl<'a, P: Capture<'a>> MatchIterator<'a, P> {
    pub fn new(pattern: P, text: &'a str) -> MatchIterator<'a, P> {
        MatchIterator {
            pattern,
            rest: text,
        }
    }
}

impl<'a, P: Capture<'a>> Iterator for MatchIterator<'a, P> {
    type Item = Captures<'a, P::Inner>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.pattern.capture(self.rest) {
                Some(cap) => {
                    self.rest = cap.rest;
                    break Some(cap.captures)
                },
                _ if self.rest.len() != 0 => {
                    self.rest = &self.rest[1..];
                },
                _ => break None,
            }
        }
    }
}
