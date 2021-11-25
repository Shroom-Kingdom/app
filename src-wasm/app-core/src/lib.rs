mod course;

pub use course::{Course, CourseTheme, Tile, TileVariant};

use bevy::{asset::LoadState, prelude::*};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSpriteHandles>()
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_player_sprites))
            .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
struct PlayerSpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn load_player_sprites(
    mut sprite_handles: ResMut<PlayerSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    sprite_handles.handles = vec![
        asset_server.load_untyped("MW_Player_MarioMdl_wait.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_walk.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_walk.1_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_jump.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_jump_fall.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_stoop.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_b_dash.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_b_dash.1_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_b_dash_jump.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_b_dash_jump_fall.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_turn.0_0.png"),
    ];
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    player_sprite_handles: ResMut<PlayerSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server
        .get_group_load_state(player_sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}
