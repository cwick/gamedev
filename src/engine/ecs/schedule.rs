use super::world::World;

pub type SystemFn = fn(&mut World, f32);

#[derive(Clone, Default)]
pub struct Schedule {
    systems: Vec<SystemFn>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn with_system(mut self, system: SystemFn) -> Self {
        self.systems.push(system);
        self
    }

    pub fn add_system(&mut self, system: SystemFn) {
        self.systems.push(system);
    }

    pub fn run(&self, world: &mut World, dt: f32) {
        for system in &self.systems {
            system(world, dt);
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
}
