
#[derive(Copy, Clone, Debug)]
pub struct NoneCapture;

impl Iterator for NoneCapture {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
