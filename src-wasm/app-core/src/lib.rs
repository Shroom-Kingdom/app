#![allow(clippy::needless_question_mark)]

mod course;
mod mode;
mod player_sprites;

pub use course::{
    sprites::{ThemeSpriteHandles, TileSpriteHandles, UiButtonSpriteHandles},
    theme::ThemeVariant,
    tile::{GroundSurroundingMatrix, GroundVariant, SelectedTile, Tile, TileVariant},
    ui_button::UiButtonVariant,
    Course,
};
pub use mode::GameMode;
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
            .init_resource::<ThemeSpriteHandles>()
            .init_resource::<UiButtonSpriteHandles>()
            .init_resource::<SelectedTile>()
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
    course_sprite_handles: Res<TileSpriteHandles>,
    theme_sprite_handles: Res<ThemeSpriteHandles>,
    ui_button_sprite_handles: Res<UiButtonSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let (LoadState::Loaded, LoadState::Loaded, LoadState::Loaded, LoadState::Loaded) = (
        asset_server
            .get_group_load_state(player_sprite_handles.0.iter().map(|(_, handle)| handle.id)),
        asset_server
            .get_group_load_state(course_sprite_handles.0.iter().map(|(_, handle)| handle.id)),
        asset_server
            .get_group_load_state(theme_sprite_handles.0.iter().map(|(_, handle)| handle.id)),
        asset_server
            .get_group_load_state(ui_button_sprite_handles.0.iter().map(|(_, handle)| handle.id)),
    ) {
        state.set(AppState::Menu).unwrap();
    }
}
