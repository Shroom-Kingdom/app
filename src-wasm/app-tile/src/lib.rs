use app_config::RAPIER_SCALE;
use bevy::{prelude::*, render::camera::Camera, winit::WinitWindows};
use web_sys::{HtmlCanvasElement, HtmlElement};
use winit::platform::web::WindowExtWebSys;

#[derive(Clone, Debug)]
pub struct Tile {
    pub variant: TileVariant,
}

#[derive(Clone, Debug)]
pub enum TileVariant {
    Block,
}

pub struct SpawnTileEvent {
    pub tile: Tile,
    pub world_pos: Vec2,
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTileEvent>()
            .add_system_to_stage(CoreStage::PostUpdate, spawn_tile);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_tile(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    winit_windows: Res<WinitWindows>,
    camera_query: Query<(&Transform, &Camera)>,
    spawn_tile_events: EventWriter<SpawnTileEvent>,
) {
    let window = windows.get_primary().unwrap();
    let winit_window = winit_windows.get_window(window.id()).unwrap();
    let canvas = winit_window.canvas();
    if mouse_button_input.just_pressed(MouseButton::Left) {
        send_spawn_tile(window, &canvas, &camera_query, spawn_tile_events);
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        web_sys::console::log_1(&"PRESSED RIGHT".into());
    }
}

fn send_spawn_tile(
    window: &Window,
    canvas: &HtmlCanvasElement,
    camera_query: &Query<(&Transform, &Camera)>,
    mut spawn_tile_events: EventWriter<SpawnTileEvent>,
) {
    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();
    let cursor_position = if let Some(cursor_pointer) = window.cursor_position() {
        cursor_pointer
    } else {
        return;
    };

    let world_pos = cursor_to_world(cursor_position, camera_query, &body, canvas);

    spawn_tile_events.send(SpawnTileEvent {
        tile: Tile {
            variant: TileVariant::Block,
        },
        world_pos,
    });
}

fn cursor_to_world(
    cursor: Vec2,
    camera_query: &Query<(&Transform, &Camera)>,
    body: &HtmlElement,
    canvas: &HtmlCanvasElement,
) -> Vec2 {
    let diff = Vec2::new(
        0.,
        (body.offset_height() - canvas.offset_height() - 5) as f32,
    );
    let cursor =
        (cursor - diff) / Vec2::new(canvas.offset_width() as f32, canvas.offset_height() as f32);

    let (transform, camera) = camera_query.single().expect("main camera not found");

    let camera_position = transform.compute_matrix();
    let projection_matrix = camera.projection_matrix;

    // Normalized device coordinate cursor position from (-1, -1, -1) to (1, 1, 1)
    let cursor_ndc = (cursor) * 2.0 - Vec2::from([1.0, 1.0]);
    let cursor_pos_ndc_far = cursor_ndc.extend(1.0);

    let ndc_to_world = camera_position * projection_matrix.inverse();
    let cursor_pos_far = ndc_to_world.project_point3(cursor_pos_ndc_far);

    [
        cursor_pos_far.truncate().x / RAPIER_SCALE,
        cursor_pos_far.truncate().y / RAPIER_SCALE,
    ]
    .into()
}
