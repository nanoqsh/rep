
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
        let number = Pattern('A'..='F') | ('0'..='9');
        let hex = Pattern('0') & 'x' & number * (1..);

        assert!(hex.test("0xFF94"));
        assert!(hex.test("0x12AB"));
        assert!(!hex.test("0x0G"));
        assert!(!hex.test("0x"));
        assert!(!hex.test("0+0"));
        assert!(!hex.test("0A"));

        let numbers: Vec<&str> = hex.matched_strs("0x00 0x01 0xAB 0xFF").collect();
        assert_eq!(numbers, ["0x00", "0x01", "0xAB", "0xFF"]);
    }

    #[test]
    fn parse() {
        let space = Pattern(' ') * ..;
        let name = Pattern(char::is_alphabetic) & Pattern(char::is_alphabetic) * ..;
        let arg = name & ',' & space;
        let args = (arg * ..) & name & space | space;
        let func = name & space & '(' & space & args & ')' & space & ';';

        assert!(func.test("func();"));
        assert!(func.test("func ( bar,  num,  str ) ;"));
        assert!(func.test("func(bar, str);"));
        assert!(func.test("func(bar);"));
        assert!(!func.test("func num, str);"));
        assert!(!func.test("func(bar, num, );"));
        assert!(!func.test("func(bar, num, str)"));
        assert!(!func.test("func(bar str);"));
    }
}
