use wasm_bindgen::prelude::*;

// Example 1: Return a string to JavaScript
#[wasm_bindgen]
pub fn hello_string() -> String {
    String::from("Hello from Rust WebAssembly! (returned string)")
}

// Example 2: Accept parameter and return personalized greeting
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust WebAssembly.", name)
}
