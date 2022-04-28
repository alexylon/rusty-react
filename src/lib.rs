use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn big_computation() {

    alert("Big computation in Rust");
}

#[wasm_bindgen]
pub fn welcome(name: &str) {

    alert(&format!("Hello {}, from Rust!", name));
}

#[wasm_bindgen]
pub fn sum_two_ints(a: i32, b: i32) -> i32 {
    a + b
}
