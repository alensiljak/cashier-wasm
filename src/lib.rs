//! Cashier WASM
//! 

use wasm_bindgen::prelude::*;

// example for importing an external function, from JS.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// // example of exporting a function to JS.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn hi() -> String {
    "Hello!".to_string()
}