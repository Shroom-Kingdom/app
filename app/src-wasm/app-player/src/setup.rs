use crate::{GroundIntersections, Player, PlayerState, PlayerVelocity, WalkAnimationTimer};
use app_config::*;
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
    let collider_size_x = sprite_size_x * 0.72;
    let collider_size_y = sprite_size_y * 0.78;

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

    let world_pos = grid_to_world(&[5, 6]);

    commands
        .spawn()
        .insert(RigidBody::KinematicVelocityBased)
        .insert_bundle(SpatialBundle {
            transform: Transform::from_xyz(world_pos.x, world_pos.y, Z_INDEX_PLAYER),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::default())
        .insert(PlayerVelocity::default())
        .insert(ReadMassProperties(MassProperties::from_rapier(
            bevy_rapier::rapier::prelude::MassProperties::from_ball(100., 100.),
            RAPIER_SCALE,
        )))
        .insert(Friction {
            combine_rule: CoefficientCombineRule::Multiply,
            ..Default::default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0., 6. * scale_size, 0.),
                    scale: Vec3::new(scale_size, scale_size, 1.),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(wait_index),
                texture_atlas: atlas_handle,
                ..Default::default()
            });
        })
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::round_cuboid(
                    collider_size_x - PLAYER_COLLIDER_BORDER_RADIUS,
                    collider_size_y - PLAYER_COLLIDER_BORDER_RADIUS,
                    PLAYER_COLLIDER_BORDER_RADIUS,
                ))
                .insert(Transform::default());
        })
        .insert(Player {
            state: PlayerState::default(),
        })
        .insert(GroundIntersections::default())
        .insert(WalkAnimationTimer(Timer::from_seconds(13., true)));
}
