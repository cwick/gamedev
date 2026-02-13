use rand::Rng;

use crate::engine::ecs::components::{BounceCollider, Spin, Transform, Velocity};
use crate::engine::{INPUT_ACTION, INPUT_DOWN, INPUT_UP};
pub mod resources;
use crate::engine::ecs::schedule::{Schedule, SystemPhase};
use crate::engine::ecs::world::World;
use crate::engine::{Snapshot, TuningApi};
pub use resources::{PongPhase, PongPlayer, PongState};

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SnapshotField {
    BallX = 0,
    BallY = 1,
    BallVx = 2,
    BallVy = 3,
    Paddle1X = 4,
    Paddle1Y = 5,
    Paddle2X = 6,
    Paddle2Y = 7,
    PlayerOneScore = 8,
    PlayerTwoScore = 9,
    FieldWidth = 10,
    FieldHeight = 11,
    GamePhase = 12,
    Winner = 13,
    BallVisible = 14,
    PaddleWidth = 15,
    PaddleHeight = 16,
    Count = 17,
}

impl SnapshotField {
    const fn idx(self) -> usize {
        self as usize
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PongTuningParam {
    BallX = 0,
    BallY = 1,
}

impl TryFrom<u32> for PongTuningParam {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::BallX),
            1 => Ok(Self::BallY),
            _ => Err(()),
        }
    }
}

const PONG_TUNING_SCHEMA_VERSION: u32 = 1;

const PADDLE_SPEED: f32 = 300.0;
const AI_DEAD_ZONE: f32 = 10.0;
const WINNING_SCORE: u32 = 11;
const SERVE_DELAY_MIN: f32 = 1.0;
const SERVE_DELAY_MAX: f32 = 3.0;

const BALL_RADIUS: f32 = 8.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 60.0;
const PADDLE1_X: f32 = 20.0;
const PADDLE2_X: f32 = 770.0;
const MAX_BOUNCE_ANGLE: f32 = std::f32::consts::FRAC_PI_3;
const BALL_SPEED_ACCEL_FACTOR: f32 = 1.08;
const BALL_MAX_SPEED: f32 = 900.0;
const SPIN_TRANSFER_RATE: f32 = 0.1;
const SPIN_DECAY_RATE: f32 = 0.90;
const SPIN_MAX: f32 = 400.0;

fn reset_game(world: &mut World) {
    let width = world.field.width;
    let height = world.field.height;

    {
        let pong = world.resource_mut::<PongState>();
        pong.player_one_score = 0;
        pong.player_two_score = 0;
        pong.phase = PongPhase::Playing;
        pong.winner = None;
        pong.conceded_by = None;
        pong.winning_score = WINNING_SCORE;
        pong.serve_delay_remaining = 0.0;
    }

    let paddle_mid = height / 2.0;
    let paddles = world.resource::<PongState>().paddles;
    for &paddle in &paddles {
        world.transform_mut(paddle).y = paddle_mid;
        world.velocity_mut(paddle).y = 0.0;
    }

    let ball = world.resource::<PongState>().ball;
    world.transform_mut(ball).x = width / 2.0;
    world.transform_mut(ball).y = height / 2.0;
    world.velocity_mut(ball).x = 0.0;
    world.velocity_mut(ball).y = 0.0;
    world.spin_mut(ball).value = 0.0;

    launch_ball(world);
}

fn center_ball(world: &mut World, side: f32) {
    let width = world.field.width;
    let height = world.field.height;
    let pong = world.resource_mut::<PongState>();
    let ball = pong.ball;

    world.transform_mut(ball).x = width / 2.0 + width * 0.1 * side;
    world.transform_mut(ball).y = height / 2.0;
    world.velocity_mut(ball).x = 0.0;
    world.velocity_mut(ball).y = 0.0;
    world.spin_mut(ball).value = 0.0;
}

fn launch_ball(world: &mut World) {
    let mut rng = rand::thread_rng();
    let half_cone = std::f32::consts::FRAC_PI_4;
    let spread = rng.gen_range(-half_cone..half_cone);
    let launch_side = {
        let pong = world.resource_mut::<PongState>();
        match pong.conceded_by.take() {
            Some(PongPlayer::One) => 1.0,
            Some(PongPlayer::Two) => -1.0,
            None => {
                if rng.gen_bool(0.5) {
                    1.0
                } else {
                    -1.0
                }
            }
        }
    };
    let base_angle = if launch_side > 0.0 {
        std::f32::consts::PI
    } else {
        0.0
    };
    let angle = base_angle + spread;
    let speed = rng.gen_range(350.0..500.0);

    center_ball(world, launch_side);

    let ball = world.resource::<PongState>().ball;
    let velocity = world.velocity_mut(ball);
    velocity.x = angle.cos() * speed;
    velocity.y = angle.sin() * speed;
    world.resource_mut::<PongState>().serve_delay_remaining = 0.0;
}

fn tick_serve_delay(world: &mut World, dt: f32) {
    let should_launch = {
        let pong = world.resource_mut::<PongState>();
        if pong.serve_delay_remaining <= 0.0 {
            return;
        }
        pong.serve_delay_remaining = (pong.serve_delay_remaining - dt).max(0.0);
        pong.serve_delay_remaining <= 0.0
    };
    if should_launch {
        launch_ball(world);
    }
}

fn ball_visible(world: &World) -> bool {
    world.resource::<PongState>().ball_visible()
}

fn compute_ai_input(world: &World) -> u32 {
    let pong = world.resource::<PongState>();
    let ball_y = world.transform(pong.ball).y;
    let paddle_y = world.transform(pong.paddles[1]).y;
    let diff = ball_y - paddle_y;

    if diff.abs() < AI_DEAD_ZONE {
        0
    } else if diff > 0.0 {
        INPUT_DOWN
    } else {
        INPUT_UP
    }
}

fn apply_input(world: &mut World, _dt: f32) {
    let phase = world.resource::<PongState>().phase;
    if phase == PongPhase::GameOver {
        return;
    }

    let p1_input = world.input.bits;
    let p2_input = compute_ai_input(world);
    let inputs = [p1_input, p2_input];
    let paddles = world.resource::<PongState>().paddles;

    for (idx, &input_bits) in inputs.iter().enumerate() {
        let up = (input_bits & INPUT_UP) != 0;
        let down = (input_bits & INPUT_DOWN) != 0;
        let dir = (down as i32) - (up as i32);
        let paddle = paddles[idx];
        world.velocity_mut(paddle).y = dir as f32 * PADDLE_SPEED;
    }
}

fn resolve_post_integration(world: &mut World, dt: f32) {
    let phase = world.resource::<PongState>().phase;
    if phase == PongPhase::GameOver {
        return;
    }

    if ball_visible(world) {
        let ball = world.resource::<PongState>().ball;
        let sv = world.spin(ball).value;
        world.velocity_mut(ball).y += sv * dt;
        world.spin_mut(ball).value *= SPIN_DECAY_RATE.powf(dt);
    }

    let paddle_half_height = PADDLE_HEIGHT / 2.0;
    let max_paddle_y = world.field.height - paddle_half_height;
    let paddles = world.resource::<PongState>().paddles;
    for &paddle in &paddles {
        let paddle_transform = world.transform_mut(paddle);
        paddle_transform.y = paddle_transform.y.clamp(paddle_half_height, max_paddle_y);
    }
}

fn check_paddle_collision(ball_x: f32, ball_y: f32, paddle_x: f32, paddle_y: f32) -> bool {
    let paddle_half_h = PADDLE_HEIGHT / 2.0;
    let paddle_half_w = PADDLE_WIDTH / 2.0;

    let closest_x = ball_x.clamp(paddle_x - paddle_half_w, paddle_x + paddle_half_w);
    let closest_y = ball_y.clamp(paddle_y - paddle_half_h, paddle_y + paddle_half_h);

    let dist_x = ball_x - closest_x;
    let dist_y = ball_y - closest_y;
    let dist_squared = dist_x * dist_x + dist_y * dist_y;

    dist_squared < (BALL_RADIUS * BALL_RADIUS)
}

fn collide_paddles(world: &mut World, _dt: f32) {
    let phase = world.resource::<PongState>().phase;
    if phase == PongPhase::GameOver || !ball_visible(world) {
        return;
    }

    let paddles = world.resource::<PongState>().paddles;
    let ball = world.resource::<PongState>().ball;
    let paddle_positions = [PADDLE1_X, PADDLE2_X];

    for (i, &paddle_x) in paddle_positions.iter().enumerate() {
        let paddle_entity = paddles[i];
        let paddle_y = world.transform(paddle_entity).y;

        let ball_transform = world.transform(ball);
        let ball_x = ball_transform.x;
        let ball_y = ball_transform.y;

        if !check_paddle_collision(ball_x, ball_y, paddle_x, paddle_y) {
            continue;
        }

        let ball_velocity = world.velocity(ball);
        let speed =
            (ball_velocity.x.hypot(ball_velocity.y) * BALL_SPEED_ACCEL_FACTOR).min(BALL_MAX_SPEED);
        let paddle_half_h = PADDLE_HEIGHT / 2.0;
        let offset = ((ball_y - paddle_y) / paddle_half_h).clamp(-1.0, 1.0);
        let angle = offset * MAX_BOUNCE_ANGLE;
        let direction = if i == 0 { 1.0_f32 } else { -1.0 };

        let ball_vel_mut = world.velocity_mut(ball);
        ball_vel_mut.x = direction * speed * angle.cos();
        ball_vel_mut.y = speed * angle.sin();

        let paddle_y_vel = world.velocity(paddle_entity).y;
        let spin_transfer = -paddle_y_vel * SPIN_TRANSFER_RATE;
        let ball_spin = world.spin_mut(ball);
        ball_spin.value = (ball_spin.value + spin_transfer).clamp(-SPIN_MAX, SPIN_MAX);

        let paddle_top = paddle_y - paddle_half_h;
        let paddle_bottom = paddle_y + paddle_half_h;
        let is_face_hit = ball_y >= paddle_top && ball_y <= paddle_bottom;

        let ball_transform = world.transform_mut(ball);
        if is_face_hit {
            ball_transform.x = paddle_x + direction * (PADDLE_WIDTH / 2.0 + BALL_RADIUS);
        } else if ball_y < paddle_top {
            ball_transform.y = paddle_top - BALL_RADIUS;
        } else {
            ball_transform.y = paddle_bottom + BALL_RADIUS;
        }
    }
}

fn collide_walls(world: &mut World, _dt: f32) {
    if world.resource::<PongState>().phase == PongPhase::GameOver || !ball_visible(world) {
        return;
    }

    let ball = world.resource::<PongState>().ball;
    let ball_transform = world.transform(ball);
    let ball_x = ball_transform.x;

    if ball_x <= BALL_RADIUS {
        world.resource_mut::<PongState>().conceded_by = Some(PongPlayer::One);
    } else if ball_x >= world.field.width - BALL_RADIUS {
        world.resource_mut::<PongState>().conceded_by = Some(PongPlayer::Two);
    }
}

fn resolve_scoring(world: &mut World, _dt: f32) {
    if !ball_visible(world) {
        return;
    }
    let conceder = match world.resource::<PongState>().conceded_by {
        Some(player) => player,
        None => return,
    };

    let scorer = match conceder {
        PongPlayer::One => PongPlayer::Two,
        PongPlayer::Two => PongPlayer::One,
    };

    let pong = world.resource_mut::<PongState>();
    match scorer {
        PongPlayer::One => pong.player_one_score += 1,
        PongPlayer::Two => pong.player_two_score += 1,
    }

    let score = match scorer {
        PongPlayer::One => pong.player_one_score,
        PongPlayer::Two => pong.player_two_score,
    };

    if score >= pong.winning_score {
        pong.phase = PongPhase::GameOver;
        pong.winner = Some(scorer);
        pong.serve_delay_remaining = 0.0;
    } else {
        let mut rng = rand::thread_rng();
        pong.serve_delay_remaining = rng.gen_range(SERVE_DELAY_MIN..SERVE_DELAY_MAX);
    }

    center_ball(world, 0.0);
}

fn handle_restart(world: &mut World, _dt: f32) {
    if world.resource::<PongState>().phase != PongPhase::GameOver {
        return;
    }
    if (world.input.bits & INPUT_ACTION) != 0 {
        reset_game(world);
    }
}

fn tick_serve(world: &mut World, dt: f32) {
    let phase = world.resource::<PongState>().phase;
    if phase != PongPhase::Playing {
        return;
    }
    if !ball_visible(world) {
        tick_serve_delay(world, dt);
    }
}

fn get_tuning_param(world: &World, param_id: u32) -> Option<f32> {
    let param = PongTuningParam::try_from(param_id).ok()?;
    let pong = world.resource::<PongState>();
    let ball_transform = world.transform(pong.ball);
    let value = match param {
        PongTuningParam::BallX => ball_transform.x,
        PongTuningParam::BallY => ball_transform.y,
    };
    Some(value)
}

pub fn build_world(width: f32, height: f32) -> (World, Schedule, Snapshot, TuningApi) {
    let mut world = World::new(width, height);

    let ball = world.spawn();
    world.set_transform(
        ball,
        Transform {
            x: width / 2.0,
            y: height / 2.0,
        },
    );
    world.set_velocity(ball, Velocity { x: 0.0, y: 0.0 });
    world.set_spin(ball, Spin { value: 0.0 });
    world.set_wall_bounce_collider(
        ball,
        BounceCollider {
            radius: BALL_RADIUS,
        },
    );

    let paddle1 = world.spawn();
    world.set_transform(
        paddle1,
        Transform {
            x: PADDLE1_X,
            y: height / 2.0,
        },
    );
    world.set_velocity(paddle1, Velocity { x: 0.0, y: 0.0 });

    let paddle2 = world.spawn();
    world.set_transform(
        paddle2,
        Transform {
            x: PADDLE2_X,
            y: height / 2.0,
        },
    );
    world.set_velocity(paddle2, Velocity { x: 0.0, y: 0.0 });

    world.insert_resource(PongState {
        ball,
        paddles: [paddle1, paddle2],
        player_one_score: 0,
        player_two_score: 0,
        phase: PongPhase::Playing,
        winner: None,
        conceded_by: None,
        winning_score: WINNING_SCORE,
        serve_delay_remaining: 0.0,
    });

    launch_ball(&mut world);

    let schedule = Schedule::new()
        .with_system_in_phase(SystemPhase::Control, handle_restart)
        .with_system_in_phase(SystemPhase::Control, apply_input)
        .with_system_in_phase(SystemPhase::Resolve, resolve_post_integration)
        .with_system_in_phase(SystemPhase::Resolve, collide_walls)
        .with_system_in_phase(SystemPhase::Resolve, collide_paddles)
        .with_system_in_phase(SystemPhase::Resolve, resolve_scoring)
        .with_system_in_phase(SystemPhase::Resolve, tick_serve);

    fn set_tuning_param(_world: &mut World, _param_id: u32, _value: f32) -> u32 {
        crate::engine::TUNING_STATUS_REJECTED
    }

    fn reset_tuning_defaults(_world: &mut World) {}

    (
        world,
        schedule,
        Snapshot::new(write_snapshot, vec![0.0; SnapshotField::Count as usize]),
        TuningApi::new(
            set_tuning_param,
            get_tuning_param,
            reset_tuning_defaults,
            PONG_TUNING_SCHEMA_VERSION,
        ),
    )
}

fn write_snapshot(world: &World, snapshot: &mut [f32]) {
    use SnapshotField::*;

    let pong = world.resource::<PongState>();
    let ball = pong.ball;
    let ball_visible = pong.ball_visible();

    let ball_transform = world.transform(ball);
    let ball_velocity = world.velocity(ball);

    snapshot[BallX.idx()] = ball_transform.x;
    snapshot[BallY.idx()] = ball_transform.y;
    snapshot[BallVx.idx()] = ball_velocity.x;
    snapshot[BallVy.idx()] = ball_velocity.y;
    snapshot[Paddle1X.idx()] = PADDLE1_X;
    snapshot[Paddle1Y.idx()] = world.transform(pong.paddles[0]).y;
    snapshot[Paddle2X.idx()] = PADDLE2_X;
    snapshot[Paddle2Y.idx()] = world.transform(pong.paddles[1]).y;
    snapshot[PlayerOneScore.idx()] = pong.player_one_score as f32;
    snapshot[PlayerTwoScore.idx()] = pong.player_two_score as f32;
    snapshot[FieldWidth.idx()] = world.field.width;
    snapshot[FieldHeight.idx()] = world.field.height;
    snapshot[GamePhase.idx()] = match pong.phase {
        PongPhase::Playing => 0.0,
        PongPhase::GameOver => 1.0,
    };
    snapshot[Winner.idx()] = match pong.winner {
        Some(PongPlayer::One) => 1.0,
        Some(PongPlayer::Two) => 2.0,
        None => 0.0,
    };
    snapshot[BallVisible.idx()] = if ball_visible { 1.0 } else { 0.0 };
    snapshot[PaddleWidth.idx()] = PADDLE_WIDTH;
    snapshot[PaddleHeight.idx()] = PADDLE_HEIGHT;
}

#[cfg(test)]
mod tests;
