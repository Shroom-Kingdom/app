mod debug;
mod ground;
mod jump;
mod walk;

use app_config::{
    LINVEL_CAP_AIR, LINVEL_CAP_GROUND, MOVE_IMPULSE_MULTIPLIER_GROUND, RAPIER_GRAVITY_VECTOR,
    RAPIER_SCALE,
};
use app_core::AppState;
use bevy::{prelude::*, sprite::TextureAtlasBuilder};
use bevy_rapier::{
    na::Vector2,
    physics::{ColliderBundle, ColliderPositionSync, RapierConfiguration, RigidBodyBundle},
    prelude::*,
};
use debug::setup_ui;
use ground::GroundIntersectEvent;
use jump::{high_jump, jump, JumpEvent};
use walk::{walk_animation, walk_start, WalkEvent};

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerStateChangeEvent>()
            .add_event::<WalkEvent>()
            .add_event::<JumpEvent>()
            .add_event::<GroundIntersectEvent>()
            .add_startup_system(setup_ui)
            .add_stage_after(
                CoreStage::First,
                PlayerStages::PostInput,
                SystemStage::parallel(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                PlayerStages::StateChange,
                SystemStage::parallel(),
            )
            .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup_character))
            .add_system_to_stage(CoreStage::First, jump)
            .add_system_to_stage(CoreStage::First, high_jump)
            .add_system_to_stage(CoreStage::First, walk_animation)
            .add_system_to_stage(CoreStage::First, walk_start)
            .add_system_to_stage(CoreStage::PreUpdate, player_movement)
            .add_system_to_stage(PlayerStages::PostInput, player_movement_cap)
            // .add_system_to_stage(CoreStage::PostUpdate, move_impulse_jump)
            // .add_system_to_stage(CoreStage::PostUpdate, debug::text_update_system)
            .add_system_to_stage(CoreStage::PostUpdate, ground::ground_intersect)
            .add_system_to_stage(PlayerStages::StateChange, player_state_change)
            .add_system_to_stage(CoreStage::Last, set_sprite);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PlayerStages {
    PostInput,
    StateChange,
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Clone, Debug)]
pub enum PlayerState {
    Wait,
    Walk {
        frame: u8,
    },
    Jump {
        tick: u8,
        released: bool,
        impulse: bool,
    },
    Fall,
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
    rapier_config.gravity = RAPIER_GRAVITY_VECTOR;

    let scale_size = 2.;
    let sprite_size_x = scale_size * 12.0;
    let sprite_size_y = scale_size * 16.0;
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
            state: PlayerState::Fall,
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(Timer::from_seconds(1.3, true));

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_atlas_texture.into()),
        transform: Transform::from_xyz(-300.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn player_state_change(
    mut query: Query<(&mut Player, &RigidBodyVelocity)>,
    mut jump_events: EventReader<JumpEvent>,
    mut ground_intersect_events: EventReader<GroundIntersectEvent>,
    mut walk_events: EventReader<WalkEvent>,
    mut psc_events: EventWriter<PlayerStateChangeEvent>,
) {
    if let Ok((mut player, rb_vel)) = query.single_mut() {
        if let Some(state) = match (
            jump_events.iter().next(),
            ground_intersect_events.iter().next(),
            walk_events.iter().next(),
        ) {
            (Some(_), _, _) => Some(PlayerState::Jump {
                tick: 0,
                impulse: false,
                released: false,
            }),
            (_, Some(GroundIntersectEvent::Start), _) => {
                if rb_vel
                    .linvel
                    .data
                    .0
                    .get(0)
                    .map(|x| x[0] == 0.)
                    .unwrap_or_default()
                {
                    Some(PlayerState::Wait)
                } else {
                    Some(PlayerState::Walk {
                        frame: 1,
                        // linvel_x: Some(rb_vel.linvel.data.0[0][0]),
                    })
                }
            }
            (_, Some(GroundIntersectEvent::Stop), _) => Some(PlayerState::Fall),
            (_, _, Some(WalkEvent::Start)) => Some(PlayerState::Walk { frame: 0 }),
            (_, _, Some(WalkEvent::Stop)) => Some(PlayerState::Wait),
            (_, _, Some(WalkEvent::Advance)) => {
                if let PlayerState::Walk { frame } = player.state {
                    let frame = match frame {
                        0 => 1,
                        _ => 0,
                    };
                    Some(PlayerState::Walk { frame })
                } else {
                    None
                }
            }
            (None, None, None) => None,
        } {
            player.state = state.clone();
            psc_events.send(PlayerStateChangeEvent { state });
        }
    }
}

fn player_movement_cap(mut query: Query<(&Player, &mut RigidBodyVelocity)>) {
    for (player, mut rb_vel) in query.iter_mut() {
        let (x_cap, y_cap) = match player.state {
            PlayerState::Jump { .. } | PlayerState::Fall => LINVEL_CAP_AIR,
            PlayerState::Wait | PlayerState::Walk { .. } => LINVEL_CAP_GROUND,
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
    mut query: Query<(
        &Player,
        &mut RigidBodyVelocity,
        &RigidBodyMassProps,
        &mut TextureAtlasSprite,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (player, mut rb_vel, rb_mprops, mut sprite) in query.iter_mut() {
        match player.state {
            PlayerState::Jump { .. } | PlayerState::Fall => {}
            PlayerState::Wait | PlayerState::Walk { .. } => {
                let left =
                    keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
                let right =
                    keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

                let x_axis = -(left as i8) + right as i8;
                match x_axis {
                    _ if x_axis > 0 => {
                        sprite.flip_x = false;
                    }
                    _ if x_axis < 0 => {
                        sprite.flip_x = true;
                    }
                    _ => {}
                }
                if x_axis != 0 {
                    let move_delta = Vector2::new(x_axis as f32, 0.);
                    rb_vel.apply_impulse(rb_mprops, move_delta * MOVE_IMPULSE_MULTIPLIER_GROUND);
                }
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
        if let Some(event) = psc_events.iter().last() {
            let texture_atlas = texture_atlases.get(atlas_handle).unwrap();
            let asset_path = match event.state {
                PlayerState::Wait => "MW_Player_MarioMdl_wait.0_0.png",
                PlayerState::Walk { frame: 0 } => "MW_Player_MarioMdl_walk.0_0.png",
                PlayerState::Walk { .. } => "MW_Player_MarioMdl_walk.1_0.png",
                PlayerState::Jump { .. } => "MW_Player_MarioMdl_jump.0_0.png",
                PlayerState::Fall => "MW_Player_MarioMdl_jump_fall.0_0.png",
            };
            let handle = assets.load(asset_path);
            let index = texture_atlas.get_texture_index(&handle).unwrap();
            sprite.index = index as u32;
        }
    }
}
