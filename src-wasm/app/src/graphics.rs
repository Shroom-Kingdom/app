use app_config::{CAMERA_MIN_X, CAMERA_MIN_Y, CAMERA_SCALE};
use bevy::{prelude::*, winit::WinitWindows};
use winit::platform::web::WindowExtWebSys;

pub fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(CAMERA_MIN_X, CAMERA_MIN_Y, 0.0),
                scale: Vec3::new(CAMERA_SCALE, CAMERA_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UiCameraConfig { show_ui: true });
}

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
        point_light: PointLight {
            intensity: 100_000_000.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn setup_resolution_scaling(mut windows: ResMut<Windows>, winit_windows: Res<WinitWindows>) {
    let window = windows.get_primary_mut().unwrap();
    let winit_window = winit_windows.get_window(window.id()).unwrap();
    let canvas = winit_window.canvas();
    let rect = canvas.get_bounding_client_rect();
    window.set_resolution(rect.width() as f32, rect.height() as f32);
    window.update_scale_factor_from_backend(rect.width() as f64 / 1020.);
}
