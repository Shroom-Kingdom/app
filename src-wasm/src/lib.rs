use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = setupPanicHook)]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once();
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() {
    App::build()
        .add_plugins(bevy_webgl2::DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("branding/icon.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });
}
