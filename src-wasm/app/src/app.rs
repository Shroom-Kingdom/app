use crate::{
    character::{
        ground_intersect, player_movement, set_sprite, setup_character, PlayerStateChangeEvent,
    },
    graphics::{enable_physics_profiling, setup_graphics},
    ground::setup_ground,
};
use app_assets::{AssetIoTarConfig, AssetIoTarPlugin};
// use app_debug::{DebugPlugin, DebugPluginState};
use bevy::{asset::LoadState, input::keyboard::keyboard_input_system, prelude::*};
use bevy_rapier::{
    physics::{NoUserData, RapierPhysicsPlugin},
    render::RapierRenderPlugin,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(assets: Vec<u8>) {
    App::new()
        .init_resource::<SpriteHandles>()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .insert_resource(WindowDescriptor {
            vsync: false,
            canvas: Some("canvas".to_string()),
            ..Default::default()
        })
        // .insert_non_send_resource(DebugPluginState {
        //     debug_state: debug_state.into(),
        //     set_debug_state,
        // })
        .insert_resource(AssetIoTarConfig(assets))
        .add_event::<PlayerStateChangeEvent>()
        .add_state(AppState::Setup)
        .add_plugins_with(bevy_webgl2::DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(AssetIoTarPlugin)
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        // .add_plugin(DebugPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_ground.system())
        .add_startup_system(enable_physics_profiling.system())
        .add_system_set(SystemSet::on_enter(AppState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(AppState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(AppState::Finished).with_system(setup_character))
        .add_startup_system(keyboard_input_system.system())
        .add_system(player_movement.system())
        .add_system(set_sprite.system())
        .add_system_to_stage(CoreStage::PostUpdate, ground_intersect.system())
        .run();
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
