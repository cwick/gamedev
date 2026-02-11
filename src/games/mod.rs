use crate::engine::ecs::schedule::Schedule;
use crate::engine::ecs::world::World;
use crate::engine::{Snapshot, TuningApi};

mod arkanoid;
mod pong;
mod registry;

type BuildFn = fn(f32, f32) -> (World, Schedule, Snapshot, TuningApi);

pub fn build_game(
    game_name: &str,
    width: f32,
    height: f32,
) -> (World, Schedule, Snapshot, TuningApi) {
    let entry = registry::GAMES
        .iter()
        .find(|entry| entry.def.name == game_name)
        .unwrap_or_else(|| {
            panic!("game definition for \"{}\" not found", game_name);
        });

    (entry.build)(width, height)
}
