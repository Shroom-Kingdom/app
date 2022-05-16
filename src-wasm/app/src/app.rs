use crate::graphics::{setup_graphics, setup_resolution_scaling};
use app_assets::{AssetIoTarConfig, AssetIoTarPlugin};
use app_core::{AppState, CorePlugin};
use app_course::CoursePlugin;
use app_menu::MenuPlugin;
use app_player::PlayerPlugin;
use app_tile::TilePlugin;
// use app_debug::{DebugPlugin, DebugPluginState};
use bevy::{input::keyboard::keyboard_input_system, prelude::*};
use bevy_rapier::{plugin::RapierPhysicsPlugin, prelude::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(assets: Vec<u8>) {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF0 as f32 / 255.0,
            0xF0 as f32 / 255.0,
            0xF0 as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .insert_resource(WindowDescriptor {
            canvas: Some("canvas".to_string()),
            ..Default::default()
        })
        // .insert_non_send_resource(DebugPluginState {
        //     debug_state: debug_state.into(),
        //     set_debug_state,
        // })
        .insert_resource(AssetIoTarConfig(assets))
        .insert_resource(State::new(AppState::Setup))
        .add_plugins_with(bevy::DefaultPlugins, |group| {
            group.add_before::<bevy::asset::AssetPlugin, _>(AssetIoTarPlugin)
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::with_physics_scale(10.))
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(CorePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TilePlugin)
        .add_plugin(CoursePlugin)
        .add_plugin(MenuPlugin)
        // .add_plugin(DebugPlugin)
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_graphics))
        .add_startup_system(setup_resolution_scaling)
        .add_startup_system(keyboard_input_system)
        .add_system_set_to_stage(CoreStage::First, State::<AppState>::get_driver())
        .add_system_set_to_stage(CoreStage::PreUpdate, State::<AppState>::get_driver())
        .add_system_set_to_stage(CoreStage::Update, State::<AppState>::get_driver())
        .add_system_set_to_stage(CoreStage::PostUpdate, State::<AppState>::get_driver())
        .add_system_set_to_stage(CoreStage::Last, State::<AppState>::get_driver())
        .run();
}
