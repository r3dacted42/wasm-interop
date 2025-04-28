use wasm_bindgen::prelude::*;

// Basic arithmetic functions
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

// String handling
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to WebAssembly interop.", name)
}

// Return a boolean
#[wasm_bindgen]
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

// Simple error handling example
#[wasm_bindgen]
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        f64::NAN // Return NaN to indicate error
    } else {
        a / b
    }
}