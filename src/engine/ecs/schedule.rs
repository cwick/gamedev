use super::world::World;

pub type SystemFn = fn(&mut World, f32);

const PHASE_COUNT: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SystemPhase {
    /// Intent and velocity setup.
    Control,
    /// Engine-owned integration and other shared physics helpers.
    Physics,
    /// Post-integration consequences (collisions, scoring, timers).
    Resolve,
}

impl SystemPhase {
    const ORDERED: [SystemPhase; PHASE_COUNT] = [
        SystemPhase::Control,
        SystemPhase::Physics,
        SystemPhase::Resolve,
    ];

    fn index(self) -> usize {
        match self {
            SystemPhase::Control => 0,
            SystemPhase::Physics => 1,
            SystemPhase::Resolve => 2,
        }
    }
}

#[derive(Clone, Default)]
pub struct Schedule {
    systems: [Vec<SystemFn>; PHASE_COUNT],
}

impl Schedule {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_system(self, system: SystemFn) -> Self {
        self.with_system_in_phase(SystemPhase::Control, system)
    }

    pub fn with_system_in_phase(mut self, phase: SystemPhase, system: SystemFn) -> Self {
        self.systems[phase.index()].push(system);
        self
    }

    pub fn run(&self, world: &mut World, dt: f32) {
        for phase in SystemPhase::ORDERED {
            for system in &self.systems[phase.index()] {
                system(world, dt);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ecs::world::World;

    fn system_set_flag(world: &mut World, _dt: f32) {
        world.input.bits = 1;
    }

    fn system_require_flag_then_set_next(world: &mut World, _dt: f32) {
        assert_eq!(world.input.bits, 1);
        world.input.bits = 2;
    }

    fn system_write_dt(world: &mut World, dt: f32) {
        world.field.width = dt;
    }

    #[derive(Default)]
    struct Trace {
        calls: Vec<&'static str>,
    }

    fn record(event: &'static str, world: &mut World, _dt: f32) {
        world.resource_mut::<Trace>().calls.push(event);
    }

    fn record_control(world: &mut World, dt: f32) {
        record("control", world, dt);
    }

    fn record_physics(world: &mut World, dt: f32) {
        record("physics", world, dt);
    }

    fn record_resolve(world: &mut World, dt: f32) {
        record("resolve", world, dt);
    }

    #[test]
    fn runs_systems_in_order() {
        let mut world = World::new(800.0, 600.0);
        let schedule = Schedule::new()
            .with_system(system_set_flag)
            .with_system(system_require_flag_then_set_next);

        schedule.run(&mut world, 0.0);

        assert_eq!(world.input.bits, 2);
    }

    #[test]
    fn passes_dt_through_to_systems() {
        let mut world = World::new(800.0, 600.0);
        let schedule = Schedule::new().with_system(system_write_dt);

        schedule.run(&mut world, 0.123);

        assert_eq!(world.field.width, 0.123);
    }

    #[test]
    fn preserves_phase_order() {
        let mut world = World::new(800.0, 600.0);
        world.insert_resource(Trace::default());

        let schedule = Schedule::new()
            .with_system_in_phase(SystemPhase::Control, record_control)
            .with_system_in_phase(SystemPhase::Physics, record_physics)
            .with_system_in_phase(SystemPhase::Resolve, record_resolve);

        schedule.run(&mut world, 0.0);

        assert_eq!(
            world.resource::<Trace>().calls,
            vec!["control", "physics", "resolve"]
        );
    }

    fn record_first(world: &mut World, _dt: f32) {
        world.resource_mut::<Trace>().calls.push("first");
    }

    fn record_second(world: &mut World, _dt: f32) {
        world.resource_mut::<Trace>().calls.push("second");
    }

    #[test]
    fn keeps_insertion_order_within_phase() {
        let mut world = World::new(800.0, 600.0);
        world.insert_resource(Trace::default());

        let schedule = Schedule::new()
            .with_system_in_phase(SystemPhase::Control, record_first)
            .with_system_in_phase(SystemPhase::Control, record_second);

        schedule.run(&mut world, 0.0);

        assert_eq!(world.resource::<Trace>().calls, vec!["first", "second"]);
    }
}
