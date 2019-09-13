
mod captures;
mod patterns;
mod scan_term;
mod capture;
mod match_iterator;

pub use scan_term::ScanTerm;
pub use patterns::{Pattern, cap};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex() {
        let number = Pattern(|c: char| match c {
            '0'..='9' | 'A'..='F' => true,
            _ => false,
        });
        let hex = Pattern('0') & 'x' & number * (1..);

        assert!(hex.test("0xFF94"));
        assert!(hex.test("0x0012AB"));
    }
}
