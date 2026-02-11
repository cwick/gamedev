use super::world::World;

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
    use super::super::components::{Transform, Velocity};
    use super::super::world::World;
    use super::integrate_velocity;

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
}
