use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use rand::Rng;

struct GameState {
    ball_x: f32,
    ball_y: f32,
    ball_vx: f32,
    ball_vy: f32,
    paddle1_y: f32,
    paddle2_y: f32,
    score1: f32,
    score2: f32,
    field_w: f32,
    field_h: f32,
}

thread_local! {
    static GAME_STATE: RefCell<GameState> = const { RefCell::new(GameState {
        ball_x: 0.0,
        ball_y: 0.0,
        ball_vx: 0.0,
        ball_vy: 0.0,
        paddle1_y: 0.0,
        paddle2_y: 0.0,
        score1: 0.0,
        score2: 0.0,
        field_w: 0.0,
        field_h: 0.0,
    }) };
}

#[wasm_bindgen]
pub fn engine_init(width: f32, height: f32) {
    GAME_STATE.with(|state| {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(150.0..250.0);

        let mut state = state.borrow_mut();
        state.field_w = width;
        state.field_h = height;
        state.ball_x = width / 2.0;
        state.ball_y = height / 2.0;
        state.ball_vx = angle.cos() * speed;
        state.ball_vy = angle.sin() * speed;
        state.paddle1_y = height / 2.0;
        state.paddle2_y = height / 2.0;
        state.score1 = 0.0;
        state.score2 = 0.0;
    });
}

#[wasm_bindgen]
pub fn engine_step(dt_seconds: f32) {
    GAME_STATE.with(|state| {
        let mut state = state.borrow_mut();

        state.ball_x += state.ball_vx * dt_seconds;
        state.ball_y += state.ball_vy * dt_seconds;

        if state.ball_x <= 0.0 || state.ball_x >= state.field_w {
            state.ball_vx = -state.ball_vx;
            state.ball_x = state.ball_x.clamp(0.0, state.field_w);
        }

        if state.ball_y <= 0.0 || state.ball_y >= state.field_h {
            state.ball_vy = -state.ball_vy;
            state.ball_y = state.ball_y.clamp(0.0, state.field_h);
        }
    });
}

#[wasm_bindgen]
pub fn game_get_state() -> Vec<f32> {
    GAME_STATE.with(|state| {
        let s = state.borrow();
        vec![
            s.ball_x,
            s.ball_y,
            s.ball_vx,
            s.ball_vy,
            s.paddle1_y,
            s.paddle2_y,
            s.score1,
            s.score2,
            s.field_w,
            s.field_h,
        ]
    })
}
