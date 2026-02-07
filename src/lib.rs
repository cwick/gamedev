use wasm_bindgen::prelude::*;

// Import JavaScript's console.log function
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Example 1: Log to browser console
#[wasm_bindgen]
pub fn hello_console() {
    log("Hello from Rust WebAssembly! (console)");
}

// Example 2: Return a string to JavaScript
#[wasm_bindgen]
pub fn hello_string() -> String {
    String::from("Hello from Rust WebAssembly! (returned string)")
}

// Example 3: Accept parameter and return personalized greeting
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust WebAssembly.", name)
}
