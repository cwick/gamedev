use wasm_bindgen::prelude::*;
use std::cell::RefCell;

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

// Game Loop Infrastructure

struct GameState {
    frame_count: u32,
    elapsed_time: f32,
    is_running: bool,
}

thread_local! {
    static GAME_STATE: RefCell<GameState> = RefCell::new(GameState {
        frame_count: 0,
        elapsed_time: 0.0,
        is_running: false,
    });
}

#[wasm_bindgen]
pub fn game_init() {
    GAME_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.frame_count = 0;
        state.elapsed_time = 0.0;
        state.is_running = false;
    });
}

#[wasm_bindgen]
pub fn game_update(delta_time: f32) {
    GAME_STATE.with(|state| {
        let mut state = state.borrow_mut();

        if !state.is_running {
            return;
        }

        state.frame_count += 1;
        state.elapsed_time += delta_time;
    });
}

#[wasm_bindgen]
pub fn game_get_frame_count() -> u32 {
    GAME_STATE.with(|state| state.borrow().frame_count)
}

#[wasm_bindgen]
pub fn game_get_elapsed_time() -> f32 {
    GAME_STATE.with(|state| state.borrow().elapsed_time)
}

#[wasm_bindgen]
pub fn game_is_running() -> bool {
    GAME_STATE.with(|state| state.borrow().is_running)
}

#[wasm_bindgen]
pub fn game_start() {
    GAME_STATE.with(|state| {
        state.borrow_mut().is_running = true;
    });
}

#[wasm_bindgen]
pub fn game_stop() {
    GAME_STATE.with(|state| {
        state.borrow_mut().is_running = false;
    });
}
