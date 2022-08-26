use crate::{
    grid_to_world_f32, pos_to_world, Course, GroundVariant, ObjectSpriteHandles, ObjectVariant,
    TileVariant,
};
use app_config::MAX_COURSE_GOAL_OFFSET_X;
use bevy::prelude::*;

#[derive(Component)]
pub struct GoalPole(i32);

pub enum MoveDirection {
    Left,
    Right,
}

impl Course {
    pub fn spawn_goal(
        &mut self,
        commands: &mut Commands,
        object_sprite_handles: Res<ObjectSpriteHandles>,
        pos_x: i32,
    ) {
        for x in (pos_x + 1)..(pos_x + MAX_COURSE_GOAL_OFFSET_X) {
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
                    [false, false, x == pos_x + MAX_COURSE_GOAL_OFFSET_X - 1],
                    [true, false, true],
                    [true, true, true],
                ]),
                false,
            );
        }
        self.spawn_tile(
            commands,
            &[pos_x, 0],
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
            &[pos_x, 1],
            &TileVariant::Ground(GroundVariant::TopLeft0),
            None,
            Some([
                [false, false, false],
                [false, false, true],
                [false, true, true],
            ]),
            false,
        );

        let world_pos = grid_to_world_f32(&[pos_x as f32, 5.5]);
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
                    translation: Vec3::new(world_pos.x, world_pos.y, -0.5),
                    scale: Vec3::new(2., 2., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GoalPole(pos_x));

        let world_pos = grid_to_world_f32(&[pos_x as f32 + 2., 5.5]);
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
                    translation: Vec3::new(world_pos.x, world_pos.y, 0.),
                    scale: Vec3::new(2., 2., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GoalPole(pos_x + 2));

        let world_pos = grid_to_world_f32(&[pos_x as f32 + 1., 5.5]);
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
                    translation: Vec3::new(world_pos.x, world_pos.y, -0.1),
                    scale: Vec3::new(2., 2., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(GoalPole(pos_x + 1));
    }
}

pub fn move_goal_pole(mut query: Query<(&mut Transform, &GoalPole)>, direction: MoveDirection) {
    for (mut transform, goal_pole) in query.iter_mut() {
        transform.translation.x = pos_to_world(match direction {
            MoveDirection::Left => goal_pole.0 - 1,
            MoveDirection::Right => goal_pole.0 + 1,
        })
    }
}
