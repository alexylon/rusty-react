use std::str::FromStr;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref NAMES: Mutex<Vec<String>> = Mutex::new(vec![]);
}

static SUM: AtomicI32 = AtomicI32::new(0);
static J: AtomicUsize = AtomicUsize::new(0);


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
    NAMES.lock().unwrap().push(name.to_string());
}

#[wasm_bindgen]
pub fn sum_two_ints(json_string: &str) -> i32 {
    let numbers: Numbers = serde_json::from_str(json_string).unwrap_or_else(|e| panic!("Error: {}", e));
    println!("json_string: {}", json_string);
    let num1 = i32::from_str(&*numbers.num1).unwrap_or(0);
    let num2 = i32::from_str(&*numbers.num2).unwrap_or(0);
    println!("num1: {}, num2: {}", num1, num2);
    SUM.store(num1 + num2, Ordering::SeqCst);

    num1 + num2
}

#[wasm_bindgen]
pub fn get_sum() -> i32 {
    SUM.load(Ordering::SeqCst)
}

#[wasm_bindgen]
pub fn get_name_next() -> String {
    let length = NAMES.lock().unwrap().len();
    let names = NAMES.lock().unwrap().to_vec();

    J.fetch_add(1, Ordering::SeqCst);
    let j = J.load(Ordering::SeqCst);

    if j > length - 1 {
        J.store(length - 1, Ordering::SeqCst);
    }

    let k = J.load(Ordering::SeqCst);

    names.get(k).unwrap().to_string()
}

#[wasm_bindgen]
pub fn get_name_previous() -> String {
    let names = NAMES.lock().unwrap().to_vec();

    let j = J.load(Ordering::SeqCst);
    if j > 0 {
        J.fetch_sub(1, Ordering::SeqCst);
    }
    let k = J.load(Ordering::SeqCst);

    names.get(k).unwrap().to_string()
}

