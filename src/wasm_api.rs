use crate::engine::Engine;
use crate::engine::TUNING_STATUS_UNKNOWN_PARAM;
use crate::games::build_game;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    static ENGINE: RefCell<Option<Engine>> = const { RefCell::new(None) };
}

#[wasm_bindgen]
pub fn engine_init(game_name: &str, width: f32, height: f32) {
    ENGINE.with(|engine| {
        let (world, schedule, snapshot, tuning_api) = build_game(game_name, width, height);
        *engine.borrow_mut() = Some(Engine::new(world, schedule, snapshot, tuning_api));
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

#[wasm_bindgen]
pub fn engine_set_tuning_param(param_id: u32, value: f32) -> u32 {
    ENGINE.with(|engine| {
        if let Some(engine) = engine.borrow_mut().as_mut() {
            engine.set_tuning_param(param_id, value)
        } else {
            TUNING_STATUS_UNKNOWN_PARAM
        }
    })
}

#[wasm_bindgen]
pub fn engine_get_tuning_param(param_id: u32) -> f32 {
    ENGINE.with(|engine| {
        if let Some(engine) = engine.borrow().as_ref() {
            engine.get_tuning_param(param_id).unwrap_or(f32::NAN)
        } else {
            f32::NAN
        }
    })
}

#[wasm_bindgen]
pub fn engine_reset_tuning_defaults() {
    ENGINE.with(|engine| {
        if let Some(engine) = engine.borrow_mut().as_mut() {
            engine.reset_tuning_defaults();
        }
    });
}

#[wasm_bindgen]
pub fn engine_tuning_schema_version() -> u32 {
    ENGINE.with(|engine| {
        engine
            .borrow()
            .as_ref()
            .map(|engine| engine.tuning_schema_version())
            .unwrap_or(0)
    })
}
