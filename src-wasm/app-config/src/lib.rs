use bevy_rapier::na::Vector2;

pub const MOVE_IMPULSE_MULTIPLIER: f32 = 12000.;
pub const MOVE_DELTA_MULTIPLIER_AIR: f32 = 80.;

pub const JUMP_FORCE: f32 = 80.;

pub const RAPIER_SCALE: f32 = 10.;
pub const RAPIER_GRAVITY: Vector2<f32> = Vector2::new(0., -160.);
