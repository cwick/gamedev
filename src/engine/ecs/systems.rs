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

pub fn bounce_in_field(world: &mut World, _dt: f32) {
    let field_width = world.field.width;
    let field_height = world.field.height;

    for ((collider, transform), velocity) in world
        .wall_bounce_colliders
        .iter()
        .zip(world.transforms.iter_mut())
        .zip(world.velocities.iter_mut())
    {
        let (Some(collider), Some(transform), Some(velocity)) = (collider, transform, velocity)
        else {
            continue;
        };

        let radius = collider.radius;

        let mut bounce_x = false;
        let mut bounce_y = false;

        if transform.x - radius <= 0.0 || transform.x + radius >= field_width {
            bounce_x = true;
            transform.x = transform.x.clamp(radius, field_width - radius);
        }

        if transform.y - radius <= 0.0 || transform.y + radius >= field_height {
            bounce_y = true;
            transform.y = transform.y.clamp(radius, field_height - radius);
        }

        if bounce_x {
            velocity.x = -velocity.x;
        }
        if bounce_y {
            velocity.y = -velocity.y;
        }
    }
}

#[cfg(test)]
mod tests {
    mod integrate_velocity {
        use super::super::integrate_velocity;
        use crate::engine::ecs::components::{Transform, Velocity};
        use crate::engine::World;

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

    mod bounce_in_field {
        use super::super::bounce_in_field;
        use crate::engine::ecs::components::{BounceCollider, Transform, Velocity};
        use crate::engine::World;

        #[test]
        fn clamps_position_and_reflects_velocity() {
            let mut world = World::new(16.0, 16.0);
            let entity = world.spawn();
            world.set_transform(entity, Transform { x: 1.0, y: 1.0 });
            world.set_velocity(entity, Velocity { x: -5.0, y: -7.0 });
            world.set_wall_bounce_collider(entity, BounceCollider { radius: 2.0 });

            bounce_in_field(&mut world, 0.0);

            assert_eq!(world.transform(entity).x, 2.0);
            assert_eq!(world.transform(entity).y, 2.0);
            assert_eq!(world.velocity(entity).x, 5.0);
            assert_eq!(world.velocity(entity).y, 7.0);
        }

        #[test]
        fn skips_entities_missing_transform_or_velocity() {
            let mut world = World::new(16.0, 16.0);

            let missing_transform = world.spawn();
            world.set_velocity(missing_transform, Velocity { x: -3.0, y: -4.0 });
            world.set_wall_bounce_collider(missing_transform, BounceCollider { radius: 2.0 });

            let missing_velocity = world.spawn();
            world.set_transform(missing_velocity, Transform { x: 1.0, y: 1.0 });
            world.set_wall_bounce_collider(missing_velocity, BounceCollider { radius: 2.0 });

            let valid = world.spawn();
            world.set_transform(valid, Transform { x: 1.0, y: 1.0 });
            world.set_velocity(valid, Velocity { x: -5.0, y: -7.0 });
            world.set_wall_bounce_collider(valid, BounceCollider { radius: 2.0 });

            bounce_in_field(&mut world, 0.0);

            assert_eq!(world.transform(valid).x, 2.0);
            assert_eq!(world.transform(valid).y, 2.0);
            assert_eq!(world.velocity(valid).x, 5.0);
            assert_eq!(world.velocity(valid).y, 7.0);

            assert_eq!(world.transform(missing_velocity).x, 1.0);
            assert_eq!(world.transform(missing_velocity).y, 1.0);
            assert_eq!(world.velocity(missing_transform).x, -3.0);
            assert_eq!(world.velocity(missing_transform).y, -4.0);
        }
    }
}
