use std::panic;

use box_core::BoxValue;
use box_core::display::{BoxDisplay, OutputFormat};
use box_core::parser::{Token, parser};
use box_core::store::BoxStore;
use chumsky::prelude::*;
use logos::Logos;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct BoxCalculator {
    store: BoxStore,
}

#[derive(Serialize, Deserialize)]
pub struct Value {
    mixed: String,
    mixed_mul: String,
    boxed: String,
    boxed_mul: String,
}

impl Value {
    pub fn new(mixed: String, mixed_mul: String, boxed: String, boxed_mul: String) -> Self {
        Self {
            mixed,
            mixed_mul,
            boxed,
            boxed_mul,
        }
    }
}

#[wasm_bindgen]
impl BoxCalculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let mut store = BoxStore::new();
        let alpha = BoxValue::alpha();
        store.store_with_name("α", alpha);

        Self { store }
    }

    /// Takes a string from JS, runs the parser and outputs the formatted string
    pub fn eval_expr(&self, input: &str) -> Result<JsValue, JsValue> {
        let lexer = Token::lexer(input);
        let mut tokens = vec![];
        for (token, span) in lexer.spanned() {
            match token {
                Ok(token) => tokens.push(token),
                Err(e) => {
                    return Err(JsValue::from_str(&format!(
                        "lexer error at {:?}: {:?}",
                        span, e
                    )));
                }
            }
        }

        // parse the tokens to construct an AST
        let ast = match parser().parse(&tokens).into_result() {
            Ok(expr) => expr,
            Err(e) => {
                return Err(JsValue::from_str(&format!("parser error: {:?}", e)));
            }
        };

        // evaluate the AST to get the result
        let val = ast.eval(&self.store);

        let mut disp = BoxDisplay::from(val);
        let mixed = format!("{}", disp);
        let mixed_mul = format!("{:#}", disp);

        disp.set_format(OutputFormat::Boxed);
        let boxed = format!("{}", disp);
        let boxe_mul = format!("{:#}", disp);

        let val = Value::new(mixed, mixed_mul, boxed, boxe_mul);
        serde_wasm_bindgen::to_value(&val).map_err(|e| e.to_string().into())
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::{console_log, wasm_bindgen_test};

    use crate::wasm::BoxCalculator;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_calculator() {
        let calc = BoxCalculator::new();
        let input = "⌊□,□⌋+⌊□,□⌋";
        let val = calc.eval_expr(input).unwrap();
        console_log!("{val:?}");
    }
}
