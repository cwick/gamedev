use std::cell::RefCell;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy)]
struct GameState {
    frame_count: u32,
    elapsed_time: f32,
    is_running: bool,
}

thread_local! {
    static GAME_STATE: RefCell<GameState> = const { RefCell::new(GameState {
        frame_count: 0,
        elapsed_time: 0.0,
        is_running: false,
    }) };
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
pub fn game_get_state() -> Vec<f32> {
    GAME_STATE.with(|state| {
        let s = state.borrow();
        vec![
            s.elapsed_time,
            s.frame_count as f32,
            if s.is_running { 1.0 } else { 0.0 }
        ]
    })
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
