use crate::{
    FacingDirection, GroundIntersections, Player, PlayerState, PlayerStateEnum, PlayerVelocity,
    WalkAnimationTimer,
};
use app_config::{PLAYER_COLLIDER_BORDER_RADIUS, RAPIER_GRAVITY, RAPIER_SCALE};
use app_core::{grid_to_world, PlayerFrame, PlayerSpriteHandles};
use bevy::{prelude::*, sprite::TextureAtlasBuilder};
use bevy_rapier::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut textures: ResMut<Assets<Image>>,
    player_sprite_handles: Res<PlayerSpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    rapier_config.gravity = Vec2::new(0., -RAPIER_GRAVITY);

    let scale_size = 2.;
    let sprite_size_x = scale_size * 7.0;
    let sprite_size_y = scale_size * 10.0;
    let collider_size_x = sprite_size_x / RAPIER_SCALE;
    let collider_size_y = sprite_size_y / RAPIER_SCALE;

    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    for (_, handle) in player_sprite_handles.0.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let wait_index = texture_atlas
        .get_texture_index(player_sprite_handles.0.get(&PlayerFrame::Wait).unwrap())
        .unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    let world_pos = grid_to_world(&[5, 2]);

    commands
        .spawn()
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Transform::from_xyz(world_pos.x, world_pos.y, 0.))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(PlayerVelocity::default())
        .insert(Collider::round_cuboid(
            collider_size_x - PLAYER_COLLIDER_BORDER_RADIUS,
            collider_size_y - PLAYER_COLLIDER_BORDER_RADIUS,
            PLAYER_COLLIDER_BORDER_RADIUS,
        ))
        .insert(MassProperties::from_rapier(
            bevy_rapier::rapier::prelude::MassProperties::from_ball(10., 10.),
            RAPIER_SCALE,
        ))
        .insert(Friction {
            combine_rule: CoefficientCombineRule::Multiply,
            ..Default::default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    scale: Vec3::new(scale_size, scale_size, 1.),
                    translation: Vec3::new(0., 6. * scale_size, 0.),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(wait_index),
                texture_atlas: atlas_handle,
                ..Default::default()
            });
        })
        .insert(Player {
            state: PlayerState {
                facing_direction: FacingDirection::Right,
                state: PlayerStateEnum::Air {
                    tick: 0,
                    high_jump_tick: 0,
                    impulse: false,
                    released: true,
                    fall: true,
                },
                is_running: false,
                is_dashing: false,
                is_stooping: false,
                is_dash_turning: false,
            },
        })
        .insert(GroundIntersections::default())
        .insert(WalkAnimationTimer(Timer::from_seconds(1.3, true)));
}
