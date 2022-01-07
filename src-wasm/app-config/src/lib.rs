use bevy_rapier::na::Vector2;

pub const MOVE_IMPULSE_MULTIPLIER_GROUND: f32 = 0.2 * GROUND_FRICTION_MULTIPLIER * RAPIER_GRAVITY;
pub const MOVE_IMPULSE_MULTIPLIER_GROUND_RUN: f32 =
    0.22 * GROUND_FRICTION_MULTIPLIER * RAPIER_GRAVITY;
pub const MOVE_IMPULSE_MULTIPLIER_AIR: f32 = 2700.;
pub const MOVE_IMPULSE_MULTIPLIER_AIR_RUN: f32 = 2800.;

pub const LINVEL_CAP_WALK: f32 = 20.;
pub const LINVEL_CAP_RUN: f32 = 35.;
pub const LINVEL_CAP_STOOP: f32 = 10.;

pub const PLAYER_COLLIDER_BORDER_RADIUS: f32 = 0.4;
pub const COLLIDER_MIN_TOI: f32 = 1. / 60.;
pub const COLLIDER_TOI_THRESHOLD: f32 = 1.2 / 60.;

pub const RUN_THRESHOLD: f32 = LINVEL_CAP_RUN - 4.5;
pub const HIGH_JUMP_WALK_THRESHOLD: f32 = 7.;

pub const JUMP_FORCE: f32 = 37.;
pub const HIGH_JUMP_TICK: u8 = 15;
pub const HIGH_JUMP_TICK_WALK: u8 = 22;

pub const RAPIER_SCALE: f32 = 10.;
pub const RAPIER_GRAVITY: f32 = 1000.;
pub const RAPIER_GRAVITY_VECTOR: Vector2<f32> = Vector2::new(0., -RAPIER_GRAVITY);

const GROUND_FRICTION_MULTIPLIER: f32 = 30.;
pub const GROUND_FRICTION: f32 = GROUND_FRICTION_MULTIPLIER * 150. / RAPIER_GRAVITY;
pub const GROUND_FRICTION_MIN_VEL: f32 = 1.5;
pub const GROUND_FRICTION_STATIC_MULTIPLIER: f32 = 2.5 / 60.;
pub const GROUND_FRICTION_KINETIC_MULTIPLIER: f32 = 0.3 / 60.;

pub const TILE_SIZE: f32 = 2.;
pub const TILE_COLLIDER_SUB: f32 = 0.5;
pub const GRID_MARGIN: f32 = 0.1;
pub const GRID_SIZE: f32 = TILE_SIZE * 2. - (2. * TILE_COLLIDER_SUB) + (2. * GRID_MARGIN);
pub const GROUND_MARGIN_MULTIPLIER: f32 = 2.;

pub const CAMERA_MIN_X: f32 = 620.;
pub const CAMERA_MIN_Y: f32 = 340.;
