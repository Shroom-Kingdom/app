mod course;
mod course_sprites;
mod player_sprites;

pub use course::{Course, CourseTheme, Tile, TileVariant};
pub use course_sprites::{CourseSpriteHandles, CourseTile};
pub use player_sprites::{PlayerFrame, PlayerSpriteHandles};

use app_config::GRID_SIZE;
use bevy::{asset::LoadState, ecs::schedule::ShouldRun, prelude::*};
use course_sprites::load_course_sprites;
use player_sprites::load_player_sprites;

#[derive(Component, Debug)]
pub struct Ground;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSpriteHandles>()
            .init_resource::<CourseSpriteHandles>()
            .init_resource::<DoneInsertCourse>()
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

#[derive(Default)]
pub struct DoneInsertCourse(pub bool);

pub fn is_done_insert_course(done: Res<DoneInsertCourse>) -> ShouldRun {
    if done.0 {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn grid_to_world(grid_pos: &[i32; 2]) -> Vec2 {
    [
        grid_pos[0] as f32 * GRID_SIZE,
        grid_pos[1] as f32 * GRID_SIZE,
    ]
    .into()
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    player_sprite_handles: ResMut<PlayerSpriteHandles>,
    course_sprite_handles: ResMut<CourseSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let (LoadState::Loaded, LoadState::Loaded) = (
        asset_server
            .get_group_load_state(player_sprite_handles.0.iter().map(|(_, handle)| handle.id)),
        asset_server
            .get_group_load_state(course_sprite_handles.0.iter().map(|(_, handle)| handle.id)),
    ) {
        state.set(AppState::Menu).unwrap();
    }
}
