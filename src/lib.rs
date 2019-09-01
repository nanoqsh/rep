
mod pattern;
mod complex_patterns;
mod scan;

pub use scan::{
    Scan,
    ScanResult,
};

pub use pattern::{
    Pattern,
    Matches,
};

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

        let test_str = "( 0; 0x0;0x; 0xFF, 0x00AB) ";
        let hex_matches: Vec<&str> = hex.matches(test_str).collect();
        assert_eq!(hex_matches, ["0x0", "0xFF", "0x00AB"]);
    }
}
