use super::*;

const DT: f32 = 1.0 / 60.0;
const FIELD_WIDTH: f32 = 800.0;
const FIELD_HEIGHT: f32 = 600.0;

fn new_game() -> GameState {
    let mut game = GameState::new();
    game.field_width = FIELD_WIDTH;
    game.field_height = FIELD_HEIGHT;
    game
}

mod wall_collisions {
    use super::*;

    #[test]
    fn bounces_off_top_wall() {
        let mut game = new_game();
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
    fn bounces_off_bottom_wall() {
        let mut game = new_game();
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
}

mod paddle_collisions {
    use super::*;

    #[test]
    fn bounces_off_left_paddle() {
        let mut game = new_game();
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
    fn bounces_off_right_paddle() {
        let mut game = new_game();
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
    fn center_hit_produces_horizontal_bounce() {
        let mut game = new_game();
        game.paddles[0].position_y = FIELD_HEIGHT / 2.0;
        game.ball.position_x = PADDLE1_X + PADDLE_WIDTH / 2.0 + BALL_RADIUS + 2.0;
        game.ball.position_y = FIELD_HEIGHT / 2.0;
        game.ball.velocity_x = -300.0;
        game.ball.velocity_y = 0.0;

        game.step(DT, 0);

        assert!(game.ball.velocity_x > 0.0, "ball should bounce right");
        assert!(
            game.ball.velocity_y.abs() < 0.01,
            "center hit should produce near-zero vy, got {}",
            game.ball.velocity_y
        );
    }

    #[test]
    fn edge_hit_top_bounces_upward_at_max_angle() {
        let mut game = new_game();
        let paddle_y = FIELD_HEIGHT / 2.0;
        let paddle_top = paddle_y - PADDLE_HEIGHT / 2.0;
        game.paddles[0].position_y = paddle_y;

        game.ball.position_x = PADDLE1_X;
        game.ball.position_y = paddle_top - BALL_RADIUS - 1.0;
        game.ball.velocity_x = 0.0;
        game.ball.velocity_y = 200.0;
        let speed_before = 200.0_f32;

        game.step(DT, 0);

        let speed_after = (game.ball.velocity_x.powi(2) + game.ball.velocity_y.powi(2)).sqrt();
        assert!(
            game.ball.velocity_y < 0.0,
            "ball should bounce upward, got vy={}",
            game.ball.velocity_y
        );
        assert!(
            game.ball.velocity_x > 0.0,
            "ball should bounce right (away from left paddle), got vx={}",
            game.ball.velocity_x
        );
        assert!(
            (speed_after - speed_before).abs() < 0.01,
            "speed should be preserved: before={}, after={}",
            speed_before,
            speed_after
        );
    }

    #[test]
    fn edge_hit_bottom_bounces_downward_at_max_angle() {
        let mut game = new_game();
        let paddle_y = FIELD_HEIGHT / 2.0;
        let paddle_bottom = paddle_y + PADDLE_HEIGHT / 2.0;
        game.paddles[0].position_y = paddle_y;

        game.ball.position_x = PADDLE1_X;
        game.ball.position_y = paddle_bottom + BALL_RADIUS + 1.0;
        game.ball.velocity_x = 0.0;
        game.ball.velocity_y = -200.0;
        let speed_before = 200.0_f32;

        game.step(DT, 0);

        let speed_after = (game.ball.velocity_x.powi(2) + game.ball.velocity_y.powi(2)).sqrt();
        assert!(
            game.ball.velocity_y > 0.0,
            "ball should bounce downward, got vy={}",
            game.ball.velocity_y
        );
        assert!(
            game.ball.velocity_x > 0.0,
            "ball should bounce right (away from left paddle), got vx={}",
            game.ball.velocity_x
        );
        assert!(
            (speed_after - speed_before).abs() < 0.01,
            "speed should be preserved: before={}, after={}",
            speed_before,
            speed_after
        );
    }

    #[test]
    fn corner_hit_resolves_cleanly() {
        let mut game = new_game();
        let paddle_y = FIELD_HEIGHT / 2.0;
        game.paddles[0].position_y = paddle_y;

        game.ball.position_x = 26.0;
        game.ball.position_y = paddle_y - PADDLE_HEIGHT / 2.0 - BALL_RADIUS - 0.5;
        game.ball.velocity_x = -200.0;
        game.ball.velocity_y = 200.0;
        let speed_before = (200.0_f32.powi(2) + 200.0_f32.powi(2)).sqrt();

        game.step(DT, 0);

        let speed_after = (game.ball.velocity_x.powi(2) + game.ball.velocity_y.powi(2)).sqrt();
        assert!(game.ball.velocity_x > 0.0, "ball should bounce right");
        assert!(
            game.ball.velocity_y < 0.0,
            "ball should bounce upward from top edge"
        );
        assert!(
            (speed_after - speed_before).abs() < 0.01,
            "speed should be preserved"
        );
        let paddle_top = paddle_y - PADDLE_HEIGHT / 2.0;
        assert!(
            game.ball.position_y <= paddle_top - BALL_RADIUS,
            "ball should be pushed clear of paddle top edge, got {}",
            game.ball.position_y
        );
    }

    #[test]
    fn paddle_cannot_move_above_top_edge() {
        let mut game = new_game();
        game.paddles[0].position_y = PADDLE_HEIGHT / 2.0 + 10.0;

        game.step(DT, INPUT_UP);

        assert!(
            game.paddles[0].position_y >= PADDLE_HEIGHT / 2.0,
            "paddle should not move above top edge"
        );
    }

    #[test]
    fn paddle_cannot_move_below_bottom_edge() {
        let mut game = new_game();
        game.paddles[0].position_y = FIELD_HEIGHT - PADDLE_HEIGHT / 2.0 - 10.0;

        game.step(DT, INPUT_DOWN);

        assert!(
            game.paddles[0].position_y <= FIELD_HEIGHT - PADDLE_HEIGHT / 2.0,
            "paddle should not move below bottom edge"
        );
    }
}

mod scoring {
    use super::*;

    #[test]
    fn ball_exiting_left_scores_for_player_two() {
        let mut game = new_game();
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
        let mut game = new_game();
        game.ball.position_x = FIELD_WIDTH - 5.0;
        game.ball.position_y = FIELD_HEIGHT / 2.0;
        game.ball.velocity_x = 400.0;
        game.ball.velocity_y = 0.0;

        game.step(DT, 0);

        assert_eq!(game.player_one_score, 1);
        assert_eq!(game.player_two_score, 0);
    }

    #[test]
    fn scores_accumulate_across_rounds() {
        let mut game = new_game();

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
    fn game_ends_at_winning_score() {
        let mut game = new_game();
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
    fn action_input_restarts_game_when_over() {
        let mut game = new_game();
        game.init(FIELD_WIDTH, FIELD_HEIGHT);
        game.phase = GamePhase::GameOver;
        game.winner = Some(Player::One);
        game.player_one_score = 11;

        game.step(DT, INPUT_ACTION);

        assert_eq!(game.phase, GamePhase::Playing);
        assert_eq!(game.winner, None);
        assert_eq!(game.player_one_score, 0);
        assert_eq!(game.player_two_score, 0);
    }

    #[test]
    fn action_input_ignored_during_play() {
        let mut game = new_game();
        game.init(FIELD_WIDTH, FIELD_HEIGHT);
        game.player_one_score = 5;

        game.step(DT, INPUT_ACTION);

        assert_eq!(game.phase, GamePhase::Playing);
        assert_eq!(game.player_one_score, 5);
    }
}

mod snapshot {
    use super::*;

    #[test]
    fn init_resets_scores_and_phase() {
        let mut game = new_game();
        game.player_one_score = 5;
        game.player_two_score = 3;
        game.phase = GamePhase::GameOver;

        game.init(FIELD_WIDTH, FIELD_HEIGHT);

        assert_eq!(game.player_one_score, 0);
        assert_eq!(game.player_two_score, 0);
        assert_eq!(game.phase, GamePhase::Playing);
    }

    #[test]
    fn scores_appear_in_snapshot() {
        let mut game = new_game();
        game.player_one_score = 7;
        game.player_two_score = 4;
        game.phase = GamePhase::GameOver;
        game.winner = Some(Player::One);

        game.update_snapshot();

        assert_eq!(game.snapshot[8], 7.0);
        assert_eq!(game.snapshot[9], 4.0);
        assert_eq!(game.snapshot[10], FIELD_WIDTH);
        assert_eq!(game.snapshot[11], FIELD_HEIGHT);
        assert_eq!(game.snapshot[12], 1.0);
        assert_eq!(game.snapshot[13], 1.0, "player 1 should be winner");
    }

    #[test]
    fn winner_field_in_snapshot() {
        let mut game = new_game();

        game.update_snapshot();
        assert_eq!(game.snapshot[13], 0.0, "no winner during play");

        game.phase = GamePhase::GameOver;
        game.winner = Some(Player::One);
        game.update_snapshot();
        assert_eq!(game.snapshot[13], 1.0, "player 1 wins");

        game.winner = Some(Player::Two);
        game.update_snapshot();
        assert_eq!(game.snapshot[13], 2.0, "player 2 wins");
    }
}
