mod debug;
mod ground;
mod jump;
mod movement;
mod setup;
mod state_change;
mod walk;

use app_core::AppState;
use bevy::prelude::*;
use debug::setup_ui;
use ground::ground_intersect;
use jump::{high_jump, jump, jump_to_fall};
use movement::{movement, movement_cap};
use setup::setup;
use state_change::state_change;
use walk::{walk_animation, walk_start};

pub use ground::GroundIntersectEvent;
pub use jump::{FallEvent, JumpEvent};
pub use walk::WalkEvent;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerStateChangeEvent>()
            .add_event::<WalkEvent>()
            .add_event::<FallEvent>()
            .add_event::<JumpEvent>()
            .add_event::<GroundIntersectEvent>()
            .add_event::<StoopEvent>()
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
            .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup))
            .add_system_to_stage(CoreStage::First, jump)
            .add_system_to_stage(CoreStage::First, high_jump)
            .add_system_to_stage(CoreStage::First, walk_animation)
            .add_system_to_stage(CoreStage::First, walk_start)
            .add_system_to_stage(CoreStage::PreUpdate, movement)
            .add_system_to_stage(CoreStage::PreUpdate, stoop)
            .add_system_to_stage(PlayerStages::PostInput, movement_cap)
            // .add_system_to_stage(CoreStage::PostUpdate, debug::text_update_system)
            .add_system_to_stage(CoreStage::PostUpdate, ground_intersect)
            .add_system_to_stage(CoreStage::PostUpdate, jump_to_fall)
            .add_system_to_stage(PlayerStages::StateChange, state_change)
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
pub struct PlayerState {
    state: PlayerStateEnum,
    is_stooping: bool,
}

#[derive(Clone, Debug)]
pub enum PlayerStateEnum {
    Wait,
    Walk {
        frame: u8,
        is_turning: bool,
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
    mut query: Query<(&Player, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
    mut psc_events: EventReader<PlayerStateChangeEvent>,
    assets: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    if let Ok((_, mut sprite, atlas_handle)) = query.single_mut() {
        if let Some(event) = psc_events.iter().last() {
            let texture_atlas = texture_atlases.get(atlas_handle).unwrap();
            let asset_path = match &event.state {
                PlayerState {
                    is_stooping: true, ..
                } => "MW_Player_MarioMdl_stoop.0_0.png",
                PlayerState {
                    is_stooping: false,
                    state,
                } => match state {
                    PlayerStateEnum::Wait { .. } => "MW_Player_MarioMdl_wait.0_0.png",
                    PlayerStateEnum::Walk { frame: 0, .. } => "MW_Player_MarioMdl_walk.0_0.png",
                    PlayerStateEnum::Walk { .. } => "MW_Player_MarioMdl_walk.1_0.png",
                    PlayerStateEnum::Jump { .. } => "MW_Player_MarioMdl_jump.0_0.png",
                    PlayerStateEnum::Fall { .. } => "MW_Player_MarioMdl_jump_fall.0_0.png",
                },
            };
            let handle = assets.load(asset_path);
            let index = texture_atlas.get_texture_index(&handle).unwrap();
            sprite.index = index as u32;
        }
    }
}
