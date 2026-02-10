use crate::engine::ecs::schedule::Schedule;
use crate::engine::ecs::world::World;
use crate::engine::Snapshot;

mod registry;

type BuildFn = fn(f32, f32) -> (World, Schedule, Snapshot);

pub fn build_game(game_name: &str, width: f32, height: f32) -> (World, Schedule, Snapshot) {
    let entry = registry::GAMES
        .iter()
        .find(|entry| entry.def.name == game_name)
        .unwrap_or_else(|| {
            panic!("game definition for \"{}\" not found", game_name);
        });

    (entry.build)(width, height)
}
