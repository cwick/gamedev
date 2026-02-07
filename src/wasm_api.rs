use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use crate::engine::GameState;

thread_local! {
    static GAME_STATE: RefCell<GameState> = const { RefCell::new(GameState::new()) };
}

#[wasm_bindgen]
pub fn engine_init(width: f32, height: f32) {
    GAME_STATE.with(|state| {
        state.borrow_mut().init(width, height);
    });
}

#[wasm_bindgen]
pub fn engine_step(dt_seconds: f32, input_bits: u32) {
    GAME_STATE.with(|state| {
        state.borrow_mut().step(dt_seconds, input_bits);
    });
}

#[wasm_bindgen]
pub fn game_state_ptr() -> *const f32 {
    GAME_STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.update_snapshot();
        state.snapshot_ptr()
    })
}

#[wasm_bindgen]
pub fn game_state_len() -> usize {
    GAME_STATE.with(|state| state.borrow().snapshot_len())
}
