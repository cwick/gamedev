use super::*;
use crate::engine::ecs::schedule::SystemPhase;
use crate::engine::ecs::systems::integrate_velocity;

const DT: f32 = 1.0 / 60.0;
const FIELD_WIDTH: f32 = 800.0;
const FIELD_HEIGHT: f32 = 600.0;

fn new_game() -> (World, Schedule) {
    let (world, schedule, _snapshot) = build_world(FIELD_WIDTH, FIELD_HEIGHT);
    let schedule = schedule.with_system_in_phase(SystemPhase::Physics, integrate_velocity);
    (world, schedule)
}

fn step(world: &mut World, schedule: &Schedule, dt: f32, input_bits: u32) {
    world.input.bits = input_bits;
    schedule.run(world, dt);
}

fn ball_entity(world: &World) -> crate::engine::ecs::entity::EntityId {
    pong_ref(world).ball
}

fn paddle_entity(world: &World, index: usize) -> crate::engine::ecs::entity::EntityId {
    pong_ref(world).paddles[index]
}

fn pong_ref(world: &World) -> &PongState {
    world.resource::<PongState>()
}

fn pong_mut(world: &mut World) -> &mut PongState {
    world.resource_mut::<PongState>()
}

mod wall_collisions {
    use super::*;

    #[test]
    fn bounces_off_top_wall() {
        let (mut world, schedule) = new_game();
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = FIELD_WIDTH / 2.0;
        world.transform_mut(ball).y = 5.0;
        world.velocity_mut(ball).x = 0.0;
        world.velocity_mut(ball).y = -300.0;

        let velocity_before = world.velocity(ball).y;
        step(&mut world, &schedule, DT, 0);

        let velocity_after = world.velocity(ball).y;
        assert!(
            velocity_after > 0.0,
            "ball should bounce down after hitting top wall"
        );
        assert_eq!(
            velocity_after, -velocity_before,
            "ball should reverse vertical direction"
        );
    }

    #[test]
    fn bounces_off_bottom_wall() {
        let (mut world, schedule) = new_game();
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = FIELD_WIDTH / 2.0;
        world.transform_mut(ball).y = FIELD_HEIGHT - 5.0;
        world.velocity_mut(ball).x = 0.0;
        world.velocity_mut(ball).y = 300.0;

        let velocity_before = world.velocity(ball).y;
        step(&mut world, &schedule, DT, 0);

        let velocity_after = world.velocity(ball).y;
        assert!(
            velocity_after < 0.0,
            "ball should bounce up after hitting bottom wall"
        );
        assert_eq!(
            velocity_after, -velocity_before,
            "ball should reverse vertical direction"
        );
    }
}

mod paddle_collisions {
    use super::*;

    #[test]
    fn bounces_off_left_paddle() {
        let (mut world, schedule) = new_game();
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = PADDLE1_X + PADDLE_WIDTH / 2.0 + BALL_RADIUS + 2.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = -200.0;
        world.velocity_mut(ball).y = 0.0;
        let paddle = paddle_entity(&world, 0);
        world.transform_mut(paddle).y = FIELD_HEIGHT / 2.0;

        step(&mut world, &schedule, DT, 0);

        let vx = world.velocity(ball).x;
        assert!(
            vx > 0.0,
            "ball should bounce right after hitting left paddle"
        );
    }

    #[test]
    fn bounces_off_right_paddle() {
        let (mut world, schedule) = new_game();
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = PADDLE2_X - PADDLE_WIDTH / 2.0 - BALL_RADIUS - 2.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = 200.0;
        world.velocity_mut(ball).y = 0.0;
        let paddle = paddle_entity(&world, 1);
        world.transform_mut(paddle).y = FIELD_HEIGHT / 2.0;

        step(&mut world, &schedule, DT, 0);

        let vx = world.velocity(ball).x;
        assert!(
            vx < 0.0,
            "ball should bounce left after hitting right paddle"
        );
    }

    #[test]
    fn center_hit_produces_horizontal_bounce() {
        let (mut world, schedule) = new_game();
        let paddle = paddle_entity(&world, 0);
        world.transform_mut(paddle).y = FIELD_HEIGHT / 2.0;
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = PADDLE1_X + PADDLE_WIDTH / 2.0 + BALL_RADIUS + 2.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = -300.0;
        world.velocity_mut(ball).y = 0.0;

        step(&mut world, &schedule, DT, 0);

        let velocity = world.velocity(ball);
        assert!(velocity.x > 0.0, "ball should bounce right");
        assert!(
            velocity.y.abs() < 0.01,
            "center hit should produce near-zero vy, got {}",
            velocity.y
        );
    }

    #[test]
    fn edge_hit_top_bounces_upward_at_max_angle() {
        let (mut world, schedule) = new_game();
        let paddle_y = FIELD_HEIGHT / 2.0;
        let paddle_top = paddle_y - PADDLE_HEIGHT / 2.0;
        let paddle = paddle_entity(&world, 0);
        world.transform_mut(paddle).y = paddle_y;

        let ball = ball_entity(&world);
        world.transform_mut(ball).x = PADDLE1_X;
        world.transform_mut(ball).y = paddle_top - BALL_RADIUS - 1.0;
        world.velocity_mut(ball).x = 0.0;
        world.velocity_mut(ball).y = 200.0;
        let speed_before = 200.0_f32;

        step(&mut world, &schedule, DT, 0);

        let velocity = world.velocity(ball);
        let speed_after = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt();
        assert!(
            velocity.y < 0.0,
            "ball should bounce upward, got vy={}",
            velocity.y
        );
        assert!(
            velocity.x > 0.0,
            "ball should bounce right (away from left paddle), got vx={}",
            velocity.x
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
        let (mut world, schedule) = new_game();
        let paddle_y = FIELD_HEIGHT / 2.0;
        let paddle_bottom = paddle_y + PADDLE_HEIGHT / 2.0;
        let paddle = paddle_entity(&world, 0);
        world.transform_mut(paddle).y = paddle_y;

        let ball = ball_entity(&world);
        world.transform_mut(ball).x = PADDLE1_X;
        world.transform_mut(ball).y = paddle_bottom + BALL_RADIUS + 1.0;
        world.velocity_mut(ball).x = 0.0;
        world.velocity_mut(ball).y = -200.0;
        let speed_before = 200.0_f32;

        step(&mut world, &schedule, DT, 0);

        let velocity = world.velocity(ball);
        let speed_after = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt();
        assert!(
            velocity.y > 0.0,
            "ball should bounce downward, got vy={}",
            velocity.y
        );
        assert!(
            velocity.x > 0.0,
            "ball should bounce right (away from left paddle), got vx={}",
            velocity.x
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
        let (mut world, schedule) = new_game();
        let paddle_y = FIELD_HEIGHT / 2.0;
        let paddle = paddle_entity(&world, 0);
        world.transform_mut(paddle).y = paddle_y;

        let ball = ball_entity(&world);
        world.transform_mut(ball).x = 26.0;
        world.transform_mut(ball).y = paddle_y - PADDLE_HEIGHT / 2.0 - BALL_RADIUS - 0.5;
        world.velocity_mut(ball).x = -200.0;
        world.velocity_mut(ball).y = 200.0;
        let speed_before = (200.0_f32.powi(2) + 200.0_f32.powi(2)).sqrt();

        step(&mut world, &schedule, DT, 0);

        let velocity = world.velocity(ball);
        let speed_after = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt();
        assert!(velocity.x > 0.0, "ball should bounce right");
        assert!(velocity.y < 0.0, "ball should bounce upward from top edge");
        assert!(
            speed_after >= speed_before,
            "speed should not decrease after paddle hit"
        );
        let paddle_top = paddle_y - PADDLE_HEIGHT / 2.0;
        let ball_y = world.transform(ball).y;
        assert!(
            ball_y <= paddle_top - BALL_RADIUS,
            "ball should be pushed clear of paddle top edge, got {}",
            ball_y
        );
    }

    #[test]
    fn paddle_cannot_move_above_top_edge() {
        let (mut world, schedule) = new_game();
        let paddle = paddle_entity(&world, 0);
        world.transform_mut(paddle).y = PADDLE_HEIGHT / 2.0 + 10.0;

        step(&mut world, &schedule, DT, INPUT_UP);

        let y = world.transform(paddle).y;
        assert!(
            y >= PADDLE_HEIGHT / 2.0,
            "paddle should not move above top edge"
        );
    }

    #[test]
    fn paddle_cannot_move_below_bottom_edge() {
        let (mut world, schedule) = new_game();
        let paddle = paddle_entity(&world, 0);
        world.transform_mut(paddle).y = FIELD_HEIGHT - PADDLE_HEIGHT / 2.0 - 10.0;

        step(&mut world, &schedule, DT, INPUT_DOWN);

        let y = world.transform(paddle).y;
        assert!(
            y <= FIELD_HEIGHT - PADDLE_HEIGHT / 2.0,
            "paddle should not move below bottom edge"
        );
    }
}

mod scoring {
    use super::*;

    fn wait_for_serve(world: &mut World, schedule: &Schedule) {
        let mut ticks = 0;
        while !ball_visible(world) {
            step(world, schedule, DT, 0);
            ticks += 1;
            assert!(ticks < 300, "serve never launched");
        }
    }

    #[test]
    fn ball_exiting_left_scores_for_player_two() {
        let (mut world, schedule) = new_game();
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = 5.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = -400.0;
        world.velocity_mut(ball).y = 0.0;

        step(&mut world, &schedule, DT, 0);

        let pong = pong_ref(&world);
        assert_eq!(pong.player_two_score, 1);
        assert_eq!(pong.player_one_score, 0);
    }

    #[test]
    fn ball_exiting_right_scores_for_player_one() {
        let (mut world, schedule) = new_game();
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = FIELD_WIDTH - 5.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = 400.0;
        world.velocity_mut(ball).y = 0.0;

        step(&mut world, &schedule, DT, 0);

        let pong = pong_ref(&world);
        assert_eq!(pong.player_one_score, 1);
        assert_eq!(pong.player_two_score, 0);
    }

    #[test]
    fn scores_accumulate_across_rounds() {
        let (mut world, schedule) = new_game();
        let ball = ball_entity(&world);

        for round in 1..=4 {
            world.transform_mut(ball).x = FIELD_WIDTH - 5.0;
            world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
            world.velocity_mut(ball).x = 400.0;
            world.velocity_mut(ball).y = 0.0;
            step(&mut world, &schedule, DT, 0);
            assert_eq!(pong_ref(&world).player_one_score, round);
            wait_for_serve(&mut world, &schedule);
        }

        for round in 1..=3 {
            world.transform_mut(ball).x = 5.0;
            world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
            world.velocity_mut(ball).x = -400.0;
            world.velocity_mut(ball).y = 0.0;
            step(&mut world, &schedule, DT, 0);
            assert_eq!(pong_ref(&world).player_two_score, round);
            wait_for_serve(&mut world, &schedule);
        }

        assert_eq!(pong_ref(&world).player_one_score, 4);
        assert_eq!(pong_ref(&world).player_two_score, 3);
    }

    #[test]
    fn game_ends_at_winning_score() {
        let (mut world, schedule) = new_game();
        pong_mut(&mut world).winning_score = 3;
        pong_mut(&mut world).player_one_score = 2;
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = FIELD_WIDTH - 5.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = 400.0;
        world.velocity_mut(ball).y = 0.0;

        step(&mut world, &schedule, DT, 0);

        let pong = pong_ref(&world);
        assert_eq!(pong.player_one_score, 3);
        assert_eq!(pong.phase, PongPhase::GameOver);

        let ball_x_before = world.transform(ball).x;
        step(&mut world, &schedule, DT, 0);
        let ball_x_after = world.transform(ball).x;
        assert_eq!(
            ball_x_after, ball_x_before,
            "step should be a no-op after game over"
        );
    }

    #[test]
    fn ball_stays_hidden_during_game_over() {
        let (mut world, schedule) = new_game();
        pong_mut(&mut world).winning_score = 2;
        pong_mut(&mut world).player_one_score = 1;
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = FIELD_WIDTH - 5.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = 400.0;
        world.velocity_mut(ball).y = 0.0;

        step(&mut world, &schedule, DT, 0);

        let pong = pong_ref(&world);
        assert_eq!(pong.phase, PongPhase::GameOver);
        assert!(
            !ball_visible(&world),
            "ball should stay hidden once the game is over"
        );
    }

    #[test]
    fn action_input_restarts_game_when_over() {
        let (mut world, schedule) = new_game();
        pong_mut(&mut world).phase = PongPhase::GameOver;
        pong_mut(&mut world).winner = Some(PongPlayer::One);
        pong_mut(&mut world).player_one_score = 11;

        step(&mut world, &schedule, DT, INPUT_ACTION);

        let pong = pong_ref(&world);
        assert_eq!(pong.phase, PongPhase::Playing);
        assert_eq!(pong.winner, None);
        assert_eq!(pong.player_one_score, 0);
        assert_eq!(pong.player_two_score, 0);
    }

    #[test]
    fn action_input_ignored_during_play() {
        let (mut world, schedule) = new_game();
        pong_mut(&mut world).player_one_score = 5;

        step(&mut world, &schedule, DT, INPUT_ACTION);

        let pong = pong_ref(&world);
        assert_eq!(pong.phase, PongPhase::Playing);
        assert_eq!(pong.player_one_score, 5);
    }

    #[test]
    fn serve_delay_hides_ball_after_score() {
        let (mut world, schedule) = new_game();
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = 5.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = -400.0;
        world.velocity_mut(ball).y = 0.0;

        step(&mut world, &schedule, DT, 0);

        assert!(
            !ball_visible(&world),
            "ball should go invisible once a score happens"
        );
        let delay = pong_ref(&world).serve_delay_remaining;
        assert!(
            (SERVE_DELAY_MIN..SERVE_DELAY_MAX).contains(&delay),
            "delay should be between {} and {} but was {}",
            SERVE_DELAY_MIN,
            SERVE_DELAY_MAX,
            delay
        );

        let prev_delay = pong_ref(&world).serve_delay_remaining;
        step(&mut world, &schedule, DT, 0);
        assert!(!ball_visible(&world));
        assert!(pong_ref(&world).serve_delay_remaining < prev_delay);

        let mut iterations = 0;
        while !ball_visible(&world) && iterations < 200 {
            step(&mut world, &schedule, DT, 0);
            iterations += 1;
        }
        assert!(ball_visible(&world), "serve should eventually launch");
        assert_eq!(pong_ref(&world).serve_delay_remaining, 0.0);
        let velocity = world.velocity(ball);
        let speed = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt();
        assert!(
            speed > 0.0,
            "ball should have non-zero velocity after launch"
        );
        assert!(
            pong_ref(&world).conceded_by.is_none(),
            "conceded_by should be cleared on launch"
        );
    }
}

mod snapshot {
    use super::*;

    #[test]
    fn scores_appear_in_snapshot() {
        let (mut world, _schedule) = new_game();
        pong_mut(&mut world).player_one_score = 7;
        pong_mut(&mut world).player_two_score = 4;
        pong_mut(&mut world).phase = PongPhase::GameOver;
        pong_mut(&mut world).winner = Some(PongPlayer::One);

        let mut snapshot = vec![0.0; SnapshotField::Count as usize];
        write_snapshot(&world, &mut snapshot);

        assert_eq!(snapshot[SnapshotField::PlayerOneScore.idx()], 7.0);
        assert_eq!(snapshot[SnapshotField::PlayerTwoScore.idx()], 4.0);
        assert_eq!(snapshot[SnapshotField::FieldWidth.idx()], FIELD_WIDTH);
        assert_eq!(snapshot[SnapshotField::FieldHeight.idx()], FIELD_HEIGHT);
        assert_eq!(snapshot[SnapshotField::GamePhase.idx()], 1.0);
        assert_eq!(
            snapshot[SnapshotField::Winner.idx()],
            1.0,
            "player 1 should be winner"
        );
        assert_eq!(
            snapshot[SnapshotField::BallVisible.idx()],
            0.0,
            "ball should stay hidden when game over"
        );

        pong_mut(&mut world).winner = Some(PongPlayer::Two);
        write_snapshot(&world, &mut snapshot);
        assert_eq!(
            snapshot[SnapshotField::Winner.idx()],
            2.0,
            "player 2 should be winner"
        );
    }
}

mod speed_progression {
    use super::*;

    #[test]
    fn speed_increases_after_paddle_hit() {
        let (mut world, schedule) = new_game();
        let paddle = paddle_entity(&world, 0);
        world.transform_mut(paddle).y = FIELD_HEIGHT / 2.0;
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = PADDLE1_X + PADDLE_WIDTH / 2.0 + BALL_RADIUS + 2.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = -300.0;
        world.velocity_mut(ball).y = 0.0;
        let speed_before = world.velocity(ball).x.hypot(world.velocity(ball).y);

        step(&mut world, &schedule, DT, 0);

        let velocity = world.velocity(ball);
        let speed_after = velocity.x.hypot(velocity.y);
        assert!(
            speed_after > speed_before,
            "speed should increase after paddle hit: before={speed_before}, after={speed_after}"
        );
    }

    #[test]
    fn speed_does_not_exceed_cap() {
        let (mut world, schedule) = new_game();
        let paddle = paddle_entity(&world, 0);
        world.transform_mut(paddle).y = FIELD_HEIGHT / 2.0;
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = PADDLE1_X + PADDLE_WIDTH / 2.0 + BALL_RADIUS + 2.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = -BALL_MAX_SPEED;
        world.velocity_mut(ball).y = 0.0;

        step(&mut world, &schedule, DT, 0);

        let velocity = world.velocity(ball);
        let speed_after = velocity.x.hypot(velocity.y);
        assert!(
            speed_after <= BALL_MAX_SPEED,
            "speed must not exceed cap: got {speed_after}"
        );
    }

    #[test]
    fn speed_resets_on_new_serve() {
        let (mut world, schedule) = new_game();
        let ball = ball_entity(&world);
        world.transform_mut(ball).x = FIELD_WIDTH - 5.0;
        world.transform_mut(ball).y = FIELD_HEIGHT / 2.0;
        world.velocity_mut(ball).x = BALL_MAX_SPEED;
        world.velocity_mut(ball).y = 0.0;

        step(&mut world, &schedule, DT, 0);
        assert!(!ball_visible(&world));

        let mut ticks = 0;
        while !ball_visible(&world) {
            step(&mut world, &schedule, DT, 0);
            ticks += 1;
            assert!(ticks < 300, "serve never launched");
        }

        let velocity = world.velocity(ball);
        let speed_after_serve = velocity.x.hypot(velocity.y);
        assert!(
            speed_after_serve <= 500.0,
            "speed should reset to serve range after a point: got {speed_after_serve}"
        );
    }
}
