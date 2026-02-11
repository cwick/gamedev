use crate::engine::ecs::components::{BounceCollider, Renderable, Transform, Velocity};
use crate::engine::ecs::entity::EntityId;
use crate::engine::ecs::schedule::{Schedule, SystemPhase};
use crate::engine::ecs::world::World;
use crate::engine::{Snapshot, INPUT_LEFT, INPUT_RIGHT};

const PADDLE_WIDTH: f32 = 96.0;
const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_SPEED: f32 = 400.0;

const BALL_RADIUS: f32 = 6.0;
const BALL_SPEED: f32 = 300.0;

const SNAPSHOT_LEN: usize = SnapshotField::Count as usize;

struct ArkanoidState {
    paddle: EntityId,
    ball: EntityId,
}

fn apply_input(world: &mut World, _dt: f32) {
    let &ArkanoidState { paddle, .. } = world.resource::<ArkanoidState>();

    let input_bits = world.input.bits;
    let left = (input_bits & INPUT_LEFT) != 0;
    let right = (input_bits & INPUT_RIGHT) != 0;
    let dir = (right as i32) - (left as i32);

    world.velocity_mut(paddle).x = dir as f32 * PADDLE_SPEED;
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
    world.set_velocity(paddle, Velocity { x: 0.0, y: 0.0 });

    // TODO: unused
    world.set_renderable(
        paddle,
        Renderable {
            w: PADDLE_WIDTH,
            h: PADDLE_HEIGHT,
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
        .with_system_in_phase(SystemPhase::Resolve, clamp_paddle_to_field);

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
