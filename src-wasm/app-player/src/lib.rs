mod debug;
mod ground;
mod walk;

use app_config::{JUMP_FORCE, MOVE_IMPULSE_MULTIPLIER, RAPIER_GRAVITY, RAPIER_SCALE};
use app_core::AppState;
use bevy::{prelude::*, sprite::TextureAtlasBuilder};
use bevy_rapier::{
    na::Vector2,
    physics::{ColliderBundle, ColliderPositionSync, RapierConfiguration, RigidBodyBundle},
    prelude::*,
};
use debug::setup_ui;
use walk::{walk_animation, walk_start};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerStateChangeEvent>()
            .add_startup_system(setup_ui)
            .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup_character))
            .add_system_to_stage(CoreStage::First, player_movement_cap)
            .add_system_to_stage(CoreStage::First, jump)
            .add_system_to_stage(CoreStage::PreUpdate, player_state_change)
            .add_system_to_stage(CoreStage::Update, player_movement)
            .add_system_to_stage(CoreStage::Update, set_sprite)
            .add_system_to_stage(CoreStage::PostUpdate, debug::text_update_system)
            .add_system_to_stage(CoreStage::PostUpdate, ground::ground_intersect)
            .add_system_to_stage(CoreStage::PostUpdate, player_momentum)
            .add_system_to_stage(CoreStage::Last, walk_animation)
            .add_system_to_stage(CoreStage::Last, walk_start);
    }
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Clone, Debug)]
pub enum PlayerState {
    Wait,
    Walk { frame: u8, linvel_x: Option<f32> },
    Jump { tick: u8, linvel_x: Option<f32> },
}

pub struct PlayerStateChangeEvent {
    pub state: PlayerState,
}

fn setup_character(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    rapier_config.scale = RAPIER_SCALE;
    rapier_config.gravity = RAPIER_GRAVITY;

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
            mass_properties: ColliderMassProps::MassProperties(Box::new(
                MassProperties::from_ball(10., 10.),
            )),
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
            state: PlayerState::Jump {
                tick: 0,
                linvel_x: None,
            },
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Timer::from_seconds(3., true));

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_atlas_texture.into()),
        transform: Transform::from_xyz(-300.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn player_state_change(
    mut query: Query<&mut Player>,
    mut psc_events: EventReader<PlayerStateChangeEvent>,
) {
    if let Ok(mut player) = query.single_mut() {
        for event in psc_events.iter() {
            player.state = event.state.clone();
        }
    }
}

fn player_momentum(
    mut query: Query<(&mut Player, &mut RigidBodyVelocity)>,
    mut psc_events: EventReader<PlayerStateChangeEvent>,
) {
    if let Ok((player, mut rb_vel)) = query.single_mut() {
        if psc_events.iter().next().is_some() {
            match player.state {
                PlayerState::Jump {
                    linvel_x: Some(linvel_x),
                    ..
                } => {
                    rb_vel.linvel.data.0[0][0] = linvel_x;
                }
                PlayerState::Walk {
                    linvel_x: Some(linvel_x),
                    ..
                } => {
                    rb_vel.linvel.data.0[0][0] = linvel_x;
                }
                _ => {}
            }
        }
    }
}

fn player_movement_cap(mut query: Query<(&Player, &mut RigidBodyVelocity)>) {
    for (player, mut rb_vel) in query.iter_mut() {
        let (x_cap, y_cap) = match player.state {
            PlayerState::Jump { .. } => (100., 35.),
            PlayerState::Wait | PlayerState::Walk { .. } => (15., 100.),
        };
        if rb_vel.linvel.data.0[0][0] > x_cap {
            rb_vel.linvel.data.0[0][0] = x_cap;
        } else if rb_vel.linvel.data.0[0][0] < -x_cap {
            rb_vel.linvel.data.0[0][0] = -x_cap;
        }
        if rb_vel.linvel.data.0[0][1] > y_cap {
            rb_vel.linvel.data.0[0][1] = y_cap;
        } else if rb_vel.linvel.data.0[0][1] < -y_cap {
            rb_vel.linvel.data.0[0][1] = -y_cap;
        }
    }
}

fn player_movement(
    mut query: Query<(&Player, &mut RigidBodyVelocity, &RigidBodyMassProps)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (player, mut rb_vel, rb_mprops) in query.iter_mut() {
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;

        let move_delta = Vector2::new(x_axis as f32, 0.);

        match player.state {
            PlayerState::Jump { .. } => {}
            _ => {
                rb_vel.apply_impulse(rb_mprops, move_delta * MOVE_IMPULSE_MULTIPLIER);
            }
        }
    }
}

fn jump(
    mut query: Query<(&Player, &mut RigidBodyVelocity)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut psc_event: EventWriter<PlayerStateChangeEvent>,
) {
    for (player, mut rb_vel) in query.iter_mut() {
        let jump = keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::W);
        if !jump {
            return;
        }
        match player.state {
            PlayerState::Jump { .. } => {}
            _ => {
                rb_vel.linvel.data.0[0][1] = JUMP_FORCE;
                psc_event.send(PlayerStateChangeEvent {
                    state: PlayerState::Jump {
                        tick: 0,
                        linvel_x: Some(rb_vel.linvel.data.0[0][0]),
                    },
                })
            }
        }
    }
}

fn set_sprite(
    mut query: Query<(&Player, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    mut psc_events: EventReader<PlayerStateChangeEvent>,
    assets: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    if let Ok((_, mut sprite, atlas_handle)) = query.single_mut() {
        for event in psc_events.iter() {
            let texture_atlas = texture_atlases.get(atlas_handle).unwrap();
            let asset_path = match event.state {
                PlayerState::Wait => "MW_Player_MarioMdl_wait.0_0.png",
                PlayerState::Walk { frame: 0, .. } => "MW_Player_MarioMdl_walk.0_0.png",
                PlayerState::Walk { .. } => "MW_Player_MarioMdl_walk.1_0.png",
                PlayerState::Jump { .. } => "MW_Player_MarioMdl_jump.0_0.png",
            };
            let handle = assets.load(asset_path);
            let index = texture_atlas.get_texture_index(&handle).unwrap();
            sprite.index = index as u32;
        }
    }
}
