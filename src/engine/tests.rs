use super::*;

const DT: f32 = 1.0 / 60.0;
const FIELD_WIDTH: f32 = 800.0;
const FIELD_HEIGHT: f32 = 600.0;

#[test]
fn ball_bounces_off_top_wall() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.ball.position_x = FIELD_WIDTH / 2.0;
    game.ball.position_y = 5.0;
    game.ball.velocity_x = 0.0;
    game.ball.velocity_y = -300.0;

    let velocity_before = game.ball.velocity_y;
    game.step(DT, 0);

    assert!(
        game.ball.velocity_y > 0.0,
        "ball should bounce down after hitting top wall"
    );
    assert_eq!(
        game.ball.velocity_y, -velocity_before,
        "ball should reverse vertical direction"
    );
}

#[test]
fn ball_bounces_off_bottom_wall() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.ball.position_x = FIELD_WIDTH / 2.0;
    game.ball.position_y = FIELD_HEIGHT - 5.0;
    game.ball.velocity_x = 0.0;
    game.ball.velocity_y = 300.0;

    let velocity_before = game.ball.velocity_y;
    game.step(DT, 0);

    assert!(
        game.ball.velocity_y < 0.0,
        "ball should bounce up after hitting bottom wall"
    );
    assert_eq!(
        game.ball.velocity_y, -velocity_before,
        "ball should reverse vertical direction"
    );
}

#[test]
fn ball_bounces_off_left_paddle() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.ball.position_x = PADDLE1_X + PADDLE_WIDTH / 2.0 + BALL_RADIUS + 2.0;
    game.ball.position_y = FIELD_HEIGHT / 2.0;
    game.ball.velocity_x = -200.0;
    game.ball.velocity_y = 0.0;
    game.paddles[0].position_y = FIELD_HEIGHT / 2.0;

    game.step(DT, 0);

    assert!(
        game.ball.velocity_x > 0.0,
        "ball should bounce right after hitting left paddle"
    );
}

#[test]
fn ball_bounces_off_right_paddle() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.ball.position_x = PADDLE2_X - PADDLE_WIDTH / 2.0 - BALL_RADIUS - 2.0;
    game.ball.position_y = FIELD_HEIGHT / 2.0;
    game.ball.velocity_x = 200.0;
    game.ball.velocity_y = 0.0;
    game.paddles[1].position_y = FIELD_HEIGHT / 2.0;

    game.step(DT, 0);

    assert!(
        game.ball.velocity_x < 0.0,
        "ball should bounce left after hitting right paddle"
    );
}

#[test]
fn paddle_cannot_move_above_top_edge() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.paddles[0].position_y = PADDLE_HEIGHT / 2.0 + 10.0;

    game.step(DT, INPUT_UP);

    assert!(
        game.paddles[0].position_y >= PADDLE_HEIGHT / 2.0,
        "paddle should not move above top edge"
    );
}

#[test]
fn paddle_cannot_move_below_bottom_edge() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.paddles[0].position_y = FIELD_HEIGHT - PADDLE_HEIGHT / 2.0 - 10.0;

    game.step(DT, INPUT_DOWN);

    assert!(
        game.paddles[0].position_y <= FIELD_HEIGHT - PADDLE_HEIGHT / 2.0,
        "paddle should not move below bottom edge"
    );
}

#[test]
fn ball_exiting_left_scores_for_player_two() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.ball.position_x = 5.0;
    game.ball.position_y = FIELD_HEIGHT / 2.0;
    game.ball.velocity_x = -400.0;
    game.ball.velocity_y = 0.0;

    game.step(DT, 0);

    assert_eq!(game.player_two_score, 1);
    assert_eq!(game.player_one_score, 0);
}

#[test]
fn ball_exiting_right_scores_for_player_one() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.ball.position_x = FIELD_WIDTH - 5.0;
    game.ball.position_y = FIELD_HEIGHT / 2.0;
    game.ball.velocity_x = 400.0;
    game.ball.velocity_y = 0.0;

    game.step(DT, 0);

    assert_eq!(game.player_one_score, 1);
    assert_eq!(game.player_two_score, 0);
}

#[test]
fn game_ends_at_winning_score() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.winning_score = 3;
    game.player_one_score = 2;
    game.ball.position_x = FIELD_WIDTH - 5.0;
    game.ball.position_y = FIELD_HEIGHT / 2.0;
    game.ball.velocity_x = 400.0;
    game.ball.velocity_y = 0.0;

    game.step(DT, 0);

    assert_eq!(game.player_one_score, 3);
    assert_eq!(game.phase, GamePhase::GameOver);

    let ball_x_before = game.ball.position_x;
    game.step(DT, 0);
    assert_eq!(
        game.ball.position_x, ball_x_before,
        "step should be a no-op after game over"
    );
}

#[test]
fn scores_accumulate_across_rounds() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;

    for round in 1..=4 {
        game.ball.position_x = FIELD_WIDTH - 5.0;
        game.ball.position_y = FIELD_HEIGHT / 2.0;
        game.ball.velocity_x = 400.0;
        game.ball.velocity_y = 0.0;
        game.step(DT, 0);
        assert_eq!(game.player_one_score, round);
    }

    for round in 1..=3 {
        game.ball.position_x = 5.0;
        game.ball.position_y = FIELD_HEIGHT / 2.0;
        game.ball.velocity_x = -400.0;
        game.ball.velocity_y = 0.0;
        game.step(DT, 0);
        assert_eq!(game.player_two_score, round);
    }

    assert_eq!(game.player_one_score, 4);
    assert_eq!(game.player_two_score, 3);
}

#[test]
fn init_resets_scores_and_phase() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.player_one_score = 5;
    game.player_two_score = 3;
    game.phase = GamePhase::GameOver;

    game.init(FIELD_WIDTH, FIELD_HEIGHT);

    assert_eq!(game.player_one_score, 0);
    assert_eq!(game.player_two_score, 0);
    assert_eq!(game.phase, GamePhase::Playing);
}

#[test]
fn ball_bounces_upward_off_paddle_top_edge() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;

    let paddle_y = FIELD_HEIGHT / 2.0;
    let paddle_top = paddle_y - PADDLE_HEIGHT / 2.0;
    game.paddles[0].position_y = paddle_y;

    game.ball.position_x = PADDLE1_X;
    game.ball.position_y = paddle_top - BALL_RADIUS - 1.0;
    game.ball.velocity_x = 0.0;
    game.ball.velocity_y = 200.0;

    game.step(DT, 0);

    assert!(
        game.ball.velocity_y < 0.0,
        "expected ball to bounce upward (negative velocity_y), got {}",
        game.ball.velocity_y
    );
    assert_eq!(
        game.ball.velocity_y, -200.0,
        "expected velocity_y to reflect to -200"
    );
}

#[test]
fn ball_bounces_downward_off_paddle_bottom_edge() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;

    let paddle_y = FIELD_HEIGHT / 2.0;
    let paddle_bottom = paddle_y + PADDLE_HEIGHT / 2.0;
    game.paddles[0].position_y = paddle_y;

    game.ball.position_x = PADDLE1_X;
    game.ball.position_y = paddle_bottom + BALL_RADIUS + 1.0;
    game.ball.velocity_x = 0.0;
    game.ball.velocity_y = -200.0;

    game.step(DT, 0);

    assert!(
        game.ball.velocity_y > 0.0,
        "expected ball to bounce downward (positive velocity_y), got {}",
        game.ball.velocity_y
    );
    assert_eq!(
        game.ball.velocity_y, 200.0,
        "expected velocity_y to reflect to 200"
    );
}

#[test]
fn ball_diagonal_hit_on_paddle_corner_resolves_in_one_frame() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;

    let paddle_y = FIELD_HEIGHT / 2.0;
    let paddle_top = paddle_y - PADDLE_HEIGHT / 2.0;
    let paddle_right = PADDLE1_X + PADDLE_WIDTH / 2.0;
    game.paddles[0].position_y = paddle_y;

    game.ball.position_x = 26.0;
    game.ball.position_y = paddle_top - BALL_RADIUS - 0.5;
    game.ball.velocity_x = -200.0;
    game.ball.velocity_y = 200.0;

    game.step(DT, 0);

    assert_eq!(
        game.ball.velocity_y, -200.0,
        "velocity_y should be reflected on edge hit"
    );
    assert_eq!(
        game.ball.velocity_x, -200.0,
        "velocity_x should be unchanged on edge hit"
    );
    assert!(
        game.ball.position_x >= paddle_right + BALL_RADIUS,
        "ball should be pushed outside paddle X extent, got {}",
        game.ball.position_x
    );
    assert!(
        game.ball.position_y <= paddle_top - BALL_RADIUS,
        "ball should be above paddle top edge, got {}",
        game.ball.position_y
    );
}

#[test]
fn scores_appear_in_snapshot() {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game.player_one_score = 7;
    game.player_two_score = 4;
    game.phase = GamePhase::GameOver;

    game.update_snapshot();

    assert_eq!(game.snapshot[6], 7.0);
    assert_eq!(game.snapshot[7], 4.0);
    assert_eq!(game.snapshot[8], FIELD_WIDTH);
    assert_eq!(game.snapshot[9], FIELD_HEIGHT);
    assert_eq!(game.snapshot[10], 1.0);
}
