mod camera;
mod debug;
mod jump;
mod movement;
mod physics;
mod setup;
mod state_change;
mod touch;
mod walk;

use app_core::AppState;
use bevy::prelude::*;
use camera::position_camera;
use debug::setup_ui;
use jump::{high_jump, jump, jump_to_fall};
use movement::{movement, run};
use physics::{apply_vel, physics};
use setup::setup;
use state_change::state_change;
use touch::touch;
use walk::{walk_animation, walk_start};

pub use jump::JumpEvent;
pub use movement::{DashTurnEvent, MovementEvent};
pub use physics::{GroundIntersectEvent, GroundIntersections, PlayerVelocity};
pub use touch::TouchEvent;
pub use walk::WalkEvent;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerStateChangeEvent>()
            .add_event::<WalkEvent>()
            .add_event::<MovementEvent>()
            .add_event::<DashTurnEvent>()
            .add_event::<JumpEvent>()
            .add_event::<GroundIntersectEvent>()
            .add_event::<StoopEvent>()
            .add_event::<TouchEvent>()
            .add_startup_system(setup_ui)
            .add_stage_after(
                CoreStage::First,
                PlayerStages::PlayerInput,
                SystemStage::parallel(),
            )
            .add_stage_after(
                CoreStage::PreUpdate,
                PlayerStages::PrePhysics,
                SystemStage::parallel(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                PlayerStages::StateChange,
                SystemStage::parallel(),
            )
            .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
            .add_system_to_stage(CoreStage::First, run)
            .add_system_to_stage(CoreStage::First, jump)
            .add_system_to_stage(CoreStage::First, high_jump)
            .add_system_to_stage(CoreStage::First, walk_animation)
            .add_system_to_stage(CoreStage::First, walk_start)
            .add_system_to_stage(PlayerStages::PlayerInput, movement)
            .add_system_to_stage(CoreStage::PreUpdate, physics)
            .add_system_to_stage(CoreStage::PreUpdate, stoop)
            .add_system_to_stage(PlayerStages::PrePhysics, apply_vel)
            // .add_system_to_stage(CoreStage::PostUpdate, debug::text_update_system)
            .add_system_to_stage(CoreStage::PostUpdate, touch)
            .add_system_to_stage(CoreStage::PostUpdate, jump_to_fall)
            .add_system_to_stage(CoreStage::PostUpdate, position_camera)
            .add_system_to_stage(PlayerStages::StateChange, state_change)
            .add_system_to_stage(CoreStage::Last, set_sprite);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PlayerStages {
    PlayerInput,
    PrePhysics,
    StateChange,
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
}

#[derive(Clone, Debug)]
pub struct PlayerState {
    facing_direction: FacingDirection,
    state: PlayerStateEnum,
    is_running: bool,
    is_dashing: bool,
    is_stooping: bool,
    is_dash_turning: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FacingDirection {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum PlayerStateEnum {
    Ground {
        frame: u8,
        is_walking: bool,
        is_turning: bool,
    },
    Air {
        tick: u8,
        high_jump_tick: u8,
        released: bool,
        impulse: bool,
        fall: bool,
    },
}

pub struct PlayerStateChangeEvent {
    pub state: PlayerState,
}

pub struct StoopEvent {
    is_stooping: bool,
}

fn stoop(
    query: Query<&Player>,
    keyboard_input: Res<Input<KeyCode>>,
    mut stoop_events: EventWriter<StoopEvent>,
) {
    if let Ok(player) = query.single() {
        let stooped = !player.state.is_stooping
            && (keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down));
        let unstooped = player.state.is_stooping
            && !keyboard_input.pressed(KeyCode::S)
            && !keyboard_input.pressed(KeyCode::Down);

        if stooped {
            stoop_events.send(StoopEvent { is_stooping: true });
        } else if unstooped {
            stoop_events.send(StoopEvent { is_stooping: false });
        }
    }
}

fn set_sprite(
    mut query: Query<(&Children, &PlayerVelocity), With<Player>>,
    mut child_query: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    mut psc_events: EventReader<PlayerStateChangeEvent>,
    assets: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    if let Ok((children, vel)) = query.single_mut() {
        let child = children.first().unwrap();
        let (mut sprite, atlas_handle) = child_query.get_mut(*child).unwrap();
        if let Some(event) = psc_events.iter().last() {
            let texture_atlas = texture_atlases.get(atlas_handle).unwrap();
            let asset_path = match &event.state {
                PlayerState {
                    is_stooping: true, ..
                } => "MW_Player_MarioMdl_stoop.0_0.png",
                PlayerState {
                    is_stooping: false,
                    is_dashing,
                    is_dash_turning,
                    facing_direction,
                    state,
                    ..
                } => match state {
                    PlayerStateEnum::Ground { .. } if vel.0[0].abs() < f32::EPSILON => {
                        "MW_Player_MarioMdl_wait.0_0.png"
                    }
                    PlayerStateEnum::Ground { frame, .. } => {
                        sprite.flip_x = match *facing_direction {
                            FacingDirection::Left => !is_dash_turning,
                            FacingDirection::Right => *is_dash_turning,
                        };
                        if *is_dash_turning {
                            "MW_Player_MarioMdl_turn.0_0.png"
                        } else if *is_dashing {
                            if *frame == 1 {
                                "MW_Player_MarioMdl_b_dash.1_0.png"
                            } else {
                                "MW_Player_MarioMdl_b_dash.0_0.png"
                            }
                        } else if *frame == 1
                            || (vel.0[0] > 0. && *facing_direction == FacingDirection::Left)
                            || (vel.0[0] < 0. && *facing_direction == FacingDirection::Right)
                        {
                            "MW_Player_MarioMdl_walk.1_0.png"
                        } else {
                            "MW_Player_MarioMdl_walk.0_0.png"
                        }
                    }
                    PlayerStateEnum::Air { tick, .. } => {
                        if vel.0[1] > 0. {
                            if *tick == 0 {
                                sprite.flip_x = match *facing_direction {
                                    FacingDirection::Left => true,
                                    FacingDirection::Right => false,
                                };
                            }
                            if !is_dashing {
                                "MW_Player_MarioMdl_jump.0_0.png"
                            } else {
                                "MW_Player_MarioMdl_b_dash_jump.0_0.png"
                            }
                        } else if !is_dashing {
                            "MW_Player_MarioMdl_jump_fall.0_0.png"
                        } else {
                            "MW_Player_MarioMdl_b_dash_jump_fall.0_0.png"
                        }
                    }
                },
            };
            let handle = assets.load(asset_path);
            let index = texture_atlas.get_texture_index(&handle).unwrap();
            sprite.index = index as u32;
        }
    }
}
