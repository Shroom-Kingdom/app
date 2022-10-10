mod jump;
mod kill;
mod movement;
mod physics;
mod setup;
mod state_change;
mod stoop;
mod touch;
mod walk;

use app_core::{AppStage, AppState};
use bevy::prelude::*;
use jump::{high_jump, jump, jump_to_fall};
use kill::below_surface;
use movement::{movement, run};
use physics::{apply_vel, physics};
use setup::setup;
use state_change::state_change;
use stoop::stoop;
use touch::touch;
use walk::{walk_animation, walk_start};

pub use jump::JumpEvent;
pub use kill::KillEvent;
pub use movement::{DashTurnEvent, FacingDirectionEvent};
pub use physics::{GroundIntersectEvent, GroundIntersections, PlayerVelocity};
pub use stoop::StoopEvent;
pub use touch::TouchEvent;
pub use walk::{WalkAnimationTimer, WalkEvent};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerStateChangeEvent>()
            .add_event::<WalkEvent>()
            .add_event::<FacingDirectionEvent>()
            .add_event::<DashTurnEvent>()
            .add_event::<JumpEvent>()
            .add_event::<GroundIntersectEvent>()
            .add_event::<StoopEvent>()
            .add_event::<TouchEvent>()
            .add_event::<KillEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::on_update(AppState::Game).with_system(below_surface),
            )
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::on_update(AppState::Game).with_system(run),
            )
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::on_update(AppState::Game).with_system(jump),
            )
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::on_update(AppState::Game).with_system(high_jump),
            )
            .add_system_set_to_stage(
                CoreStage::First,
                SystemSet::on_update(AppState::Game).with_system(walk_animation),
            )
            .add_system_set_to_stage(
                AppStage::PlayerInput,
                SystemSet::on_update(AppState::Game).with_system(movement),
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_update(AppState::Game).with_system(physics),
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_update(AppState::Game).with_system(stoop),
            )
            .add_system_set_to_stage(
                AppStage::PrePhysics,
                SystemSet::on_update(AppState::Game).with_system(apply_vel),
            )
            .add_system_set_to_stage(
                AppStage::PrePhysics,
                SystemSet::on_update(AppState::Game).with_system(walk_start),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(AppState::Game).with_system(touch),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(AppState::Game).with_system(jump_to_fall),
            )
            .add_system_set_to_stage(
                AppStage::StateChange,
                SystemSet::on_update(AppState::Game).with_system(state_change),
            )
            .add_system_set_to_stage(
                CoreStage::Last,
                SystemSet::on_update(AppState::Game).with_system(set_sprite),
            );
    }
}

#[derive(Component, Debug)]
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

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
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
        }
    }
}

impl PlayerState {
    pub fn float() -> Self {
        PlayerState {
            facing_direction: FacingDirection::Right,
            state: PlayerStateEnum::Ground {
                frame: 0,
                is_walking: false,
                is_turning: false,
            },
            is_running: false,
            is_dashing: false,
            is_stooping: false,
            is_dash_turning: false,
        }
    }
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

#[derive(Debug)]
pub struct PlayerStateChangeEvent {
    pub state: PlayerState,
}

fn set_sprite(
    mut query: Query<(&Children, &PlayerVelocity), With<Player>>,
    mut child_query: Query<(&mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    mut psc_events: EventReader<PlayerStateChangeEvent>,
    assets: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    if let Ok((children, vel)) = query.get_single_mut() {
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
                    PlayerStateEnum::Ground { .. } if vel.0.x.abs() < f32::EPSILON => {
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
                            || (vel.0.x > 0. && *facing_direction == FacingDirection::Left)
                            || (vel.0.x < 0. && *facing_direction == FacingDirection::Right)
                        {
                            "MW_Player_MarioMdl_walk.1_0.png"
                        } else {
                            "MW_Player_MarioMdl_walk.0_0.png"
                        }
                    }
                    PlayerStateEnum::Air { tick, .. } => {
                        if vel.0.y > 0. {
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
            sprite.index = index;
        }
    }
}
