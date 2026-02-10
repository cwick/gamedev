use super::{arkanoid, pong, BuildFn};
use crate::engine::{GameDefinition, GameId};

#[derive(Clone, Copy)]
pub struct GameEntry {
    pub def: GameDefinition,
    pub build: BuildFn,
}

const PONG_DEF: GameDefinition = GameDefinition {
    id: GameId(0),
    name: "Pong",
};

const ARKANOID_DEF: GameDefinition = GameDefinition {
    id: GameId(1),
    name: "Arkanoid",
};

pub const GAMES: &[GameEntry] = &[
    GameEntry {
        def: PONG_DEF,
        build: pong::build_world,
    },
    GameEntry {
        def: ARKANOID_DEF,
        build: arkanoid::build_world,
    },
];
