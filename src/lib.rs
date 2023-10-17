//! Cashier WASM
//! 

use wasm_bindgen::prelude::*;

// example for importing an external function, from JS.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// example of exporting a function to JS.
#[wasm_bindgen]
pub fn greeting(name: &str) {
    alert(&format!("Hello, {}!", name));
}