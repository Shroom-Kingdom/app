use crate::Player;
use app_config::*;
use app_core::{pos_to_world, Course, GoalPoleDragDirection, GoalPoleDragEvent};
use bevy::{
    prelude::*,
    render::{camera::Camera, primitives::Frustum},
};

pub fn position_camera(
    mut query: Query<&mut Transform, (With<Camera>, With<Frustum>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    course: Res<Course>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        if let Ok(rb_transform) = player_query.get_single() {
            transform.translation.x = rb_transform.translation.x;
            if transform.translation.x < CAMERA_MIN_X {
                transform.translation.x = CAMERA_MIN_X;
            }
            let max_pos_x = pos_to_world(course.goal_pos_x);
            if transform.translation.x > max_pos_x {
                transform.translation.x = max_pos_x;
            }
        }
    }
}

pub fn move_player_on_goal_pole_drag(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut drag_events: EventReader<GoalPoleDragEvent>,
    course: Res<Course>,
) {
    if let Some(GoalPoleDragEvent { direction }) = drag_events.iter().next() {
        let pos_diff = pos_to_world(match direction {
            GoalPoleDragDirection::Left => -1,
            GoalPoleDragDirection::Right => 1,
        });
        if let Ok(mut transform) = player_query.get_single_mut() {
            transform.translation.x += pos_diff;
            if transform.translation.x < CAMERA_MIN_X {
                transform.translation.x = CAMERA_MIN_X;
            }
            let max_pos_x = pos_to_world(course.goal_pos_x);
            if transform.translation.x > max_pos_x {
                transform.translation.x = max_pos_x;
            }
        }
    }
}
