use std::str::FromStr;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicI32, Ordering};

static SUM: AtomicI32 = AtomicI32::new(0);


#[derive(Debug, Serialize, Deserialize)]
pub struct Numbers {
    num1: String,
    num2: String,
}

struct AppState {
    sum: i32,
}

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
pub fn sum_two_ints(json_string: &str) -> i32 {
    let numbers: Numbers = serde_json::from_str(json_string).unwrap_or_else(|e| panic!("Error: {}", e));
    println!("json_string: {}", json_string);
    let num1 = i32::from_str(&*numbers.num1).unwrap_or(0);
    let num2 = i32::from_str(&*numbers.num2).unwrap_or(0);
    println!("num1: {}, num2: {}", num1, num2);
    SUM.store(num1 + num2, Ordering::Relaxed);

    num1 + num2
}

#[wasm_bindgen]
pub fn get_sum() -> i32 {
    SUM.load(Ordering::Relaxed)
}

