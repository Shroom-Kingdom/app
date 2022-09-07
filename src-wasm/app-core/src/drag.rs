use crate::{cursor_to_world, MainCameraQuery};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier::prelude::*;

#[derive(Component)]
pub struct Draggable;

#[derive(Default)]
pub struct Dragging(pub Option<Entity>);

pub fn drag_mouse_motion(
    mut query: Query<&mut Transform, Without<Camera>>,
    dragging: Res<Dragging>,
    mut motion_event: EventReader<MouseMotion>,
    camera_query: MainCameraQuery,
    windows: Res<Windows>,
) {
    if let Some(entity) = dragging.0 {
        if let Ok(mut transform) = query.get_mut(entity) {
            let window = windows.get_primary().unwrap();
            let cursor_position = if let Some(cursor_pointer) = window.cursor_position() {
                cursor_pointer
            } else {
                return;
            };
            if motion_event.iter().next().is_none() {
                return;
            }
            let world_pos = cursor_to_world(cursor_position, &camera_query, window);
            transform.translation = Vec2::from(world_pos).extend(0.);
        }
    }
}

pub fn drag_mouse_button(
    draggable_query: Query<With<Draggable>>,
    camera_query: MainCameraQuery,
    ctx: Res<RapierContext>,
    mut dragging: ResMut<Dragging>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let cursor_position = if let Some(cursor_pointer) = window.cursor_position() {
        cursor_pointer
    } else {
        return;
    };

    if mouse.just_pressed(MouseButton::Left) {
        let world_pos = cursor_to_world(cursor_position, &camera_query, window);

        ctx.intersections_with_point(
            world_pos.into(),
            QueryFilter {
                predicate: Some(&|entity| draggable_query.get(entity).is_ok()),
                ..Default::default()
            },
            |entity| {
                dragging.0 = Some(entity);
                false
            },
        );
    } else if mouse.just_released(MouseButton::Left) {
        dragging.0 = None;
    }
}
