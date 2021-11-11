use bevy_rapier::na::Vector2;

pub const MOVE_IMPULSE_MULTIPLIER_GROUND: f32 = 20000.;
pub const MOVE_IMPULSE_MULTIPLIER_AIR: f32 = 30000.;

pub const LINVEL_CAP_GROUND: (f32, f32) = (17.5, 100.);
pub const LINVEL_CAP_AIR: (f32, f32) = (100., 65.);

pub const JUMP_FORCE: f32 = 150.;
pub const MAX_JUMP_TICK: u8 = 10;

pub const RAPIER_SCALE: f32 = 10.;
pub const RAPIER_GRAVITY: Vector2<f32> = Vector2::new(0., -320.);
