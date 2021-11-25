use crate::graphics::{enable_physics_profiling, setup_graphics};
use app_assets::{AssetIoTarConfig, AssetIoTarPlugin};
use app_core::{AppState, CorePlugin};
use app_course::CoursePlugin;
use app_ground::GroundPlugin;
use app_player::CharacterPlugin;
use app_tile::TilePlugin;
// use app_debug::{DebugPlugin, DebugPluginState};
use bevy::{input::keyboard::keyboard_input_system, prelude::*};
use bevy_rapier::{
    physics::{NoUserData, RapierPhysicsPlugin},
    render::RapierRenderPlugin,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(assets: Vec<u8>) {
    App::new()
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
        .add_state(AppState::Setup)
        .add_plugins_with(bevy_webgl2::DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(AssetIoTarPlugin)
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(CorePlugin)
        .add_plugin(CharacterPlugin)
        .add_plugin(GroundPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(CoursePlugin)
        // .add_plugin(DebugPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(enable_physics_profiling.system())
        .add_startup_system(keyboard_input_system.system())
        .run();
}
