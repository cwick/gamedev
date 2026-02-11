use super::components::{BounceCollider, Spin, Transform, Velocity};
use super::entity::{EntityAllocator, EntityId};
use super::resources::{FieldBounds, InputBits};
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct World {
    pub transforms: Vec<Option<Transform>>,
    pub velocities: Vec<Option<Velocity>>,
    pub wall_bounce_colliders: Vec<Option<BounceCollider>>,
    pub spins: Vec<Option<Spin>>,
    pub input: InputBits,
    pub field: FieldBounds,
    resources: HashMap<TypeId, Box<dyn Any>>,
    allocator: EntityAllocator,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            transforms: Vec::new(),
            velocities: Vec::new(),
            wall_bounce_colliders: Vec::new(),
            spins: Vec::new(),
            input: InputBits { bits: 0 },
            field: FieldBounds { width, height },
            resources: HashMap::new(),
            allocator: EntityAllocator::new(),
        }
    }

    pub fn spawn(&mut self) -> EntityId {
        let entity = self.allocator.alloc();
        let idx = entity.0 as usize;
        self.ensure_capacity(idx);
        entity
    }

    pub fn despawn(&mut self, entity: EntityId) {
        let idx = entity.0 as usize;
        if idx < self.transforms.len() {
            self.transforms[idx] = None;
        }
        if idx < self.velocities.len() {
            self.velocities[idx] = None;
        }
        if idx < self.wall_bounce_colliders.len() {
            self.wall_bounce_colliders[idx] = None;
        }
        if idx < self.spins.len() {
            self.spins[idx] = None;
        }
        self.allocator.free(entity);
    }

    pub fn set_transform(&mut self, entity: EntityId, value: Transform) {
        self.ensure_capacity(entity.0 as usize);
        self.transforms[entity.0 as usize] = Some(value);
    }

    pub fn set_velocity(&mut self, entity: EntityId, value: Velocity) {
        self.ensure_capacity(entity.0 as usize);
        self.velocities[entity.0 as usize] = Some(value);
    }

    pub fn set_wall_bounce_collider(&mut self, entity: EntityId, value: BounceCollider) {
        self.ensure_capacity(entity.0 as usize);
        self.wall_bounce_colliders[entity.0 as usize] = Some(value);
    }

    pub fn set_spin(&mut self, entity: EntityId, value: Spin) {
        self.ensure_capacity(entity.0 as usize);
        self.spins[entity.0 as usize] = Some(value);
    }

    pub fn transform(&self, entity: EntityId) -> &Transform {
        let idx = entity.0 as usize;
        self.transforms
            .get(idx)
            .and_then(|opt| opt.as_ref())
            .expect("transform component missing")
    }

    pub fn transform_mut(&mut self, entity: EntityId) -> &mut Transform {
        let idx = entity.0 as usize;
        self.transforms
            .get_mut(idx)
            .and_then(|opt| opt.as_mut())
            .expect("transform component missing")
    }

    pub fn velocity(&self, entity: EntityId) -> &Velocity {
        let idx = entity.0 as usize;
        self.velocities
            .get(idx)
            .and_then(|opt| opt.as_ref())
            .expect("velocity component missing")
    }

    pub fn velocity_mut(&mut self, entity: EntityId) -> &mut Velocity {
        let idx = entity.0 as usize;
        self.velocities
            .get_mut(idx)
            .and_then(|opt| opt.as_mut())
            .expect("velocity component missing")
    }

    pub fn collider(&self, entity: EntityId) -> &BounceCollider {
        let idx = entity.0 as usize;
        self.wall_bounce_colliders
            .get(idx)
            .and_then(|opt| opt.as_ref())
            .expect("collider component missing")
    }

    pub fn collider_mut(&mut self, entity: EntityId) -> &mut BounceCollider {
        let idx = entity.0 as usize;
        self.wall_bounce_colliders
            .get_mut(idx)
            .and_then(|opt| opt.as_mut())
            .expect("collider component missing")
    }

    pub fn spin(&self, entity: EntityId) -> &Spin {
        let idx = entity.0 as usize;
        self.spins
            .get(idx)
            .and_then(|opt| opt.as_ref())
            .expect("spin component missing")
    }

    pub fn spin_mut(&mut self, entity: EntityId) -> &mut Spin {
        let idx = entity.0 as usize;
        self.spins
            .get_mut(idx)
            .and_then(|opt| opt.as_mut())
            .expect("spin component missing")
    }

    pub fn insert_resource<T: Any>(&mut self, value: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn resource<T: Any + 'static>(&self) -> &T {
        let type_name = std::any::type_name::<T>();
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .unwrap_or_else(|| panic!("resource {} not found", type_name))
    }

    pub fn resource_mut<T: Any + 'static>(&mut self) -> &mut T {
        let type_name = std::any::type_name::<T>();
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
            .unwrap_or_else(|| panic!("resource {} not found", type_name))
    }

    fn ensure_capacity(&mut self, idx: usize) {
        let target = idx + 1;
        if self.transforms.len() < target {
            self.transforms.resize_with(target, || None);
        }
        if self.velocities.len() < target {
            self.velocities.resize_with(target, || None);
        }
        if self.wall_bounce_colliders.len() < target {
            self.wall_bounce_colliders.resize_with(target, || None);
        }
        if self.spins.len() < target {
            self.spins.resize_with(target, || None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ecs::components::{Transform, Velocity};

    #[test]
    fn spawn_grows_all_component_storages() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        let idx = entity.0 as usize;

        assert!(world.transforms.len() > idx);
        assert!(world.velocities.len() > idx);
        assert!(world.wall_bounce_colliders.len() > idx);
        assert!(world.spins.len() > idx);
    }

    #[test]
    fn get_component_returns_what_was_set() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        let transform = Transform { x: 1.0, y: 2.0 };
        world.set_transform(entity, transform);

        assert_eq!(world.transform(entity), &transform);
    }

    #[test]
    fn get_component_mut_allows_modification() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_velocity(entity, Velocity { x: 1.0, y: 2.0 });

        world.velocity_mut(entity).x = 5.0;

        assert_eq!(world.velocity(entity), &Velocity { x: 5.0, y: 2.0 });
    }

    #[test]
    #[should_panic(expected = "transform component missing")]
    fn despawn_clears_component_slots() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_transform(entity, Transform { x: 1.0, y: 2.0 });
        world.set_velocity(entity, Velocity { x: 3.0, y: 4.0 });

        world.despawn(entity);

        world.transform(entity); // Should panic
    }
}
