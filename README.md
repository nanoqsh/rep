# Regular Expression Parser

Simple and expressive regular expression parser built into Rust.
Unlike classic regular expressions, the pattern compiles during the bulding a Rust program.

## Examples
Use the `Pattern` struct to make a pattern. You can combine any patterns via `|`, `&` operators and also multiply pattern via `*` operator.
You can use strings, chars, functions, or ranges as a pattern.

```rust
let pattern = Pattern('a') | Pattern('b');
let or_shorter = Pattern('a') | 'b';
```

This allows you to construct any expression. For example, a parser for a hexadecimal number:
```rust
let number = Pattern('A'..='F') | ('0'..='9');
let hex = Pattern('0') & 'x' & number * (1..);

assert!(hex.test("0xFF94"));
assert!(hex.test("0x12AB"));

// collect matches
let numbers: Vec<&str> = hex.matched_strs("0x00 0x01 0xAB 0xFF").collect();
assert_eq!(numbers, ["0x00", "0x01", "0xAB", "0xFF"]);
```

Or for example, a function parser in a code snippet:
```rust
let space = Pattern(' ') * ..;
let name = Pattern(char::is_alphabetic) & Pattern(char::is_alphabetic) * ..;
let arg = name & ',' & space;
let args = (arg * ..) & name & space | space;
let func = name & space & '(' & space & args & ')' & space & ';';

assert!(func.test("func();"));
assert!(func.test("func ( bar,  num,  str ) ;"));
assert!(func.test("func(bar, str);"));
assert!(func.test("func(bar);"));
```
