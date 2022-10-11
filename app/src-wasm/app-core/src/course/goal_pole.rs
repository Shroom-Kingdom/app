use std::time::Duration;

use crate::{
    grid_to_world, grid_to_world_f32, pos_to_world, CourseRes, DespawnTileEvent, DragEventFlags,
    Draggable, GroundTileUpdateEvent, GroundVariant, ObjectSpriteHandles, ObjectVariant,
    TileVariant,
};
use app_config::*;
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier::prelude::*;

#[derive(Component)]
pub struct GoalPole(i32);

pub struct GoalPoleDragEvent {
    pub direction: GoalPoleDragDirection,
}

pub enum GoalPoleDragDirection {
    Left,
    Right,
}

#[derive(Component, Deref, DerefMut)]
pub struct GoalPoleDragTimer(pub Timer);

pub struct RespawnGoalPoleEvent;

impl CourseRes {
    pub fn spawn_goal(
        &mut self,
        commands: &mut Commands,
        object_sprite_handles: &ObjectSpriteHandles,
        ground_tile_update_events: &mut EventWriter<GroundTileUpdateEvent>,
    ) {
        let mut events = HashMap::new();
        for x in (self.goal_pos_x + 1)..(self.goal_pos_x + MAX_COURSE_GOAL_OFFSET_X) {
            self.spawn_tile(
                commands,
                &[x, 0],
                &TileVariant::Ground(GroundVariant::Full0),
                &mut events,
            );
            self.spawn_tile(
                commands,
                &[x, 1],
                &TileVariant::Ground(GroundVariant::Top0),
                &mut events,
            );
        }
        self.spawn_tile(
            commands,
            &[self.goal_pos_x, 0],
            &TileVariant::Ground(GroundVariant::Left0),
            &mut events,
        );
        self.spawn_tile(
            commands,
            &[self.goal_pos_x, 1],
            &TileVariant::Ground(GroundVariant::TopLeft0),
            &mut events,
        );
        for event in events.into_values() {
            ground_tile_update_events.send(event);
        }

        let world_pos = grid_to_world_f32(&[self.goal_pos_x as f32, 5.5]);
        let texture = object_sprite_handles
            .0
            .get(&ObjectVariant::GoalPoleL)
            .unwrap()
            .clone();
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(world_pos.x, world_pos.y, Z_INDEX_GOAL_L),
                    scale: Vec3::new(2., 2., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GoalPole(self.goal_pos_x));

        let world_pos = grid_to_world_f32(&[self.goal_pos_x as f32 + 2., 5.5]);
        let texture = object_sprite_handles
            .0
            .get(&ObjectVariant::GoalPoleR)
            .unwrap()
            .clone();
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(world_pos.x, world_pos.y, Z_INDEX_GOAL_R),
                    scale: Vec3::new(2., 2., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GoalPole(self.goal_pos_x + 2));

        let world_pos = grid_to_world_f32(&[self.goal_pos_x as f32 + 1., 5.5]);
        let texture = object_sprite_handles
            .0
            .get(&ObjectVariant::GoalPole)
            .unwrap()
            .clone();
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(world_pos.x, world_pos.y, Z_INDEX_GOAL),
                    scale: Vec3::new(2., 2., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GoalPole(self.goal_pos_x + 1));
    }

    pub(crate) fn spawn_goal_drag(&mut self, commands: &mut Commands, asset_server: &AssetServer) {
        let world_pos = grid_to_world(&[self.goal_pos_x + 1, 1]);
        let texture = asset_server.load("icons/leftright.png");
        commands
            .spawn()
            .insert(RigidBody::Fixed)
            .insert_bundle(SpatialBundle {
                transform: Transform::from_xyz(world_pos.x, world_pos.y, Z_INDEX_GOAL_DRAG),
                visibility: Visibility { is_visible: true },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    texture,
                    transform: Transform {
                        scale: Vec3::new(0.07, 0.07, 1.),
                        ..Default::default()
                    },
                    ..default()
                });
            })
            .insert(Collider::cuboid(
                TILE_GRID_SIZE * TILE_SIZE,
                TILE_GRID_SIZE * TILE_SIZE,
            ))
            .insert(Sensor)
            .insert(GoalPole(self.goal_pos_x + 1))
            .insert(Draggable {
                flags: DragEventFlags::ONLY_HORIZONTAL | DragEventFlags::WITHOUT_MOUSE_MOTION,
            })
            .insert(GoalPoleDragTimer(Timer::new(
                Duration::from_millis(100),
                true,
            )));
    }

    pub fn despawn_goal(
        &mut self,
        query: Query<Entity, (With<GoalPole>, Without<Draggable>)>,
        direction: &GoalPoleDragDirection,
        mut commands: Commands,
        mut despawn_tile_events: EventWriter<DespawnTileEvent>,
    ) {
        let pos_x = if let GoalPoleDragDirection::Left = direction {
            self.goal_pos_x - 1
        } else {
            self.goal_pos_x
        };
        despawn_tile_events.send_batch((pos_x..(self.goal_pos_x + MAX_COURSE_GOAL_OFFSET_X)).map(
            |x| DespawnTileEvent {
                grid_pos: [x, 0],
                force: true,
            },
        ));
        despawn_tile_events.send_batch((pos_x..(self.goal_pos_x + MAX_COURSE_GOAL_OFFSET_X)).map(
            |x| DespawnTileEvent {
                grid_pos: [x, 1],
                force: true,
            },
        ));
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn move_goal_pole(
    query: Query<Entity, (With<GoalPole>, Without<Draggable>)>,
    mut drag_query: Query<&mut Transform, (With<GoalPole>, With<Draggable>)>,
    commands: Commands,
    mut drag_events: EventReader<GoalPoleDragEvent>,
    despawn_tile_events: EventWriter<DespawnTileEvent>,
    mut respawn_events: EventWriter<RespawnGoalPoleEvent>,
    mut course: ResMut<CourseRes>,
) {
    if let Some(GoalPoleDragEvent { direction }) = drag_events.iter().next() {
        course.despawn_goal(query, direction, commands, despawn_tile_events);
        course.goal_pos_x += match direction {
            GoalPoleDragDirection::Left => -1,
            GoalPoleDragDirection::Right => 1,
        };
        if let Ok(mut transform) = drag_query.get_single_mut() {
            let world_pos = pos_to_world(course.goal_pos_x + 1);
            transform.translation.x = world_pos;
        }

        respawn_events.send(RespawnGoalPoleEvent);
    }
}

pub fn respawn_goal_pole(
    mut commands: Commands,
    mut course: ResMut<CourseRes>,
    object_sprite_handles: Res<ObjectSpriteHandles>,
    mut respawn_events: EventReader<RespawnGoalPoleEvent>,
    mut ground_tile_update_events: EventWriter<GroundTileUpdateEvent>,
) {
    if respawn_events.iter().next().is_some() {
        course.spawn_goal(
            &mut commands,
            &object_sprite_handles,
            &mut ground_tile_update_events,
        );
    }
}
