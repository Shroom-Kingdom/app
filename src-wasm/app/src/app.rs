use crate::rapier::{enable_physics_profiling, setup_graphics, setup_physics};
use app_assets::{AssetIoTarConfig, AssetIoTarPlugin};
use app_debug::{DebugPlugin, DebugPluginState};
use bevy::prelude::*;
use bevy_rapier::{
    physics::{NoUserData, RapierPhysicsPlugin},
    render::RapierRenderPlugin,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(debug_state: app_debug::DebugState, set_debug_state: js_sys::Function) {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .insert_non_send_resource(DebugPluginState {
            debug_state: debug_state.into(),
            set_debug_state,
        })
        .insert_resource(AssetIoTarConfig(
            // TODO bytes from GUI
            include_bytes!("../../../../../../shroom-assets/assets.tar").to_vec(),
        ))
        .add_plugins_with(bevy_webgl2::DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(AssetIoTarPlugin)
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_startup_system(enable_physics_profiling.system())
        .run();
}
