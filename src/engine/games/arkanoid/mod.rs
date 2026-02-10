use crate::engine::ecs::components::{Collider, Renderable, Transform, Velocity};
use crate::engine::ecs::entity::EntityId;
use crate::engine::ecs::schedule::Schedule;
use crate::engine::ecs::world::World;
use crate::engine::{Snapshot, INPUT_LEFT, INPUT_RIGHT};

const PADDLE_WIDTH: f32 = 96.0;
const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_SPEED: f32 = 400.0;

const SNAPSHOT_LEN: usize = SnapshotField::Count as usize;

struct ArkanoidState {
    paddle: EntityId,
}

fn apply_input(world: &mut World, _dt: f32) {
    let state = world
        .get_resource::<ArkanoidState>()
        .expect("apply_input requires ArkanoidState");
    let paddle = state.paddle;

    let input_bits = world.input.bits;
    let left = (input_bits & INPUT_LEFT) != 0;
    let right = (input_bits & INPUT_RIGHT) != 0;
    let dir = (right as i32) - (left as i32);

    world.velocity_mut(paddle).x = dir as f32 * PADDLE_SPEED;
}

fn apply_physics(world: &mut World, dt: f32) {
    let state = world
        .get_resource::<ArkanoidState>()
        .expect("apply_physics requires ArkanoidState");
    let paddle = state.paddle;

    let vx = world.velocity(paddle).x;
    let field_width = world.field.width;
    world.transform_mut(paddle).x += vx * dt;

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

    world.insert_resource(ArkanoidState { paddle });

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

    let state = world
        .get_resource::<ArkanoidState>()
        .expect("arkanoid snapshot requires ArkanoidState");
    let transform = world.transform(state.paddle);

    snapshot[PaddleX.idx()] = transform.x;
    snapshot[PaddleY.idx()] = transform.y;
    snapshot[PaddleWidth.idx()] = PADDLE_WIDTH;
    snapshot[PaddleHeight.idx()] = PADDLE_HEIGHT;
}

#[repr(usize)]
enum SnapshotField {
    PaddleX = 0,
    PaddleY = 1,
    PaddleWidth = 2,
    PaddleHeight = 3,
    Count = 4,
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
