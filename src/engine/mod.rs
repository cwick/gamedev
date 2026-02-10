#![allow(dead_code)]

pub mod ecs;

use crate::engine::ecs::schedule::Schedule;
use crate::engine::ecs::world::World;

pub const INPUT_UP: u32 = 0b0000_0001;
pub const INPUT_DOWN: u32 = 0b0000_0010;
pub const INPUT_LEFT: u32 = 0b0000_0100;
pub const INPUT_RIGHT: u32 = 0b0000_1000;
pub const INPUT_ACTION: u32 = 0b0001_0000;

pub const MAX_DT: f32 = 0.05;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GameId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameDefinition {
    pub id: GameId,
    pub name: &'static str,
}

fn clamp_dt(dt_seconds: f32) -> f32 {
    dt_seconds.clamp(0.0, MAX_DT)
}

type SnapshotWriter = fn(&World, &mut [f32]);
pub struct Snapshot {
    writer: SnapshotWriter,
    buffer: Vec<f32>,
}

impl Snapshot {
    pub fn new(writer: SnapshotWriter, buffer: Vec<f32>) -> Self {
        Self { writer, buffer }
    }

    pub fn update(&mut self, world: &World) {
        (self.writer)(world, &mut self.buffer);
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn as_ptr(&self) -> *const f32 {
        self.buffer.as_ptr()
    }
}

pub struct Engine {
    world: World,
    schedule: Schedule,
    snapshot: Snapshot,
}

impl Engine {
    pub fn new(world: World, schedule: Schedule, snapshot: Snapshot) -> Self {
        let mut engine = Self {
            world,
            schedule,
            snapshot,
        };
        engine.snapshot.update(&engine.world);
        engine
    }

    pub fn step(&mut self, dt_seconds: f32, input_bits: u32) {
        let dt = clamp_dt(dt_seconds);
        self.world.input.bits = input_bits;
        self.schedule.run(&mut self.world, dt);
    }

    pub fn snapshot_ptr(&mut self) -> *const f32 {
        self.snapshot.update(&self.world);
        self.snapshot.as_ptr()
    }

    pub fn snapshot_len(&self) -> usize {
        self.snapshot.len()
    }
}
