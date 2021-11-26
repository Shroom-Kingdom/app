use app_config::{GRID_SIZE, RAPIER_SCALE};
use app_core::{Course, TileVariant};
use bevy::{prelude::*, render::camera::Camera, winit::WinitWindows};
use web_sys::{HtmlCanvasElement, HtmlElement};
use winit::platform::web::WindowExtWebSys;

pub struct SpawnTileEvent {
    pub tile_variant: TileVariant,
    pub grid_pos: [i32; 2],
}

pub struct DespawnTileEvent {
    pub grid_pos: [i32; 2],
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTileEvent>()
            .add_event::<DespawnTileEvent>()
            .add_system_to_stage(CoreStage::PostUpdate, spawn_tile);
    }
}

fn spawn_tile(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    winit_windows: Res<WinitWindows>,
    camera_query: Query<(&Transform, &Camera)>,
    spawn_tile_events: EventWriter<SpawnTileEvent>,
    despawn_tile_events: EventWriter<DespawnTileEvent>,
    course: Res<Course>,
) {
    let window = windows.get_primary().unwrap();
    let winit_window = winit_windows.get_window(window.id()).unwrap();
    let canvas = winit_window.canvas();
    if mouse_button_input.just_pressed(MouseButton::Left) {
        send_spawn_tile(window, &canvas, &camera_query, spawn_tile_events, &course);
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        send_despawn_tile(window, &canvas, &camera_query, despawn_tile_events, &course);
    }
}

fn send_spawn_tile(
    window: &Window,
    canvas: &HtmlCanvasElement,
    camera_query: &Query<(&Transform, &Camera)>,
    mut spawn_tile_events: EventWriter<SpawnTileEvent>,
    course: &Course,
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

    let grid_pos = cursor_to_grid(cursor_position, camera_query, &body, canvas);
    if !course.tiles.contains_key(&grid_pos) {
        spawn_tile_events.send(SpawnTileEvent {
            tile_variant: TileVariant::Block,
            grid_pos,
        });
    }
}

fn send_despawn_tile(
    window: &Window,
    canvas: &HtmlCanvasElement,
    camera_query: &Query<(&Transform, &Camera)>,
    mut despawn_tile_events: EventWriter<DespawnTileEvent>,
    course: &Course,
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

    let grid_pos = cursor_to_grid(cursor_position, camera_query, &body, canvas);
    if course.tiles.contains_key(&grid_pos) {
        despawn_tile_events.send(DespawnTileEvent { grid_pos });
    }
}

fn cursor_to_grid(
    cursor: Vec2,
    camera_query: &Query<(&Transform, &Camera)>,
    body: &HtmlElement,
    canvas: &HtmlCanvasElement,
) -> [i32; 2] {
    let diff = Vec2::new(
        0.,
        (body.offset_height() - canvas.offset_height() - 5) as f32,
    );
    let cursor =
        (cursor - diff) / Vec2::new(canvas.offset_width() as f32, canvas.offset_height() as f32);

    let (transform, camera) = camera_query.single().expect("main camera not found");

    let camera_position = transform.compute_matrix();
    let projection_matrix = camera.projection_matrix;

    let cursor_ndc = (cursor) * 2.0 - Vec2::from([1.0, 1.0]);
    let cursor_pos_ndc_far = cursor_ndc.extend(1.0);

    let ndc_to_world = camera_position * projection_matrix.inverse();
    let cursor_pos_far = ndc_to_world.project_point3(cursor_pos_ndc_far);

    let world_pos = [
        cursor_pos_far.truncate().x / RAPIER_SCALE,
        cursor_pos_far.truncate().y / RAPIER_SCALE,
    ];

    [
        (world_pos[0] / GRID_SIZE).round() as i32,
        (world_pos[1] / GRID_SIZE).round() as i32,
    ]
}
