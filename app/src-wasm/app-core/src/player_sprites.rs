use bevy::{prelude::*, utils::HashMap};

#[derive(Default)]
pub struct PlayerSpriteHandles(pub HashMap<PlayerFrame, Handle<Image>>);

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

pub(crate) fn load_player_sprites(
    mut sprite_handles: ResMut<PlayerSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    sprite_handles.0 = HashMap::default();
    sprite_handles.0.insert(
        PlayerFrame::Wait,
        asset_server.load("MW_Player_MarioMdl_wait.0_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::Walk0,
        asset_server.load("MW_Player_MarioMdl_walk.0_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::Walk1,
        asset_server.load("MW_Player_MarioMdl_walk.1_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::Jump,
        asset_server.load("MW_Player_MarioMdl_jump.0_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::Fall,
        asset_server.load("MW_Player_MarioMdl_jump_fall.0_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::Stoop,
        asset_server.load("MW_Player_MarioMdl_stoop.0_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::Dash0,
        asset_server.load("MW_Player_MarioMdl_b_dash.0_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::Dash1,
        asset_server.load("MW_Player_MarioMdl_b_dash.1_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::DashJump,
        asset_server.load("MW_Player_MarioMdl_b_dash_jump.0_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::DashJumpFall,
        asset_server.load("MW_Player_MarioMdl_b_dash_jump_fall.0_0.png"),
    );
    sprite_handles.0.insert(
        PlayerFrame::Turn,
        asset_server.load("MW_Player_MarioMdl_turn.0_0.png"),
    );
}
