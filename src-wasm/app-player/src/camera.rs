use crate::Player;
use app_config::{CAMERA_MIN_X, RAPIER_SCALE};
use bevy::{
    prelude::*,
    render::{camera::Camera, primitives::Frustum},
};
use bevy_rapier::prelude::*;

pub fn position_camera(
    mut query: Query<&mut Transform, (With<Camera>, With<Frustum>)>,
    // player_query: Query<&Transform, (With<Player>, With<RigidBody>)>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        // if let Ok(rb_transform) = player_query.get_single() {
        //     transform.translation.x = rb_transform.translation.x * RAPIER_SCALE;
        //     if transform.translation.x < CAMERA_MIN_X {
        //         transform.translation.x = CAMERA_MIN_X;
        //     }
        // }
    }
}
