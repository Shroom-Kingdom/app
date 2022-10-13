#![allow(clippy::needless_question_mark)]

mod course;
mod drag;
mod game_mode;
mod player_sprites;
mod tile;
mod utils;

pub use course::{
    get_surrounding_matrix,
    goal_pole::{GoalPole, GoalPoleDragDirection, GoalPoleDragEvent, GoalPoleDragTimer},
    object::ObjectVariant,
    sprites::{
        ObjectSpriteHandles, ThemeSpriteHandles, TileSpriteHandles, TileSpriteHandlesTransparent,
        UiButtonSpriteHandles,
    },
    tile::{
        GroundSurroundingMatrix, SelectedTile, Tile, TileComponent, TileNotEditable,
        TilePlacePreview, TilePreview,
    },
    ui_button::UiButtonVariant,
    CourseLoading, CourseRes,
};
pub use drag::{DragEvent, DragEventFlags, Draggable, Dragging};
pub use game_mode::{
    GameMode, GameModeEdit, GameModeToggleButton, GameModeToggleButtonImage, GameModeToggleEvent,
};
pub use player_sprites::{PlayerFrame, PlayerSpriteHandles};
use tile::update_ground_tile;
pub use tile::{DespawnTileEvent, GroundTileUpdateEvent, SpawnTileEvent};
pub use utils::*;

use bevy::{asset::LoadState, prelude::*};
use course::{
    goal_pole::{move_goal_pole, respawn_goal_pole, RespawnGoalPoleEvent},
    sprites::load_course_sprites,
};
use drag::{drag_mouse_button, drag_mouse_motion, handle_drag_events};
use player_sprites::load_player_sprites;
use std::sync::{Arc, RwLock};

#[derive(Component, Debug)]
pub struct Ground;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSpriteHandles>()
            .init_resource::<TileSpriteHandles>()
            .init_resource::<TileSpriteHandlesTransparent>()
            .init_resource::<ThemeSpriteHandles>()
            .init_resource::<UiButtonSpriteHandles>()
            .init_resource::<ObjectSpriteHandles>()
            .init_resource::<SelectedTile>()
            .init_resource::<Option<Dragging>>()
            .insert_resource(TilePlacePreview(None))
            .insert_resource(Arc::new(RwLock::new(CourseLoading(None))))
            .add_event::<GameModeToggleEvent>()
            .add_event::<DragEvent>()
            .add_event::<GoalPoleDragEvent>()
            .add_event::<RespawnGoalPoleEvent>()
            .add_stage_after(
                CoreStage::First,
                AppStage::PlayerInput,
                SystemStage::parallel(),
            )
            .add_stage_after(
                CoreStage::PreUpdate,
                AppStage::PrePhysics,
                SystemStage::parallel(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                AppStage::StateChange,
                SystemStage::parallel(),
            )
            .add_stage_after(
                CoreStage::PostUpdate,
                AppStage::TileSpawning,
                SystemStage::parallel(),
            )
            .add_system_set_to_stage(AppStage::PlayerInput, State::<AppState>::get_driver())
            .add_system_set_to_stage(AppStage::PrePhysics, State::<AppState>::get_driver())
            .add_system_set_to_stage(AppStage::StateChange, State::<AppState>::get_driver())
            .add_system_set_to_stage(AppStage::TileSpawning, State::<AppState>::get_driver())
            .add_startup_system_to_stage(StartupStage::Startup, load_player_sprites)
            .add_startup_system_to_stage(StartupStage::Startup, load_course_sprites)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_update(AppState::Game).with_system(drag_mouse_button),
            )
            .add_system_set_to_stage(
                CoreStage::Update,
                SystemSet::on_update(AppState::Game).with_system(drag_mouse_motion),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(AppState::Game).with_system(handle_drag_events),
            )
            .add_system_set_to_stage(
                AppStage::TileSpawning,
                SystemSet::on_update(AppState::Game)
                    .with_system(move_goal_pole)
                    .before(AppLabel::DespawnTile),
            )
            .add_system_set_to_stage(
                AppStage::TileSpawning,
                SystemSet::on_update(AppState::Game)
                    .with_system(respawn_goal_pole)
                    .after(AppLabel::DespawnTile),
            )
            .add_system_set_to_stage(
                CoreStage::Last,
                SystemSet::on_update(AppState::Game).with_system(update_ground_tile),
            )
            .add_system_set(SystemSet::on_exit(AppState::Load).with_system(update_ground_tile))
            .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    Menu,
    Load,
    Game,
}

#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppLabel {
    InsertCourse,
    DespawnTile,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum AppStage {
    PlayerInput,
    PrePhysics,
    StateChange,
    TileSpawning,
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    player_sprite_handles: Res<PlayerSpriteHandles>,
    tile_sprite_handles: Res<TileSpriteHandles>,
    tile_sprite_handles_transparent: Res<TileSpriteHandlesTransparent>,
    theme_sprite_handles: Res<ThemeSpriteHandles>,
    ui_button_sprite_handles: Res<UiButtonSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let (
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
        LoadState::Loaded,
    ) = (
        asset_server
            .get_group_load_state(player_sprite_handles.0.iter().map(|(_, handle)| handle.id)),
        asset_server
            .get_group_load_state(tile_sprite_handles.0.iter().map(|(_, handle)| handle.id)),
        asset_server.get_group_load_state(
            tile_sprite_handles_transparent
                .0
                .iter()
                .map(|(_, handle)| handle.id),
        ),
        asset_server
            .get_group_load_state(theme_sprite_handles.0.iter().map(|(_, handle)| handle.id)),
        asset_server.get_group_load_state(
            ui_button_sprite_handles
                .0
                .iter()
                .map(|(_, handle)| handle.id),
        ),
    ) {
        state.overwrite_set(AppState::Menu).unwrap();
    }
}
