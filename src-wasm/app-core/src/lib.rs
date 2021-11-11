use bevy::{asset::LoadState, prelude::*};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteHandles>()
            .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
            .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = vec![
        asset_server.load_untyped("MW_Player_MarioMdl_wait.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_walk.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_walk.1_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_jump.0_0.png"),
        asset_server.load_untyped("MW_Player_MarioMdl_jump_fall.0_0.png"),
    ];
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(AppState::Finished).unwrap();
    }
}
