#![allow(clippy::needless_question_mark)]

mod course;
mod game_mode;
mod player_sprites;

pub use course::{
    get_surrounding_matrix,
    sprites::{
        ThemeSpriteHandles, TileSpriteHandles, TileSpriteHandlesTransparent, UiButtonSpriteHandles,
    },
    theme::ThemeVariant,
    tile::{
        GroundSurroundingMatrix, GroundVariant, SelectedTile, Tile, TilePlacePreview, TilePreview,
        TileVariant,
    },
    ui_button::UiButtonVariant,
    Course,
};
pub use game_mode::{GameMode, GameModeToggleEvent};
pub use player_sprites::{PlayerFrame, PlayerSpriteHandles};

use app_config::{GRID_SIZE, RAPIER_SCALE};
use bevy::{asset::LoadState, prelude::*};
use course::sprites::load_course_sprites;
use player_sprites::load_player_sprites;

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
            .init_resource::<SelectedTile>()
            .insert_resource(TilePlacePreview(None))
            .add_event::<GameModeToggleEvent>()
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
            .add_system_set_to_stage(PlayerStages::PlayerInput, State::<AppState>::get_driver())
            .add_system_set_to_stage(PlayerStages::PrePhysics, State::<AppState>::get_driver())
            .add_system_set_to_stage(PlayerStages::StateChange, State::<AppState>::get_driver())
            .add_startup_system_to_stage(StartupStage::Startup, load_player_sprites)
            .add_startup_system_to_stage(StartupStage::Startup, load_course_sprites)
            .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    Menu,
    Game,
}

#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppLabel {
    InsertCourse,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PlayerStages {
    PlayerInput,
    PrePhysics,
    StateChange,
}

pub fn grid_to_world(grid_pos: &[i32; 2]) -> Vec2 {
    [
        grid_pos[0] as f32 * GRID_SIZE * RAPIER_SCALE,
        grid_pos[1] as f32 * GRID_SIZE * RAPIER_SCALE,
    ]
    .into()
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
        state.set(AppState::Menu).unwrap();
    }
}
