use crate::engine::ecs::world::World;

pub fn integrate_velocity(world: &mut World, dt: f32) {
    let (transforms, velocities) = (&mut world.transforms, &world.velocities);

    for (transform, velocity) in transforms.iter_mut().zip(velocities.iter()) {
        if let (Some(transform), Some(velocity)) = (transform, velocity) {
            transform.x += velocity.x * dt;
            transform.y += velocity.y * dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ecs::components::{Transform, Velocity};
    use crate::engine::ecs::world::World;

    #[test]
    fn moves_entities_with_both_components() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_transform(entity, Transform { x: 0.0, y: 0.0 });
        world.set_velocity(entity, Velocity { x: 100.0, y: 50.0 });

        integrate_velocity(&mut world, 0.1);

        assert_eq!(world.transform(entity).x, 10.0);
        assert_eq!(world.transform(entity).y, 5.0);
    }

    #[test]
    fn skips_entities_missing_transform() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_velocity(entity, Velocity { x: 100.0, y: 50.0 });

        integrate_velocity(&mut world, 0.1);
    }

    #[test]
    fn skips_entities_missing_velocity() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_transform(entity, Transform { x: 10.0, y: 20.0 });

        integrate_velocity(&mut world, 0.1);

        assert_eq!(world.transform(entity).x, 10.0);
        assert_eq!(world.transform(entity).y, 20.0);
    }

    #[test]
    fn respects_dt_scaling() {
        let mut world1 = World::new(800.0, 600.0);
        let entity1 = world1.spawn();
        world1.set_transform(entity1, Transform { x: 0.0, y: 0.0 });
        world1.set_velocity(entity1, Velocity { x: 100.0, y: 0.0 });

        integrate_velocity(&mut world1, 0.05);
        let x_after_half_step = world1.transform(entity1).x;

        let mut world2 = World::new(800.0, 600.0);
        let entity2 = world2.spawn();
        world2.set_transform(entity2, Transform { x: 0.0, y: 0.0 });
        world2.set_velocity(entity2, Velocity { x: 100.0, y: 0.0 });

        integrate_velocity(&mut world2, 0.1);
        let x_after_full_step = world2.transform(entity2).x;

        assert_eq!(x_after_half_step, 5.0);
        assert_eq!(x_after_full_step, 10.0);
        assert_eq!(x_after_full_step, x_after_half_step * 2.0);
    }

    #[test]
    fn handles_negative_velocity() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_transform(entity, Transform { x: 100.0, y: 100.0 });
        world.set_velocity(entity, Velocity { x: -50.0, y: -25.0 });

        integrate_velocity(&mut world, 0.1);

        assert_eq!(world.transform(entity).x, 95.0);
        assert_eq!(world.transform(entity).y, 97.5);
    }

    #[test]
    fn handles_zero_velocity() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_transform(entity, Transform { x: 50.0, y: 75.0 });
        world.set_velocity(entity, Velocity { x: 0.0, y: 0.0 });

        integrate_velocity(&mut world, 1.0);

        assert_eq!(world.transform(entity).x, 50.0);
        assert_eq!(world.transform(entity).y, 75.0);
    }

    #[test]
    fn handles_zero_dt() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_transform(entity, Transform { x: 0.0, y: 0.0 });
        world.set_velocity(entity, Velocity { x: 100.0, y: 100.0 });

        integrate_velocity(&mut world, 0.0);

        assert_eq!(world.transform(entity).x, 0.0);
        assert_eq!(world.transform(entity).y, 0.0);
    }

    #[test]
    fn handles_multiple_entities_independently() {
        let mut world = World::new(800.0, 600.0);

        let entity1 = world.spawn();
        world.set_transform(entity1, Transform { x: 0.0, y: 0.0 });
        world.set_velocity(entity1, Velocity { x: 100.0, y: 50.0 });

        let entity2 = world.spawn();
        world.set_transform(entity2, Transform { x: 10.0, y: 20.0 });
        world.set_velocity(entity2, Velocity { x: 20.0, y: 30.0 });

        integrate_velocity(&mut world, 0.1);

        assert_eq!(world.transform(entity1).x, 10.0);
        assert_eq!(world.transform(entity1).y, 5.0);
        assert_eq!(world.transform(entity2).x, 12.0);
        assert_eq!(world.transform(entity2).y, 23.0);
    }

    #[test]
    fn handles_large_velocity_and_dt() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_transform(entity, Transform { x: 0.0, y: 0.0 });
        world.set_velocity(
            entity,
            Velocity {
                x: 1000.0,
                y: 500.0,
            },
        );

        integrate_velocity(&mut world, 0.1);

        assert_eq!(world.transform(entity).x, 100.0);
        assert_eq!(world.transform(entity).y, 50.0);
    }

    #[test]
    fn handles_small_velocity_and_dt() {
        let mut world = World::new(800.0, 600.0);
        let entity = world.spawn();
        world.set_transform(entity, Transform { x: 0.0, y: 0.0 });
        world.set_velocity(entity, Velocity { x: 0.1, y: 0.01 });

        integrate_velocity(&mut world, 0.001);

        let x = world.transform(entity).x;
        let y = world.transform(entity).y;
        assert!((x - 0.0001).abs() < 1e-8);
        assert!((y - 0.00001).abs() < 1e-8);
    }
}
