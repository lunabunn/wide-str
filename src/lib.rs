extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn wide_str(ts: TokenStream) -> TokenStream {
    let mut iter = ts.into_iter();
    if let Some(TokenTree::Literal(lit)) = iter.next() {
        if iter.next().is_some() {
            panic!("Invalid argument(s)!");
        }
        let old: String = lit.to_string();
        let mut new: String = String::new();
        let len = old.len();

        let mut iter = old.chars().skip(1).take(len - 2);
        while let Some(c) = iter.next() {
            if c == '\\' {
                new.push(match iter.next().unwrap() {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '0' => '\0',
                    
                    'x' => {
                        let mut code_iter = (&mut iter).take(2);
                        if (&mut code_iter).count() != 2 {
                            panic!(r"this form of character escape may only be used with characters in the range [\x00-\x7f]");
                        }
                        let parsed = u8::from_str_radix(&code_iter.collect::<String>(), 16).unwrap_or_else(|_| panic!("Syntax Error: ASCII hex escape code must contain only hex characters"));
                        if parsed > b'\x7F' {
                            panic!("Syntax Error: ASCII hex escape code must be at most 0x7F");
                        }
                        parsed as char
                    },
                    'u' => {
                        if iter.next() != Some('{') {
                            panic!("Syntax Error: Missing `{` to begin the unicode escape");
                        }
                        let code_iter = (&mut iter).take_while(|x| *x != '}').filter(|x| *x != '_').enumerate();
                        let parsed = u32::from_str_radix(&code_iter.map(|val| {
                            let (i, c) = val;
                            if i > 6 {
                                panic!("Syntax Error: Unicode escape code must have at most 6 digits");
                            }
                            c
                        }).collect::<String>(), 16).unwrap_or_else(|_| panic!("Syntax Error: ASCII hex escape code must contain only hex characters"));
                        core::char::from_u32(parsed).expect("Syntax Error: Unicode escape code must be at most 0x10FFFF")
                    },

                    c => c,
                });
            } else {
                new.push(c);
            }
        }
        new.push('\0');

        let mut result = "[".to_string();
        for c in new.chars() {
            result.push_str(&format!("0x{:x}, ", c as u16));
        }
        result.push(']');

        result.parse().unwrap()
    } else {
        panic!("Invalid argument(s)!");
    }
}
