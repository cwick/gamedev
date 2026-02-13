use crate::engine::ecs::components::{BounceCollider, Transform, Velocity};
use crate::engine::ecs::entity::EntityId;
use crate::engine::ecs::schedule::{Schedule, SystemPhase};
use crate::engine::ecs::world::World;
use crate::engine::{
    Snapshot, TuningApi, INPUT_LEFT, INPUT_RIGHT, TUNING_STATUS_APPLIED, TUNING_STATUS_REJECTED,
    TUNING_STATUS_UNKNOWN_PARAM,
};

const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_SPEED: f32 = 400.0;
const BALL_RADIUS: f32 = 6.0;
const BALL_SPEED: f32 = 420.0;
const BOUNCE_ZONE_ANGLES: [f32; 8] = [15.0, 20.0, 30.0, 60.0, 60.0, 30.0, 20.0, 15.0];

const MIN_PADDLE_WIDTH: f32 = 20.0;
const MIN_PADDLE_HEIGHT: f32 = 6.0;
const MAX_PADDLE_HEIGHT: f32 = 80.0;
const MIN_PADDLE_SPEED: f32 = 50.0;
const MAX_PADDLE_SPEED: f32 = 1200.0;
const MIN_BALL_RADIUS: f32 = 2.0;
const MAX_BALL_RADIUS: f32 = 30.0;
const MIN_BALL_SPEED: f32 = 50.0;
const MAX_BALL_SPEED: f32 = 2000.0;
const MIN_ZONE_ANGLE: f32 = 5.0;
const MAX_ZONE_ANGLE: f32 = 85.0;
const PADDLE_WIDTH_MAX_RATIO: f32 = 0.9;

const SNAPSHOT_LEN: usize = SnapshotField::Count as usize;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArkanoidTuningParam {
    PaddleWidth = 0,
    PaddleHeight = 1,
    PaddleSpeed = 2,
    BallRadius = 3,
    BallSpeed = 4,
    BounceZone0Angle = 5,
    BounceZone1Angle = 6,
    BounceZone2Angle = 7,
    BounceZone3Angle = 8,
    BounceZone4Angle = 9,
    BounceZone5Angle = 10,
    BounceZone6Angle = 11,
    BounceZone7Angle = 12,
}

impl TryFrom<u32> for ArkanoidTuningParam {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PaddleWidth),
            1 => Ok(Self::PaddleHeight),
            2 => Ok(Self::PaddleSpeed),
            3 => Ok(Self::BallRadius),
            4 => Ok(Self::BallSpeed),
            5 => Ok(Self::BounceZone0Angle),
            6 => Ok(Self::BounceZone1Angle),
            7 => Ok(Self::BounceZone2Angle),
            8 => Ok(Self::BounceZone3Angle),
            9 => Ok(Self::BounceZone4Angle),
            10 => Ok(Self::BounceZone5Angle),
            11 => Ok(Self::BounceZone6Angle),
            12 => Ok(Self::BounceZone7Angle),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ArkanoidTuning {
    paddle_width: f32,
    paddle_height: f32,
    paddle_speed: f32,
    ball_radius: f32,
    ball_speed: f32,
    bounce_zone_angles: [f32; 8],
}

impl Default for ArkanoidTuning {
    fn default() -> Self {
        Self {
            paddle_width: PADDLE_WIDTH,
            paddle_height: PADDLE_HEIGHT,
            paddle_speed: PADDLE_SPEED,
            ball_radius: BALL_RADIUS,
            ball_speed: BALL_SPEED,
            bounce_zone_angles: BOUNCE_ZONE_ANGLES,
        }
    }
}

#[derive(Clone, Copy)]
struct ArkanoidState {
    paddle: EntityId,
    ball: EntityId,
}

fn zone_angle_index(param: ArkanoidTuningParam) -> Option<usize> {
    match param {
        ArkanoidTuningParam::BounceZone0Angle => Some(0),
        ArkanoidTuningParam::BounceZone1Angle => Some(1),
        ArkanoidTuningParam::BounceZone2Angle => Some(2),
        ArkanoidTuningParam::BounceZone3Angle => Some(3),
        ArkanoidTuningParam::BounceZone4Angle => Some(4),
        ArkanoidTuningParam::BounceZone5Angle => Some(5),
        ArkanoidTuningParam::BounceZone6Angle => Some(6),
        ArkanoidTuningParam::BounceZone7Angle => Some(7),
        _ => None,
    }
}

fn launch_velocity(speed: f32) -> Velocity {
    let angle = std::f32::consts::PI / 4.0;
    Velocity {
        x: angle.cos() * speed,
        y: angle.sin() * speed,
    }
}

fn apply_tuning(world: &mut World) {
    let state = *world.resource::<ArkanoidState>();
    let tuning = *world.resource::<ArkanoidTuning>();
    let paddle_y = world.field.height - tuning.paddle_height * 1.5;

    world.collider_mut(state.ball).radius = tuning.ball_radius;
    world.transform_mut(state.paddle).y = paddle_y;
    apply_ball_speed(world, state.ball, tuning.ball_speed);
}

fn apply_ball_speed(world: &mut World, ball: EntityId, target_speed: f32) {
    let current_velocity = world.velocity_mut(ball);
    let current_speed = current_velocity.x.hypot(current_velocity.y);
    let (dir_x, dir_y) = (
        current_velocity.x / current_speed,
        current_velocity.y / current_speed,
    );

    current_velocity.x = dir_x * target_speed;
    current_velocity.y = dir_y * target_speed;
}

fn set_tuning_param(world: &mut World, param_id: u32, value: f32) -> u32 {
    if !value.is_finite() {
        return TUNING_STATUS_REJECTED;
    }

    let Ok(param) = ArkanoidTuningParam::try_from(param_id) else {
        return TUNING_STATUS_UNKNOWN_PARAM;
    };

    let field_width = world.field.width;
    let mut tuning = *world.resource::<ArkanoidTuning>();
    match param {
        ArkanoidTuningParam::PaddleWidth => {
            let max_width = field_width * PADDLE_WIDTH_MAX_RATIO;
            tuning.paddle_width = value.clamp(MIN_PADDLE_WIDTH, max_width);
        }
        ArkanoidTuningParam::PaddleHeight => {
            tuning.paddle_height = value.clamp(MIN_PADDLE_HEIGHT, MAX_PADDLE_HEIGHT);
        }
        ArkanoidTuningParam::PaddleSpeed => {
            tuning.paddle_speed = value.clamp(MIN_PADDLE_SPEED, MAX_PADDLE_SPEED);
        }
        ArkanoidTuningParam::BallRadius => {
            tuning.ball_radius = value.clamp(MIN_BALL_RADIUS, MAX_BALL_RADIUS);
        }
        ArkanoidTuningParam::BallSpeed => {
            tuning.ball_speed = value.clamp(MIN_BALL_SPEED, MAX_BALL_SPEED);
        }
        _ => {
            if let Some(zone_index) = zone_angle_index(param) {
                tuning.bounce_zone_angles[zone_index] = value.clamp(MIN_ZONE_ANGLE, MAX_ZONE_ANGLE);
            }
        }
    }

    *world.resource_mut::<ArkanoidTuning>() = tuning;
    apply_tuning(world);

    TUNING_STATUS_APPLIED
}

fn get_tuning_param(world: &World, param_id: u32) -> Option<f32> {
    let param = ArkanoidTuningParam::try_from(param_id).ok()?;
    let tuning = *world.resource::<ArkanoidTuning>();
    let value = match param {
        ArkanoidTuningParam::PaddleWidth => tuning.paddle_width,
        ArkanoidTuningParam::PaddleHeight => tuning.paddle_height,
        ArkanoidTuningParam::PaddleSpeed => tuning.paddle_speed,
        ArkanoidTuningParam::BallRadius => tuning.ball_radius,
        ArkanoidTuningParam::BallSpeed => tuning.ball_speed,
        _ => {
            let zone_index = zone_angle_index(param)?;
            tuning.bounce_zone_angles[zone_index]
        }
    };
    Some(value)
}

fn reset_tuning_defaults(world: &mut World) {
    let defaults = ArkanoidTuning::default();
    *world.resource_mut::<ArkanoidTuning>() = defaults;
    apply_tuning(world);
}

fn apply_input(world: &mut World, dt: f32) {
    let state = *world.resource::<ArkanoidState>();
    let paddle_speed = world.resource::<ArkanoidTuning>().paddle_speed;

    let input_bits = world.input.bits;
    let mouse_delta_raw = (input_bits >> 16) as u16 as i16;
    let mouse_delta = mouse_delta_raw as f32;

    if mouse_delta.abs() > 0.1 {
        const MOUSE_SENSITIVITY: f32 = 1.5;
        world.transform_mut(state.paddle).x += mouse_delta * MOUSE_SENSITIVITY;
    } else {
        let left = (input_bits & INPUT_LEFT) != 0;
        let right = (input_bits & INPUT_RIGHT) != 0;
        let dir = (right as i32) - (left as i32);
        world.transform_mut(state.paddle).x += dir as f32 * paddle_speed * dt;
    }
}

fn clamp_paddle_to_field(world: &mut World, _dt: f32) {
    let state = *world.resource::<ArkanoidState>();
    let paddle_width = world.resource::<ArkanoidTuning>().paddle_width;
    let paddle = state.paddle;
    let field_width = world.field.width;
    let paddle_half_width = paddle_width / 2.0;
    let min_x = paddle_half_width;
    let max_x = field_width - paddle_half_width;
    let paddle_transform = world.transform_mut(paddle);
    paddle_transform.x = paddle_transform.x.clamp(min_x, max_x);
}

fn ball_paddle_collision(world: &mut World, _dt: f32) {
    let state = *world.resource::<ArkanoidState>();
    let tuning = *world.resource::<ArkanoidTuning>();

    let paddle_transform = *world.transform(state.paddle);
    let ball_transform = *world.transform(state.ball);
    let ball_velocity = world.velocity(state.ball);

    if ball_velocity.y <= 0.0 {
        return;
    }

    let paddle_half_width = tuning.paddle_width / 2.0;
    let paddle_half_height = tuning.paddle_height / 2.0;
    let paddle_left = paddle_transform.x - paddle_half_width;
    let paddle_right = paddle_transform.x + paddle_half_width;
    let paddle_top = paddle_transform.y - paddle_half_height;
    let paddle_bottom = paddle_transform.y + paddle_half_height;

    let ball_bottom = ball_transform.y + tuning.ball_radius;

    if ball_bottom < paddle_top || ball_bottom > paddle_bottom {
        return;
    }

    if ball_transform.x < paddle_left || ball_transform.x > paddle_right {
        return;
    }

    let relative_hit = (ball_transform.x - paddle_left) / tuning.paddle_width;
    let zone_index = (relative_hit * 8.0).floor() as usize;
    let zone_index = zone_index.clamp(0, 7);

    let angle_from_horizontal = tuning.bounce_zone_angles[zone_index];
    let angle_rad = (90.0 - angle_from_horizontal).to_radians();
    let speed = ball_velocity.x.hypot(ball_velocity.y);
    let direction = if zone_index < 4 { -1.0 } else { 1.0 };

    let new_vx = direction * angle_rad.sin() * speed;
    let new_vy = -angle_rad.cos() * speed;

    let ball_velocity_mut = world.velocity_mut(state.ball);
    ball_velocity_mut.x = new_vx;
    ball_velocity_mut.y = new_vy;

    let ball_transform_mut = world.transform_mut(state.ball);
    ball_transform_mut.y = paddle_top - tuning.ball_radius;
}

pub fn build_world(width: f32, height: f32) -> (World, Schedule, Snapshot, TuningApi) {
    let mut world = World::new(width, height);
    let tuning = ArkanoidTuning::default();
    let paddle = world.spawn();
    world.set_transform(
        paddle,
        Transform {
            x: width / 2.0,
            y: height - tuning.paddle_height * 1.5,
        },
    );

    let ball = world.spawn();
    world.set_transform(
        ball,
        Transform {
            x: width / 2.0,
            y: height / 2.0,
        },
    );
    world.set_velocity(ball, launch_velocity(tuning.ball_speed));
    world.set_wall_bounce_collider(
        ball,
        BounceCollider {
            radius: tuning.ball_radius,
        },
    );

    world.insert_resource(ArkanoidState { paddle, ball });
    world.insert_resource(tuning);

    let schedule = Schedule::new()
        .with_system_in_phase(SystemPhase::Control, apply_input)
        .with_system_in_phase(SystemPhase::Resolve, clamp_paddle_to_field)
        .with_system_in_phase(SystemPhase::Resolve, ball_paddle_collision);

    (
        world,
        schedule,
        Snapshot::new(write_snapshot, vec![0.0; SNAPSHOT_LEN]),
        TuningApi::new(set_tuning_param, get_tuning_param, reset_tuning_defaults),
    )
}

fn write_snapshot(world: &World, snapshot: &mut [f32]) {
    use SnapshotField::*;

    let &ArkanoidState { paddle, ball } = world.resource::<ArkanoidState>();
    let tuning = *world.resource::<ArkanoidTuning>();

    let paddle_transform = world.transform(paddle);
    snapshot[PaddleX.idx()] = paddle_transform.x;
    snapshot[PaddleY.idx()] = paddle_transform.y;
    snapshot[PaddleWidth.idx()] = tuning.paddle_width;
    snapshot[PaddleHeight.idx()] = tuning.paddle_height;

    let ball_transform = world.transform(ball);
    snapshot[BallX.idx()] = ball_transform.x;
    snapshot[BallY.idx()] = ball_transform.y;
    snapshot[BallSize.idx()] = tuning.ball_radius * 2.0;
}

#[repr(usize)]
enum SnapshotField {
    PaddleX = 0,
    PaddleY = 1,
    PaddleWidth = 2,
    PaddleHeight = 3,
    BallX = 4,
    BallY = 5,
    BallSize = 6,
    Count = 7,
}

impl SnapshotField {
    const fn idx(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::TUNING_STATUS_UNKNOWN_PARAM;

    const EPS: f32 = 0.001;

    fn approx_eq(left: f32, right: f32) {
        assert!(
            (left - right).abs() <= EPS,
            "left={} right={} diff={}",
            left,
            right,
            (left - right).abs()
        );
    }

    fn new_world() -> World {
        let (world, _schedule, _snapshot, _tuning_api) = build_world(800.0, 600.0);
        world
    }

    #[test]
    fn ball_radius_update_changes_collider() {
        let mut world = new_world();
        let ball = world.resource::<ArkanoidState>().ball;

        set_tuning_param(&mut world, ArkanoidTuningParam::BallRadius as u32, 12.0);

        approx_eq(world.collider(ball).radius, 12.0);
    }

    #[test]
    fn ball_speed_update_preserves_direction_and_sets_magnitude() {
        let mut world = new_world();
        let ball = world.resource::<ArkanoidState>().ball;
        let velocity_before = *world.velocity(ball);
        let before_speed = velocity_before.x.hypot(velocity_before.y);
        let dir_x = velocity_before.x / before_speed;
        let dir_y = velocity_before.y / before_speed;

        set_tuning_param(&mut world, ArkanoidTuningParam::BallSpeed as u32, 900.0);

        let velocity_after = *world.velocity(ball);
        approx_eq(velocity_after.x.hypot(velocity_after.y), 900.0);
        approx_eq(velocity_after.x / 900.0, dir_x);
        approx_eq(velocity_after.y / 900.0, dir_y);
    }

    #[test]
    fn out_of_range_values_are_clamped() {
        let mut world = new_world();

        set_tuning_param(&mut world, ArkanoidTuningParam::PaddleHeight as u32, 2.0);
        approx_eq(
            get_tuning_param(&world, ArkanoidTuningParam::PaddleHeight as u32).unwrap(),
            MIN_PADDLE_HEIGHT,
        );

        set_tuning_param(&mut world, ArkanoidTuningParam::BallSpeed as u32, 9000.0);
        approx_eq(
            get_tuning_param(&world, ArkanoidTuningParam::BallSpeed as u32).unwrap(),
            MAX_BALL_SPEED,
        );

        set_tuning_param(
            &mut world,
            ArkanoidTuningParam::BounceZone0Angle as u32,
            1.0,
        );
        approx_eq(
            get_tuning_param(&world, ArkanoidTuningParam::BounceZone0Angle as u32).unwrap(),
            MIN_ZONE_ANGLE,
        );
    }

    #[test]
    fn non_finite_values_are_rejected_without_mutation() {
        let mut world = new_world();
        let before = get_tuning_param(&world, ArkanoidTuningParam::PaddleSpeed as u32).unwrap();

        let status_nan = set_tuning_param(
            &mut world,
            ArkanoidTuningParam::PaddleSpeed as u32,
            f32::NAN,
        );
        let status_inf = set_tuning_param(
            &mut world,
            ArkanoidTuningParam::PaddleSpeed as u32,
            f32::INFINITY,
        );

        assert_eq!(status_nan, TUNING_STATUS_REJECTED);
        assert_eq!(status_inf, TUNING_STATUS_REJECTED);
        approx_eq(
            get_tuning_param(&world, ArkanoidTuningParam::PaddleSpeed as u32).unwrap(),
            before,
        );
    }

    #[test]
    fn unknown_parameter_returns_unknown_status() {
        let mut world = new_world();
        let status = set_tuning_param(&mut world, 1000, 1.0);
        assert_eq!(status, TUNING_STATUS_UNKNOWN_PARAM);
        assert_eq!(get_tuning_param(&world, 1000), None);
    }

    #[test]
    fn reset_restores_defaults() {
        let mut world = new_world();
        set_tuning_param(&mut world, ArkanoidTuningParam::BallRadius as u32, 20.0);
        set_tuning_param(&mut world, ArkanoidTuningParam::PaddleWidth as u32, 140.0);

        reset_tuning_defaults(&mut world);

        approx_eq(
            get_tuning_param(&world, ArkanoidTuningParam::BallRadius as u32).unwrap(),
            BALL_RADIUS,
        );
        approx_eq(
            get_tuning_param(&world, ArkanoidTuningParam::PaddleWidth as u32).unwrap(),
            PADDLE_WIDTH,
        );
    }
}
