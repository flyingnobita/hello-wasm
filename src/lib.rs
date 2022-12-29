use wasm_bindgen::prelude::*;

#[wasm_bindgen]  // modifies extern to call JS functions
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]  // modify fn to be called by JS
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
