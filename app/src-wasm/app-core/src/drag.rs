use crate::{
    cursor_to_world, grid_to_world, world_to_grid, world_to_grid_pos, GoalPole,
    GoalPoleDragDirection, GoalPoleDragEvent, GoalPoleDragTimer, MainCameraQuery,
};
use app_config::{MAX_GOAL_POS_X, MIN_GOAL_POS_X};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier::prelude::*;

#[derive(Component, Default)]
pub struct Draggable {
    pub flags: DragEventFlags,
}

pub struct Dragging {
    entity: Entity,
    flags: DragEventFlags,
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct DragEventFlags: u32 {
        const ONLY_HORIZONTAL = 0b0001;
        const ONLY_VERTICAL = 0b0010;
        const WITHOUT_MOUSE_MOTION = 0b0100;
    }
}

pub struct DragEvent {
    entity: Entity,
    grid_pos: [i32; 2],
}

pub fn drag_mouse_motion(
    query: Query<&Transform, Without<Camera>>,
    camera_query: MainCameraQuery,
    dragging: Res<Option<Dragging>>,
    windows: Res<Windows>,
    mut motion_event: EventReader<MouseMotion>,
    mut drag_events: EventWriter<DragEvent>,
) {
    if let Some(Dragging { entity, flags }) = *dragging {
        if let Ok(transform) = query.get(entity) {
            let window = windows.get_primary().unwrap();
            let cursor_position = if let Some(cursor_pointer) = window.cursor_position() {
                cursor_pointer
            } else {
                return;
            };
            if flags & DragEventFlags::WITHOUT_MOUSE_MOTION != DragEventFlags::WITHOUT_MOUSE_MOTION
                && motion_event.iter().next().is_none()
            {
                return;
            }
            let world_pos = cursor_to_world(cursor_position, &camera_query, window);
            let mut grid_pos = world_to_grid(&world_pos);
            let grid_pos_entity = world_to_grid(&transform.translation.truncate().into());

            if flags & DragEventFlags::ONLY_HORIZONTAL == DragEventFlags::ONLY_HORIZONTAL {
                grid_pos[1] = grid_pos_entity[1];
            }
            if flags & DragEventFlags::ONLY_VERTICAL == DragEventFlags::ONLY_VERTICAL {
                grid_pos[0] = grid_pos_entity[0];
            }

            if grid_pos != grid_pos_entity {
                drag_events.send(DragEvent { entity, grid_pos });
            }
        }
    }
}

pub fn drag_mouse_button(
    draggable_query: Query<&Draggable>,
    camera_query: MainCameraQuery,
    ctx: Res<RapierContext>,
    mut dragging: ResMut<Option<Dragging>>,
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
                let Draggable { flags } = draggable_query.get(entity).unwrap();
                *dragging = Some(Dragging {
                    entity,
                    flags: *flags,
                });
                false
            },
        );
    } else if mouse.just_released(MouseButton::Left) {
        *dragging = None;
    }
}

pub fn handle_drag_events(
    mut query: Query<&mut Transform>,
    mut goal_pole_query: Query<&mut GoalPoleDragTimer, With<GoalPole>>,
    mut drag_events: EventReader<DragEvent>,
    mut goal_pole_drag_events: EventWriter<GoalPoleDragEvent>,
    time: Res<Time>,
) {
    for DragEvent { entity, grid_pos } in drag_events.iter() {
        if let Ok(mut transform) = query.get_mut(*entity) {
            let world_pos = grid_to_world(grid_pos);

            if let Ok(mut timer) = goal_pole_query.get_mut(*entity) {
                timer.tick(time.delta());
                if !timer.finished() {
                    return;
                }
                let old_grid_pos = world_to_grid_pos(transform.translation.x);
                if old_grid_pos == grid_pos[0] {
                    return;
                }
                let direction = if old_grid_pos > grid_pos[0] {
                    GoalPoleDragDirection::Left
                } else {
                    GoalPoleDragDirection::Right
                };
                match direction {
                    GoalPoleDragDirection::Left if old_grid_pos - 1 < MIN_GOAL_POS_X => return,
                    GoalPoleDragDirection::Right if old_grid_pos + 1 > MAX_GOAL_POS_X => return,
                    _ => {}
                };
                timer.reset();
                goal_pole_drag_events.send(GoalPoleDragEvent { direction });
            } else {
                transform.translation = world_pos.extend(0.);
            }
        }
    }
}
