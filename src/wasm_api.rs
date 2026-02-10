use crate::engine::Engine;
use crate::games::build_game;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    static ENGINE: RefCell<Option<Engine>> = const { RefCell::new(None) };
}

#[wasm_bindgen]
pub fn engine_init(game_name: &str, width: f32, height: f32) {
    ENGINE.with(|engine| {
        let (world, schedule, snapshot) = build_game(game_name, width, height);
        *engine.borrow_mut() = Some(Engine::new(world, schedule, snapshot));
    });
}

#[wasm_bindgen]
pub fn engine_step(dt_seconds: f32, input_bits: u32) {
    ENGINE.with(|engine| {
        if let Some(engine) = engine.borrow_mut().as_mut() {
            engine.step(dt_seconds, input_bits);
        }
    });
}

#[wasm_bindgen]
pub fn game_state_ptr() -> *const f32 {
    ENGINE.with(|engine| {
        if let Some(engine) = engine.borrow_mut().as_mut() {
            engine.snapshot_ptr()
        } else {
            std::ptr::null()
        }
    })
}

#[wasm_bindgen]
pub fn game_state_len() -> usize {
    ENGINE.with(|engine| {
        engine
            .borrow()
            .as_ref()
            .map(|engine| engine.snapshot_len())
            .unwrap_or(0)
    })
}
