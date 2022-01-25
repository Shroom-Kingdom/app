use app_config::{CAMERA_MIN_X, CAMERA_MIN_Y};
use bevy::{prelude::*, winit::WinitWindows};
use bevy_rapier::prelude::*;
use rapier::pipeline::PhysicsPipeline;
use winit::platform::web::WindowExtWebSys;

pub fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

pub fn setup_graphics(mut commands: Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 10.0;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(CAMERA_MIN_X, CAMERA_MIN_Y, 0.0));
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
        point_light: PointLight {
            intensity: 100_000_000.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(camera);
}

pub fn setup_resolution_scaling(mut windows: ResMut<Windows>, winit_windows: Res<WinitWindows>) {
    let window = windows.get_primary_mut().unwrap();
    let winit_window = winit_windows.get_window(window.id()).unwrap();
    let canvas = winit_window.canvas();
    let rect = canvas.get_bounding_client_rect();
    window.set_resolution(rect.width() as f32, rect.height() as f32);
    window.update_scale_factor_from_backend(rect.width() as f64 / 1020.);
}
