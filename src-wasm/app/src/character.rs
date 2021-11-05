use crate::ground::Grounds;
use bevy::{prelude::*, sprite::TextureAtlasBuilder};
use bevy_rapier::{
    na::Vector2,
    physics::{ColliderBundle, ColliderPositionSync, RapierConfiguration, RigidBodyBundle},
    prelude::*,
};

pub fn setup_character(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    rapier_config.scale = 10.0;

    let scale_size = 2.;
    let sprite_size_x = scale_size * 12.0;
    let sprite_size_y = scale_size * 16.0;
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    let wait_handle = assets.load("MW_Player_MarioMdl_wait.0_0.png");
    let texture = textures.get(&wait_handle).unwrap();
    texture_atlas_builder.add_texture(wait_handle.clone(), texture);

    let handle = assets.load("MW_Player_MarioMdl_walk.0_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_walk.1_0.png");
    let texture = textures.get(&handle).unwrap();
    texture_atlas_builder.add_texture(handle, texture);

    let handle = assets.load("MW_Player_MarioMdl_jump.0_0.png");
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
            ..Default::default()
        })
        .insert_bundle(SpriteSheetBundle {
            transform: Transform {
                scale: Vec3::new(scale_size, scale_size, 1.),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(wait_index as u32),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Player {
            state: PlayerState::Jump,
        })
        .insert(ColliderPositionSync::Discrete);

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_atlas_texture.into()),
        transform: Transform::from_xyz(-300.0, 0.0, 0.0),
        ..Default::default()
    });
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Clone, Debug)]
pub enum PlayerState {
    Wait,
    Walk,
    Jump,
}

pub struct PlayerStateChangeEvent {
    pub state: PlayerState,
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut query: Query<(&Player, &mut RigidBodyVelocity, &RigidBodyMassProps)>,
) {
    for (_, mut rb_vels, rb_mprops) in query.iter_mut() {
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;

        let mut move_delta = Vector2::new(x_axis as f32, 0.);
        if move_delta != Vector2::zeros() {
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        rb_vels.apply_impulse(rb_mprops, move_delta * 400.);
    }
}

pub fn set_sprite(
    mut query: Query<(&Player, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    mut events: EventReader<PlayerStateChangeEvent>,
    assets: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    if let Ok((_, mut sprite, atlas_handle)) = query.single_mut() {
        for event in events.iter() {
            let texture_atlas = texture_atlases.get(atlas_handle).unwrap();
            let asset_path = match event.state {
                PlayerState::Wait => "MW_Player_MarioMdl_wait.0_0.png",
                PlayerState::Walk => "MW_Player_MarioMdl_walk.0_0.png",
                PlayerState::Jump => "MW_Player_MarioMdl_jump.0_0.png",
            };
            let handle = assets.load(asset_path);
            let index = texture_atlas.get_texture_index(&handle).unwrap();
            sprite.index = index as u32;
        }
    }
}

pub fn walk_animation() {}

pub fn ground_intersect(
    mut query: Query<(&mut Player, Entity, &RigidBodyVelocity)>,
    grounds: Res<Grounds>,
    mut psc_event: EventWriter<PlayerStateChangeEvent>,
    mut intersection_events: EventReader<IntersectionEvent>,
) {
    if let Ok((mut player, player_entity, rb_vel)) = query.single_mut() {
        for intersection_event in intersection_events.iter() {
            match intersection_event {
                IntersectionEvent {
                    collider1,
                    collider2,
                    intersecting: true,
                    ..
                } => {
                    if !grounds.contains(&collider1.entity())
                        && !grounds.contains(&collider2.entity())
                    {
                        return;
                    }
                    if collider1.entity() == player_entity || collider2.entity() == player_entity {
                        if rb_vel
                            .linvel
                            .data
                            .0
                            .get(0)
                            .map(|x| x[0] == 0.)
                            .unwrap_or_default()
                        {
                            player.state = PlayerState::Wait;
                            psc_event.send(PlayerStateChangeEvent {
                                state: PlayerState::Wait,
                            });
                        } else {
                            player.state = PlayerState::Walk;
                            psc_event.send(PlayerStateChangeEvent {
                                state: PlayerState::Walk,
                            });
                        };
                    }
                }
                IntersectionEvent {
                    collider1,
                    collider2,
                    intersecting: false,
                } => {
                    if !grounds.contains(&collider1.entity())
                        && !grounds.contains(&collider2.entity())
                    {
                        return;
                    }
                    if collider1.entity() == player_entity || collider2.entity() == player_entity {
                        player.state = PlayerState::Jump;
                        psc_event.send(PlayerStateChangeEvent {
                            state: PlayerState::Jump,
                        });
                    }
                }
            }
        }
    }
}
