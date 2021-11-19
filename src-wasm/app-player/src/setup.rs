use crate::{FacingDirection, GroundIntersections, Player, PlayerState, PlayerStateEnum};
use app_config::{RAPIER_GRAVITY_VECTOR, RAPIER_SCALE};
use bevy::{prelude::*, sprite::TextureAtlasBuilder};
use bevy_rapier::{
    physics::{RapierConfiguration, RigidBodyBundle},
    prelude::*,
};

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
    let texture_atlas_texture = texture_atlas.texture.clone();
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(RigidBodyBundle {
            position: [0., 10.].into(),
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_x, collider_size_y),
            mass_properties: ColliderMassProps::MassProperties(Box::new(
                MassProperties::from_ball(10., 10.),
            )),
            material: ColliderMaterial {
                friction_combine_rule: CoefficientCombineRule::Multiply,
                ..Default::default()
            },
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
                state: PlayerStateEnum::Jump {
                    tick: 0,
                    high_jump_tick: 0,
                    impulse: false,
                    released: true,
                },
                is_touching_ground: None,
                is_running: false,
                is_dashing: false,
                is_stooping: false,
                is_dash_turning: false,
            },
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(GroundIntersections::default())
        .insert(Timer::from_seconds(1.3, true));

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_atlas_texture.into()),
        transform: Transform::from_xyz(-300.0, 0.0, 0.0),
        ..Default::default()
    });
}
