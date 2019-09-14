
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

    #[test]
    fn parse() {
        let space = Pattern(' ') * ..;
        let name = Pattern(char::is_alphabetic) & Pattern(char::is_alphabetic) * ..;
        let arg = name & Pattern(',') & space;
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
