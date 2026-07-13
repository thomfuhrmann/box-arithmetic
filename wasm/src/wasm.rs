use box_core::BoxValue;
use box_core::parser::{Token, parser};
use box_core::store::BoxStore;
use chumsky::prelude::*;
use logos::Logos;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct BoxCalculator {
    store: BoxStore,
}

#[wasm_bindgen]
impl BoxCalculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let mut store = BoxStore::new();
        let alpha = BoxValue::alpha();
        store.store_box_with_name("alpha", alpha);

        Self { store }
    }

    /// Takes a string from JS, runs the parser and outputs the formatted string
    pub fn eval_expr(&self, input: &str) -> Result<String, JsValue> {
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

        let layout = format!("{}", val);
        Ok(layout)
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
        let input = "⌊⌈⌊□⌋,⌊□⌋⌉,⌈⌊□⌋,⌊₂□⌋⌉,₂⌈⌊₂□⌋,⌊₂□⌋⌉⌋";
        let val = calc.eval_expr(input).unwrap();
        console_log!("{val:?}");
    }
}
