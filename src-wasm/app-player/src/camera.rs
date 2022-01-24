use crate::Player;
use app_config::{CAMERA_MIN_X, RAPIER_SCALE};
use bevy::{
    prelude::*,
    render::{camera::Camera, primitives::Frustum},
};
use bevy_rapier::prelude::*;

pub fn position_camera(
    mut query: Query<&mut Transform, (With<Camera>, With<Frustum>)>,
    player_query: Query<&RigidBodyPositionComponent, With<Player>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        if let Ok(rb_pos) = player_query.get_single() {
            transform.translation.x =
                rb_pos.position.translation.vector.data.0[0][0] * RAPIER_SCALE;
            if transform.translation.x < CAMERA_MIN_X {
                transform.translation.x = CAMERA_MIN_X;
            }
        }
    }
}
