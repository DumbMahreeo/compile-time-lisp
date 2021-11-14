mod parse_utils;

use crate::parse_utils::parse_value;
use proc_macro::TokenStream;
use std::{cell::RefCell, rc::Rc};

#[proc_macro]
pub fn lisp(input: TokenStream) -> TokenStream {
    let env = Rc::new(RefCell::new(rust_lisp::default_env()));

    let mut input = input.to_string();

    if input.is_empty() {
        return "()".parse().unwrap();
    }

    input = input.replace("_ @ ", "'");

    macro_rules! replace_question_mark {
        ($($ident:literal),*) => {
            $(input = input.replace(concat!($ident, " ?"), concat!($ident, "?"));)*
        }
    }

    replace_question_mark!("null", "number", "symbol", "boolean", "procedure", "pair");

    if !input.starts_with('\'') {
        input.insert(0, '(');
        input.push(')');
    }

    let mut ast = rust_lisp::parse(&input);

    if let Some(expr) = ast.next() {
        match expr {
            Ok(expr) => {
                match rust_lisp::eval(env, &expr) {
                    Ok(value) => {
                        return parse_value(&value, false)
                            .parse()
                            .expect("Couldn't parse type in macro");
                    }

                    Err(e) => panic!("Couldn't evaluate expression (evaluation error):\n{}", e),
                };
            }

            Err(e) => panic!("Couldn't evaluate expression (parse error):\n{}", e),
        }
    }

    "()".parse().unwrap()
}
