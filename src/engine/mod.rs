#![allow(dead_code)]

pub mod ecs;

use crate::engine::ecs::schedule::{Schedule, SystemPhase};
use crate::engine::ecs::systems::{bounce_in_field, integrate_velocity};
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
type TuningSetFn = fn(&mut World, u32, f32) -> u32;
type TuningGetFn = fn(&World, u32) -> Option<f32>;
type TuningResetFn = fn(&mut World);

pub const TUNING_STATUS_APPLIED: u32 = 0;
pub const TUNING_STATUS_UNKNOWN_PARAM: u32 = 1;
pub const TUNING_STATUS_REJECTED: u32 = 2;

pub struct TuningApi {
    set: TuningSetFn,
    get: TuningGetFn,
    reset: TuningResetFn,
}

impl TuningApi {
    pub fn unsupported() -> Self {
        fn set_unsupported(_world: &mut World, _param_id: u32, _value: f32) -> u32 {
            TUNING_STATUS_UNKNOWN_PARAM
        }

        fn get_unsupported(_world: &World, _param_id: u32) -> Option<f32> {
            None
        }

        fn reset_unsupported(_world: &mut World) {}

        Self {
            set: set_unsupported,
            get: get_unsupported,
            reset: reset_unsupported,
        }
    }

    pub fn new(set: TuningSetFn, get: TuningGetFn, reset: TuningResetFn) -> Self {
        Self { set, get, reset }
    }
}

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
    tuning_api: TuningApi,
}

impl Engine {
    pub fn new(
        world: World,
        schedule: Schedule,
        snapshot: Snapshot,
        tuning_api: TuningApi,
    ) -> Self {
        let schedule = schedule
            .with_system_in_phase(SystemPhase::Physics, integrate_velocity)
            .with_system_in_phase(SystemPhase::Physics, bounce_in_field);
        let mut engine = Self {
            world,
            schedule,
            snapshot,
            tuning_api,
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

    pub fn set_tuning_param(&mut self, param_id: u32, value: f32) -> u32 {
        (self.tuning_api.set)(&mut self.world, param_id, value)
    }

    pub fn get_tuning_param(&self, param_id: u32) -> Option<f32> {
        (self.tuning_api.get)(&self.world, param_id)
    }

    pub fn reset_tuning_defaults(&mut self) {
        (self.tuning_api.reset)(&mut self.world);
    }
}
