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
            speed_after >= speed_before,
            "speed should not decrease after paddle hit: before={}, after={}",
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
            speed_after >= speed_before,
            "speed should not decrease after paddle hit: before={}, after={}",
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
            speed_after >= speed_before,
            "speed should not decrease after paddle hit"
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

    fn wait_for_serve(game: &mut GameState) {
        let mut ticks = 0;
        while !game.ball_visible() {
            game.step(DT, 0);
            ticks += 1;
            assert!(ticks < 300, "serve never launched");
        }
    }

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
            wait_for_serve(&mut game);
        }

        for round in 1..=3 {
            game.ball.position_x = 5.0;
            game.ball.position_y = FIELD_HEIGHT / 2.0;
            game.ball.velocity_x = -400.0;
            game.ball.velocity_y = 0.0;
            game.step(DT, 0);
            assert_eq!(game.player_two_score, round);
            wait_for_serve(&mut game);
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

    #[test]
    fn serve_delay_hides_ball_after_score() {
        let mut game = new_game();
        game.ball.position_x = 5.0;
        game.ball.position_y = FIELD_HEIGHT / 2.0;
        game.ball.velocity_x = -400.0;
        game.ball.velocity_y = 0.0;

        game.step(DT, 0);

        assert!(
            !game.ball_visible(),
            "ball should go invisible once a score happens"
        );
        assert!(
            game.serve_delay_remaining >= SERVE_DELAY_MIN
                && game.serve_delay_remaining < SERVE_DELAY_MAX,
            "delay should be between {} and {} but was {}",
            SERVE_DELAY_MIN,
            SERVE_DELAY_MAX,
            game.serve_delay_remaining
        );

        let prev_delay = game.serve_delay_remaining;
        game.step(DT, 0);
        assert!(!game.ball_visible());
        assert!(game.serve_delay_remaining < prev_delay);

        let mut iterations = 0;
        while !game.ball_visible() && iterations < 200 {
            game.step(DT, 0);
            iterations += 1;
        }
        assert!(game.ball_visible(), "serve should eventually launch");
        assert_eq!(game.serve_delay_remaining, 0.0);
        let speed = (game.ball.velocity_x.powi(2) + game.ball.velocity_y.powi(2)).sqrt();
        assert!(
            speed > 0.0,
            "ball should have non-zero velocity after launch"
        );
        assert!(
            game.conceded_by.is_none(),
            "conceded_by should be cleared on launch"
        );
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

        assert_eq!(game.snapshot[SnapshotField::PlayerOneScore.idx()], 7.0);
        assert_eq!(game.snapshot[SnapshotField::PlayerTwoScore.idx()], 4.0);
        assert_eq!(game.snapshot[SnapshotField::FieldWidth.idx()], FIELD_WIDTH);
        assert_eq!(
            game.snapshot[SnapshotField::FieldHeight.idx()],
            FIELD_HEIGHT
        );
        assert_eq!(game.snapshot[SnapshotField::GamePhase.idx()], 1.0);
        assert_eq!(
            game.snapshot[SnapshotField::Winner.idx()],
            1.0,
            "player 1 should be winner"
        );
        assert_eq!(
            game.snapshot[SnapshotField::BallVisible.idx()],
            1.0,
            "ball should remain visible when game over"
        );
    }

    #[test]
    fn winner_field_in_snapshot() {
        let mut game = new_game();

        game.update_snapshot();
        assert_eq!(
            game.snapshot[SnapshotField::Winner.idx()],
            0.0,
            "no winner during play"
        );

        game.phase = GamePhase::GameOver;
        game.winner = Some(Player::One);
        game.update_snapshot();
        assert_eq!(
            game.snapshot[SnapshotField::Winner.idx()],
            1.0,
            "player 1 wins"
        );

        game.winner = Some(Player::Two);
        game.update_snapshot();
        assert_eq!(
            game.snapshot[SnapshotField::Winner.idx()],
            2.0,
            "player 2 wins"
        );
    }
}

mod speed_progression {
    use super::*;

    #[test]
    fn speed_increases_after_paddle_hit() {
        let mut game = new_game();
        game.paddles[0].position_y = FIELD_HEIGHT / 2.0;
        game.ball.position_x = PADDLE1_X + PADDLE_WIDTH / 2.0 + BALL_RADIUS + 2.0;
        game.ball.position_y = FIELD_HEIGHT / 2.0;
        game.ball.velocity_x = -300.0;
        game.ball.velocity_y = 0.0;
        let speed_before = game.ball.velocity_x.hypot(game.ball.velocity_y);

        game.step(DT, 0);

        let speed_after = game.ball.velocity_x.hypot(game.ball.velocity_y);
        assert!(
            speed_after > speed_before,
            "speed should increase after paddle hit: before={speed_before}, after={speed_after}"
        );
    }

    #[test]
    fn speed_does_not_exceed_cap() {
        let mut game = new_game();
        game.paddles[0].position_y = FIELD_HEIGHT / 2.0;
        game.ball.position_x = PADDLE1_X + PADDLE_WIDTH / 2.0 + BALL_RADIUS + 2.0;
        game.ball.position_y = FIELD_HEIGHT / 2.0;
        game.ball.velocity_x = -BALL_MAX_SPEED;
        game.ball.velocity_y = 0.0;

        game.step(DT, 0);

        let speed_after = game.ball.velocity_x.hypot(game.ball.velocity_y);
        assert!(
            speed_after <= BALL_MAX_SPEED,
            "speed must not exceed cap: got {speed_after}"
        );
    }

    #[test]
    fn speed_resets_on_new_serve() {
        let mut game = new_game();
        game.ball.position_x = FIELD_WIDTH - 5.0;
        game.ball.position_y = FIELD_HEIGHT / 2.0;
        game.ball.velocity_x = BALL_MAX_SPEED;
        game.ball.velocity_y = 0.0;

        game.step(DT, 0);
        assert!(!game.ball_visible());

        let mut ticks = 0;
        while !game.ball_visible() {
            game.step(DT, 0);
            ticks += 1;
            assert!(ticks < 300, "serve never launched");
        }

        let speed_after_serve = game.ball.velocity_x.hypot(game.ball.velocity_y);
        assert!(
            speed_after_serve <= 500.0,
            "speed should reset to serve range after a point: got {speed_after_serve}"
        );
    }
}
