use crate::engine::ecs::components::{BounceCollider, Transform, Velocity};
use crate::engine::ecs::entity::EntityId;
use crate::engine::ecs::schedule::{Schedule, SystemPhase};
use crate::engine::ecs::world::World;
use crate::engine::{Snapshot, INPUT_LEFT, INPUT_RIGHT};

const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_SPEED: f32 = 400.0;

const BALL_RADIUS: f32 = 6.0;
const BALL_SPEED: f32 = 420.0;

const SNAPSHOT_LEN: usize = SnapshotField::Count as usize;

struct ArkanoidState {
    paddle: EntityId,
    ball: EntityId,
}

fn apply_input(world: &mut World, dt: f32) {
    let &ArkanoidState { paddle, .. } = world.resource::<ArkanoidState>();

    let input_bits = world.input.bits;
    let mouse_delta_raw = (input_bits >> 16) as u16 as i16;
    let mouse_delta = mouse_delta_raw as f32;

    if mouse_delta.abs() > 0.1 {
        const MOUSE_SENSITIVITY: f32 = 1.5;
        world.transform_mut(paddle).x += mouse_delta * MOUSE_SENSITIVITY;
    } else {
        let left = (input_bits & INPUT_LEFT) != 0;
        let right = (input_bits & INPUT_RIGHT) != 0;
        let dir = (right as i32) - (left as i32);
        world.transform_mut(paddle).x += dir as f32 * PADDLE_SPEED * dt;
    }
}

fn clamp_paddle_to_field(world: &mut World, _dt: f32) {
    let &ArkanoidState { paddle, .. } = world.resource::<ArkanoidState>();
    let field_width = world.field.width;
    let paddle_half_width = PADDLE_WIDTH / 2.0;
    let min_x = paddle_half_width;
    let max_x = field_width - paddle_half_width;
    let paddle_transform = world.transform_mut(paddle);
    paddle_transform.x = paddle_transform.x.clamp(min_x, max_x);
}

fn ball_paddle_collision(world: &mut World, _dt: f32) {
    let &ArkanoidState { paddle, ball } = world.resource::<ArkanoidState>();

    let paddle_transform = *world.transform(paddle);
    let ball_transform = *world.transform(ball);
    let ball_velocity = world.velocity(ball);

    if ball_velocity.y <= 0.0 {
        return;
    }

    let paddle_half_width = PADDLE_WIDTH / 2.0;
    let paddle_half_height = PADDLE_HEIGHT / 2.0;
    let paddle_left = paddle_transform.x - paddle_half_width;
    let paddle_right = paddle_transform.x + paddle_half_width;
    let paddle_top = paddle_transform.y - paddle_half_height;
    let paddle_bottom = paddle_transform.y + paddle_half_height;

    let ball_bottom = ball_transform.y + BALL_RADIUS;

    if ball_bottom < paddle_top || ball_bottom > paddle_bottom {
        return;
    }

    if ball_transform.x < paddle_left || ball_transform.x > paddle_right {
        return;
    }

    let relative_hit = (ball_transform.x - paddle_left) / PADDLE_WIDTH;
    let zone_index = (relative_hit * 8.0).floor() as usize;
    let zone_index = zone_index.clamp(0, 7);

    let angles_from_horizontal: [f32; 8] = [15.0, 20.0, 30.0, 60.0, 60.0, 30.0, 20.0, 15.0];
    let angle_from_horizontal = angles_from_horizontal[zone_index];
    let angle_rad = (90.0 - angle_from_horizontal).to_radians();

    let speed = (ball_velocity.x * ball_velocity.x + ball_velocity.y * ball_velocity.y).sqrt();

    let direction = if zone_index < 4 { -1.0 } else { 1.0 };

    let new_vx = direction * angle_rad.sin() * speed;
    let new_vy = -angle_rad.cos() * speed;

    let ball_velocity_mut = world.velocity_mut(ball);
    ball_velocity_mut.x = new_vx;
    ball_velocity_mut.y = new_vy;

    let ball_transform_mut = world.transform_mut(ball);
    ball_transform_mut.y = paddle_top - BALL_RADIUS;
}

pub fn build_world(width: f32, height: f32) -> (World, Schedule, Snapshot) {
    let mut world = World::new(width, height);
    let paddle = world.spawn();

    world.set_transform(
        paddle,
        Transform {
            x: width / 2.0,
            y: height - PADDLE_HEIGHT - (PADDLE_HEIGHT / 2.0),
        },
    );

    let ball = world.spawn();
    world.set_transform(
        ball,
        Transform {
            x: width / 2.0,
            y: height / 2.0,
        },
    );

    let angle = std::f32::consts::PI / 4.0;
    let vx = angle.cos() * BALL_SPEED;
    let vy = angle.sin() * BALL_SPEED;
    world.set_velocity(ball, Velocity { x: vx, y: vy });
    world.set_wall_bounce_collider(
        ball,
        BounceCollider {
            radius: BALL_RADIUS,
        },
    );

    world.insert_resource(ArkanoidState { paddle, ball });

    let schedule = Schedule::new()
        .with_system_in_phase(SystemPhase::Control, apply_input)
        .with_system_in_phase(SystemPhase::Resolve, clamp_paddle_to_field)
        .with_system_in_phase(SystemPhase::Resolve, ball_paddle_collision);

    (
        world,
        schedule,
        Snapshot::new(write_snapshot, vec![0.0; SNAPSHOT_LEN]),
    )
}

fn write_snapshot(world: &World, snapshot: &mut [f32]) {
    use SnapshotField::*;

    let &ArkanoidState { paddle, ball } = world.resource::<ArkanoidState>();

    let paddle_transform = world.transform(paddle);
    snapshot[PaddleX.idx()] = paddle_transform.x;
    snapshot[PaddleY.idx()] = paddle_transform.y;
    snapshot[PaddleWidth.idx()] = PADDLE_WIDTH;
    snapshot[PaddleHeight.idx()] = PADDLE_HEIGHT;

    let ball_transform = world.transform(ball);
    snapshot[BallX.idx()] = ball_transform.x;
    snapshot[BallY.idx()] = ball_transform.y;
    snapshot[BallSize.idx()] = BALL_RADIUS * 2.0;
}

#[repr(usize)]
enum SnapshotField {
    PaddleX = 0,
    PaddleY = 1,
    PaddleWidth = 2,
    PaddleHeight = 3,
    BallX = 4,
    BallY = 5,
    BallSize = 6,
    Count = 7,
}

impl SnapshotField {
    const fn idx(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arkanoid_world_has_paddle_below_center() {
        let (world, _schedule, _snapshot) = build_world(800.0, 600.0);

        let paddle_transform = world.transforms.iter().find_map(|t| t.as_ref());
        let transform = paddle_transform.expect("paddle transform");
        assert!(transform.y > 550.0);
    }
}
