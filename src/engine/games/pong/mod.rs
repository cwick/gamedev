use rand::Rng;

use crate::engine::ecs::components::{Spin, Transform, Velocity};
use crate::engine::{clamp_dt, INPUT_ACTION, INPUT_DOWN, INPUT_UP};
pub mod resources;
use crate::engine::ecs::schedule::Schedule;
use crate::engine::ecs::world::World;
use crate::engine::Snapshot;
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

const PADDLE_SPEED: f32 = 300.0;
const AI_DEAD_ZONE: f32 = 10.0;
const DEFAULT_WINNING_SCORE: u32 = 11;
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

fn pong_mut(world: &mut World) -> &mut PongState {
    world
        .get_resource_mut::<PongState>()
        .expect("pong systems require PongState in World resources")
}

fn pong_ref(world: &World) -> &PongState {
    world
        .get_resource::<PongState>()
        .expect("pong systems require PongState in World resources")
}

fn reset_game(world: &mut World) {
    let width = world.field.width;
    let height = world.field.height;

    {
        let pong = pong_mut(world);
        pong.player_one_score = 0;
        pong.player_two_score = 0;
        pong.phase = PongPhase::Playing;
        pong.winner = None;
        pong.conceded_by = None;
        pong.winning_score = DEFAULT_WINNING_SCORE;
        pong.serve_delay_remaining = 0.0;
    }

    let paddle_mid = height / 2.0;
    let paddles = pong_ref(world).paddles;
    for &paddle in &paddles {
        if let Some(transform) = world.transforms[paddle.0 as usize].as_mut() {
            transform.y = paddle_mid;
        }
        if let Some(velocity) = world.velocities[paddle.0 as usize].as_mut() {
            velocity.y = 0.0;
        }
    }

    let ball = pong_ref(world).ball;
    if let Some(transform) = world.transforms[ball.0 as usize].as_mut() {
        transform.x = width / 2.0;
        transform.y = height / 2.0;
    }
    if let Some(velocity) = world.velocities[ball.0 as usize].as_mut() {
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
    if let Some(spin) = world.spins[ball.0 as usize].as_mut() {
        spin.value = 0.0;
    }

    launch_ball(world);
}

fn center_ball(world: &mut World, side: f32) {
    let width = world.field.width;
    let height = world.field.height;
    let pong = pong_mut(world);
    let ball = pong.ball;

    if let Some(transform) = world.transforms[ball.0 as usize].as_mut() {
        transform.x = width / 2.0 + width * 0.1 * side;
        transform.y = height / 2.0;
    }
    if let Some(velocity) = world.velocities[ball.0 as usize].as_mut() {
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
    if let Some(spin) = world.spins[ball.0 as usize].as_mut() {
        spin.value = 0.0;
    }
}

fn launch_ball(world: &mut World) {
    let mut rng = rand::thread_rng();
    let half_cone = std::f32::consts::FRAC_PI_4;
    let spread = rng.gen_range(-half_cone..half_cone);
    let launch_side = {
        let pong = pong_mut(world);
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

    let ball = pong_ref(world).ball;
    if let Some(velocity) = world.velocities[ball.0 as usize].as_mut() {
        velocity.x = angle.cos() * speed;
        velocity.y = angle.sin() * speed;
    }
    pong_mut(world).serve_delay_remaining = 0.0;
}

fn tick_serve_delay(world: &mut World, dt: f32) {
    let should_launch = {
        let pong = pong_mut(world);
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
    pong_ref(world).ball_visible()
}

fn compute_ai_input(world: &World) -> u32 {
    let pong = pong_ref(world);
    let ball_y = world.transforms[pong.ball.0 as usize]
        .as_ref()
        .map(|t| t.y)
        .unwrap_or(0.0);
    let paddle_y = world.transforms[pong.paddles[1].0 as usize]
        .as_ref()
        .map(|t| t.y)
        .unwrap_or(0.0);
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
    let phase = pong_ref(world).phase;
    if phase == PongPhase::GameOver {
        return;
    }

    let p1_input = world.input.bits;
    let p2_input = compute_ai_input(world);
    let inputs = [p1_input, p2_input];
    let paddles = pong_ref(world).paddles;

    for (idx, &input_bits) in inputs.iter().enumerate() {
        let up = (input_bits & INPUT_UP) != 0;
        let down = (input_bits & INPUT_DOWN) != 0;
        let dir = (down as i32) - (up as i32);
        let paddle = paddles[idx];
        if let Some(velocity) = world.velocities[paddle.0 as usize].as_mut() {
            velocity.y = dir as f32 * PADDLE_SPEED;
        }
    }
}

fn move_entities(world: &mut World, dt: f32) {
    let phase = pong_ref(world).phase;
    if phase == PongPhase::GameOver {
        return;
    }

    let ball = pong_ref(world).ball;
    let paddles = pong_ref(world).paddles;
    if ball_visible(world) {
        if let (Some(transform), Some(velocity)) = (
            world.transforms[ball.0 as usize].as_mut(),
            world.velocities[ball.0 as usize].as_mut(),
        ) {
            transform.x += velocity.x * dt;
            transform.y += velocity.y * dt;
            if let Some(spin) = world.spins[ball.0 as usize].as_mut() {
                velocity.y += spin.value * dt;
                spin.value *= SPIN_DECAY_RATE.powf(dt);
            }
        }
    }

    let paddle_half_height = PADDLE_HEIGHT / 2.0;
    for &paddle in &paddles {
        if let (Some(transform), Some(velocity)) = (
            world.transforms[paddle.0 as usize].as_mut(),
            world.velocities[paddle.0 as usize].as_mut(),
        ) {
            transform.y += velocity.y * dt;
            transform.y = transform
                .y
                .clamp(paddle_half_height, world.field.height - paddle_half_height);
        }
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

fn collide_paddles(world: &mut World) {
    let phase = pong_ref(world).phase;
    if phase == PongPhase::GameOver || !ball_visible(world) {
        return;
    }

    let paddles = pong_ref(world).paddles;
    let ball = pong_ref(world).ball;
    let paddle_positions = [PADDLE1_X, PADDLE2_X];

    for (i, &paddle_x) in paddle_positions.iter().enumerate() {
        let paddle_entity = paddles[i];
        let paddle_y = world.transforms[paddle_entity.0 as usize]
            .as_ref()
            .map(|t| t.y)
            .unwrap_or(0.0);

        let (ball_x, ball_y) = world.transforms[ball.0 as usize]
            .as_ref()
            .map(|t| (t.x, t.y))
            .unwrap_or((0.0, 0.0));

        if !check_paddle_collision(ball_x, ball_y, paddle_x, paddle_y) {
            continue;
        }

        let (ball_velocity_x, ball_velocity_y) = world.velocities[ball.0 as usize]
            .as_ref()
            .map(|v| (v.x, v.y))
            .unwrap_or((0.0, 0.0));
        let speed =
            (ball_velocity_x.hypot(ball_velocity_y) * BALL_SPEED_ACCEL_FACTOR).min(BALL_MAX_SPEED);
        let paddle_half_h = PADDLE_HEIGHT / 2.0;
        let offset = ((ball_y - paddle_y) / paddle_half_h).clamp(-1.0, 1.0);
        let angle = offset * MAX_BOUNCE_ANGLE;
        let direction = if i == 0 { 1.0_f32 } else { -1.0 };

        if let Some(velocity) = world.velocities[ball.0 as usize].as_mut() {
            velocity.x = direction * speed * angle.cos();
            velocity.y = speed * angle.sin();
        }

        if let (Some(spin), Some(paddle_velocity)) = (
            world.spins[ball.0 as usize].as_mut(),
            world.velocities[paddle_entity.0 as usize].as_ref(),
        ) {
            let spin_transfer = -paddle_velocity.y * SPIN_TRANSFER_RATE;
            spin.value = (spin.value + spin_transfer).clamp(-SPIN_MAX, SPIN_MAX);
        }

        let paddle_top = paddle_y - paddle_half_h;
        let paddle_bottom = paddle_y + paddle_half_h;
        let is_face_hit = ball_y >= paddle_top && ball_y <= paddle_bottom;

        if let Some(transform) = world.transforms[ball.0 as usize].as_mut() {
            if is_face_hit {
                transform.x = paddle_x + direction * (PADDLE_WIDTH / 2.0 + BALL_RADIUS);
            } else if ball_y < paddle_top {
                transform.y = paddle_top - BALL_RADIUS;
            } else {
                transform.y = paddle_bottom + BALL_RADIUS;
            }
        }
    }
}

fn collide_walls(world: &mut World) {
    if pong_ref(world).phase == PongPhase::GameOver || !ball_visible(world) {
        return;
    }

    let ball = pong_ref(world).ball;
    let (ball_x, ball_y) = world.transforms[ball.0 as usize]
        .as_ref()
        .map(|t| (t.x, t.y))
        .unwrap_or((0.0, 0.0));

    if ball_x - BALL_RADIUS <= 0.0 {
        pong_mut(world).conceded_by = Some(PongPlayer::One);
    } else if ball_x + BALL_RADIUS >= world.field.width {
        pong_mut(world).conceded_by = Some(PongPlayer::Two);
    }

    if ball_y - BALL_RADIUS <= 0.0 || ball_y + BALL_RADIUS >= world.field.height {
        if let Some(velocity) = world.velocities[ball.0 as usize].as_mut() {
            velocity.y = -velocity.y;
        }
        if let Some(spin) = world.spins[ball.0 as usize].as_mut() {
            spin.value = -spin.value;
        }
        if let Some(transform) = world.transforms[ball.0 as usize].as_mut() {
            transform.y = transform
                .y
                .clamp(BALL_RADIUS, world.field.height - BALL_RADIUS);
        }
    }
}

fn resolve_scoring(world: &mut World) {
    let conceder = match pong_ref(world).conceded_by {
        Some(player) => player,
        None => return,
    };

    let scorer = match conceder {
        PongPlayer::One => PongPlayer::Two,
        PongPlayer::Two => PongPlayer::One,
    };

    let pong = pong_mut(world);
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
    if pong_ref(world).phase != PongPhase::GameOver {
        return;
    }
    if (world.input.bits & INPUT_ACTION) != 0 {
        reset_game(world);
    }
}

fn tick_serve(world: &mut World, dt: f32) {
    let phase = pong_ref(world).phase;
    if phase != PongPhase::Playing {
        return;
    }
    if !ball_visible(world) {
        tick_serve_delay(world, dt);
    }
}

fn apply_physics(world: &mut World, dt: f32) {
    let phase = pong_ref(world).phase;
    if phase == PongPhase::GameOver {
        return;
    }
    let dt = clamp_dt(dt);
    move_entities(world, dt);
    if ball_visible(world) {
        collide_paddles(world);
        collide_walls(world);
        resolve_scoring(world);
    }
}

pub fn build_world(width: f32, height: f32) -> (World, Schedule, Snapshot) {
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
        winning_score: DEFAULT_WINNING_SCORE,
        serve_delay_remaining: 0.0,
    });

    launch_ball(&mut world);

    let schedule = Schedule::new()
        .with_system(handle_restart)
        .with_system(apply_input)
        .with_system(apply_physics)
        .with_system(tick_serve);

    (
        world,
        schedule,
        Snapshot::new(write_snapshot, vec![0.0; SnapshotField::Count as usize]),
    )
}

fn write_snapshot(world: &World, snapshot: &mut [f32]) {
    use SnapshotField::*;

    let pong = pong_ref(world);
    let ball = pong.ball;
    let ball_visible = pong.ball_visible();

    let (ball_x, ball_y) = world.transforms[ball.0 as usize]
        .as_ref()
        .map(|t| (t.x, t.y))
        .unwrap_or((0.0, 0.0));
    let (ball_vx, ball_vy) = world.velocities[ball.0 as usize]
        .as_ref()
        .map(|v| (v.x, v.y))
        .unwrap_or((0.0, 0.0));

    snapshot[BallX.idx()] = ball_x;
    snapshot[BallY.idx()] = ball_y;
    snapshot[BallVx.idx()] = ball_vx;
    snapshot[BallVy.idx()] = ball_vy;
    snapshot[Paddle1X.idx()] = PADDLE1_X;
    snapshot[Paddle1Y.idx()] = world.transforms[pong.paddles[0].0 as usize]
        .as_ref()
        .map(|t| t.y)
        .unwrap_or(0.0);
    snapshot[Paddle2X.idx()] = PADDLE2_X;
    snapshot[Paddle2Y.idx()] = world.transforms[pong.paddles[1].0 as usize]
        .as_ref()
        .map(|t| t.y)
        .unwrap_or(0.0);
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
