mod course;

use app_config::GRID_SIZE;
pub use course::{Course, CourseTheme, Tile, TileVariant};

use bevy::{asset::LoadState, prelude::*, utils::HashMap};

#[derive(Component, Debug)]
pub struct Ground;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSpriteHandles>()
            .add_startup_system_to_stage(StartupStage::Startup, load_player_sprites)
            .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    Menu,
    Game,
}

#[derive(Default)]
pub struct PlayerSpriteHandles {
    pub handles: HashMap<PlayerFrame, Handle<Image>>,
}

#[derive(Eq, Hash, PartialEq)]
pub enum PlayerFrame {
    Wait,
    Walk0,
    Walk1,
    Jump,
    Fall,
    Stoop,
    Dash0,
    Dash1,
    DashJump,
    DashJumpFall,
    Turn,
}

fn load_player_sprites(
    mut sprite_handles: ResMut<PlayerSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    sprite_handles.handles = HashMap::default();
    sprite_handles.handles.insert(
        PlayerFrame::Wait,
        asset_server.load("MW_Player_MarioMdl_wait.0_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::Walk0,
        asset_server.load("MW_Player_MarioMdl_walk.0_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::Walk1,
        asset_server.load("MW_Player_MarioMdl_walk.1_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::Jump,
        asset_server.load("MW_Player_MarioMdl_jump.0_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::Fall,
        asset_server.load("MW_Player_MarioMdl_jump_fall.0_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::Stoop,
        asset_server.load("MW_Player_MarioMdl_stoop.0_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::Dash0,
        asset_server.load("MW_Player_MarioMdl_b_dash.0_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::Dash1,
        asset_server.load("MW_Player_MarioMdl_b_dash.1_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::DashJump,
        asset_server.load("MW_Player_MarioMdl_b_dash_jump.0_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::DashJumpFall,
        asset_server.load("MW_Player_MarioMdl_b_dash_jump_fall.0_0.png"),
    );
    sprite_handles.handles.insert(
        PlayerFrame::Turn,
        asset_server.load("MW_Player_MarioMdl_turn.0_0.png"),
    );
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    player_sprite_handles: ResMut<PlayerSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(
        player_sprite_handles
            .handles
            .iter()
            .map(|(_, handle)| handle.id),
    ) {
        state.set(AppState::Menu).unwrap();
    }
}

pub fn grid_to_world(grid_pos: &[i32; 2]) -> Vec2 {
    [
        grid_pos[0] as f32 * GRID_SIZE,
        grid_pos[1] as f32 * GRID_SIZE,
    ]
    .into()
}
