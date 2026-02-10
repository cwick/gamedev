use crate::engine::ecs::components::{Collider, Renderable, Transform};
use crate::engine::ecs::entity::EntityId;
use crate::engine::ecs::schedule::Schedule;
use crate::engine::ecs::world::World;
use crate::engine::Snapshot;

const PADDLE_WIDTH: f32 = 96.0;
const PADDLE_HEIGHT: f32 = 16.0;

const SNAPSHOT_LEN: usize = SnapshotField::Count as usize;

struct ArkanoidState {
    paddle: EntityId,
}

pub fn build_world(width: f32, height: f32) -> (World, Schedule, Snapshot) {
    let mut world = World::new(width, height);
    let paddle = world.spawn();

    world.set_transform(
        paddle,
        Transform {
            x: width / 2.0,
            y: height - PADDLE_HEIGHT * 2.0,
        },
    );
    world.set_collider(
        paddle,
        Collider {
            half_w: PADDLE_WIDTH / 2.0,
            half_h: PADDLE_HEIGHT / 2.0,
        },
    );
    world.set_renderable(
        paddle,
        Renderable {
            w: PADDLE_WIDTH,
            h: PADDLE_HEIGHT,
        },
    );

    world.insert_resource(ArkanoidState { paddle });

    let schedule = Schedule::new();

    (
        world,
        schedule,
        Snapshot::new(write_snapshot, vec![0.0; SNAPSHOT_LEN]),
    )
}

fn write_snapshot(world: &World, snapshot: &mut [f32]) {
    let state = world
        .get_resource::<ArkanoidState>()
        .expect("arkanoid snapshot requires ArkanoidState");
    let transform = world.transforms[state.paddle.0 as usize]
        .as_ref()
        .expect("paddle transform missing");

    snapshot[SnapshotField::PaddleX.idx()] = transform.x;
    snapshot[SnapshotField::PaddleY.idx()] = transform.y;
}

#[repr(usize)]
enum SnapshotField {
    PaddleX = 0,
    PaddleY = 1,
    Count = 2,
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
