use crate::engine::ecs::components::{Collider, Renderable, Transform, Velocity};
use crate::engine::ecs::entity::EntityId;
use crate::engine::ecs::schedule::Schedule;
use crate::engine::ecs::world::World;
use crate::engine::{Snapshot, INPUT_LEFT, INPUT_RIGHT};

const PADDLE_WIDTH: f32 = 96.0;
const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_SPEED: f32 = 400.0;

const BALL_SIZE: f32 = 12.0;
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

fn apply_physics(world: &mut World, dt: f32) {
    let &ArkanoidState { paddle, ball } = world.resource::<ArkanoidState>();

    let paddle_vx = world.velocity(paddle).x;
    let field_width = world.field.width;
    world.transform_mut(paddle).x += paddle_vx * dt;

    let paddle_half_width = PADDLE_WIDTH / 2.0;
    let min_x = paddle_half_width;
    let max_x = field_width - paddle_half_width;
    let paddle_transform = world.transform_mut(paddle);
    paddle_transform.x = paddle_transform.x.clamp(min_x, max_x);

    let field_height = world.field.height;
    let ball_radius = BALL_SIZE / 2.0;

    let ball_vx = world.velocity(ball).x;
    let ball_vy = world.velocity(ball).y;
    world.transform_mut(ball).x += ball_vx * dt;
    world.transform_mut(ball).y += ball_vy * dt;

    let ball_transform = world.transform_mut(ball);

    let mut bounce_x = false;
    let mut bounce_y = false;

    if ball_transform.x - ball_radius <= 0.0 || ball_transform.x + ball_radius >= field_width {
        bounce_x = true;
        ball_transform.x = ball_transform
            .x
            .clamp(ball_radius, field_width - ball_radius);
    }

    if ball_transform.y - ball_radius <= 0.0 || ball_transform.y + ball_radius >= field_height {
        bounce_y = true;
        ball_transform.y = ball_transform
            .y
            .clamp(ball_radius, field_height - ball_radius);
    }

    if bounce_x || bounce_y {
        let ball_velocity = world.velocity_mut(ball);
        if bounce_x {
            ball_velocity.x = -ball_velocity.x;
        }
        if bounce_y {
            ball_velocity.y = -ball_velocity.y;
        }
    }
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
    // TODO: unused
    world.set_collider(
        paddle,
        Collider {
            half_w: PADDLE_WIDTH / 2.0,
            half_h: PADDLE_HEIGHT / 2.0,
        },
    );
    // TODO: unused
    world.set_renderable(
        paddle,
        Renderable {
            w: PADDLE_WIDTH,
            h: PADDLE_HEIGHT,
        },
    );
    world.set_velocity(paddle, Velocity { x: 0.0, y: 0.0 });

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

    world.insert_resource(ArkanoidState { paddle, ball });

    let schedule = Schedule::new()
        .with_system(apply_input)
        .with_system(apply_physics);

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
    snapshot[BallSize.idx()] = BALL_SIZE;
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
