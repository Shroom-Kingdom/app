use crate::{
    FacingDirection, GroundIntersections, Player, PlayerState, PlayerStateEnum, PlayerVelocity,
};
use app_config::{PLAYER_COLLIDER_BORDER_RADIUS, RAPIER_GRAVITY_VECTOR, RAPIER_SCALE};
use app_core::grid_to_world;
use bevy::{prelude::*, sprite::TextureAtlasBuilder};
use bevy_rapier::{
    physics::{RapierConfiguration, RigidBodyBundle},
    prelude::*,
};

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    rapier_config.scale = RAPIER_SCALE;
    rapier_config.gravity = RAPIER_GRAVITY_VECTOR;

    let scale_size = 2.;
    let sprite_size_x = scale_size * 7.0;
    let sprite_size_y = scale_size * 10.0;
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    let handle = assets.load("MW_Player_MarioMdl_wait.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle.clone(), texture);

    let handle = assets.load("MW_Player_MarioMdl_walk.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_walk.1_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_stoop.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_b_dash.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_b_dash.1_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_b_dash_jump.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_b_dash_jump_fall.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_turn.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_jump.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_jump_fall.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle.clone(), texture);

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let wait_index = texture_atlas.get_texture_index(&handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    let world_pos = grid_to_world(&[5, 2]);

    commands
        .spawn_bundle(RigidBodyBundle {
            position: world_pos.into(),
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                ..Default::default()
            },
            body_type: RigidBodyType::KinematicVelocityBased,
            ..Default::default()
        })
        .insert(PlayerVelocity::default())
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::round_cuboid(
                collider_size_x - PLAYER_COLLIDER_BORDER_RADIUS,
                collider_size_y - PLAYER_COLLIDER_BORDER_RADIUS,
                PLAYER_COLLIDER_BORDER_RADIUS,
            ),
            mass_properties: ColliderMassProps::MassProperties(Box::new(
                MassProperties::from_ball(10., 10.),
            )),
            material: ColliderMaterial {
                friction_combine_rule: CoefficientCombineRule::Multiply,
                ..Default::default()
            },
            flags: ActiveEvents::CONTACT_EVENTS.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    scale: Vec3::new(scale_size, scale_size, 1.),
                    translation: Vec3::new(0., 6. * scale_size, 1.),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(wait_index as u32),
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
        .insert(ColliderPositionSync::Discrete)
        .insert(GroundIntersections::default())
        .insert(Timer::from_seconds(1.3, true));
}
