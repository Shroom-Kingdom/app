use crate::{
    grid_to_world, grid_to_world_f32, Course, DespawnTileEvent, DragEventFlags, Draggable,
    GroundVariant, ObjectSpriteHandles, ObjectVariant, TileVariant,
};
use app_config::*;
use bevy::prelude::*;
use bevy_rapier::prelude::*;

#[derive(Component)]
pub struct GoalPole(i32);

pub struct GoalPoleDragEvent {
    pub grid_pos: [i32; 2],
}

pub struct RespawnGoalPoleEvent;

impl Course {
    pub fn spawn_goal(
        &mut self,
        commands: &mut Commands,
        asset_server: &AssetServer,
        object_sprite_handles: &ObjectSpriteHandles,
    ) {
        for x in (self.goal_pos_x + 1)..(self.goal_pos_x + MAX_COURSE_GOAL_OFFSET_X) {
            self.spawn_tile(
                commands,
                &[x, 0],
                &TileVariant::Ground(GroundVariant::Full0),
                None,
                Some([[true, true, true], [true, false, true], [true, true, true]]),
                false,
            );
            self.spawn_tile(
                commands,
                &[x, 1],
                &TileVariant::Ground(GroundVariant::Top0),
                None,
                Some([
                    [
                        false,
                        false,
                        x == self.goal_pos_x + MAX_COURSE_GOAL_OFFSET_X - 1,
                    ],
                    [true, false, true],
                    [true, true, true],
                ]),
                false,
            );
        }
        self.spawn_tile(
            commands,
            &[self.goal_pos_x, 0],
            &TileVariant::Ground(GroundVariant::Left0),
            None,
            Some([
                [false, true, true],
                [false, false, true],
                [true, true, true],
            ]),
            false,
        );
        self.spawn_tile(
            commands,
            &[self.goal_pos_x, 1],
            &TileVariant::Ground(GroundVariant::TopLeft0),
            None,
            Some([
                [false, false, false],
                [false, false, true],
                [false, true, true],
            ]),
            false,
        );

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
                flags: DragEventFlags::ONLY_HORIZONTAL,
            });
    }

    pub fn despawn_goal(
        &mut self,
        query: Query<Entity, (With<GoalPole>, Without<Draggable>)>,
        mut commands: Commands,
        mut despawn_tile_events: EventWriter<DespawnTileEvent>,
    ) {
        despawn_tile_events.send_batch(
            (self.goal_pos_x..(self.goal_pos_x + MAX_COURSE_GOAL_OFFSET_X)).map(|x| {
                DespawnTileEvent {
                    grid_pos: [x, 0],
                    force: true,
                }
            }),
        );
        despawn_tile_events.send_batch(
            (self.goal_pos_x..(self.goal_pos_x + MAX_COURSE_GOAL_OFFSET_X)).map(|x| {
                DespawnTileEvent {
                    grid_pos: [x, 1],
                    force: true,
                }
            }),
        );
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn move_goal_pole(
    query: Query<Entity, (With<GoalPole>, Without<Draggable>)>,
    commands: Commands,
    mut drag_events: EventReader<GoalPoleDragEvent>,
    despawn_tile_events: EventWriter<DespawnTileEvent>,
    mut respawn_events: EventWriter<RespawnGoalPoleEvent>,
    mut course: ResMut<Course>,
) {
    if let Some(GoalPoleDragEvent { grid_pos }) = drag_events.iter().next() {
        course.despawn_goal(query, commands, despawn_tile_events);
        course.goal_pos_x = grid_pos[0];

        respawn_events.send(RespawnGoalPoleEvent);
    }
}

pub fn respawn_goal_pole(
    mut commands: Commands,
    mut course: ResMut<Course>,
    asset_server: Res<AssetServer>,
    object_sprite_handles: Res<ObjectSpriteHandles>,
    mut respawn_events: EventReader<RespawnGoalPoleEvent>,
) {
    for _ in respawn_events.iter() {
        course.spawn_goal(&mut commands, &asset_server, &object_sprite_handles);
    }
}