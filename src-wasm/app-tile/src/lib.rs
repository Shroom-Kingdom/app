use app_config::{GRID_SIZE, RAPIER_SCALE};
use app_core::{AppState, Course, SelectedTile, TileVariant};
use bevy::{
    prelude::*,
    render::{camera::Camera, primitives::Frustum},
};

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
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(AppState::Game).with_system(spawn_tile),
            );
    }
}

fn spawn_tile(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_query: Query<(&Transform, &Camera), With<Frustum>>,
    spawn_tile_events: EventWriter<SpawnTileEvent>,
    despawn_tile_events: EventWriter<DespawnTileEvent>,
    course: Res<Course>,
    selected_tile: Res<SelectedTile>,
) {
    let window = windows.get_primary().unwrap();
    if mouse_button_input.pressed(MouseButton::Left) {
        send_spawn_tile(
            window,
            &camera_query,
            spawn_tile_events,
            &course,
            &selected_tile,
        );
    }
    if mouse_button_input.pressed(MouseButton::Right) {
        send_despawn_tile(window, &camera_query, despawn_tile_events, &course);
    }
}

fn send_spawn_tile(
    window: &Window,
    camera_query: &Query<(&Transform, &Camera), With<Frustum>>,
    mut spawn_tile_events: EventWriter<SpawnTileEvent>,
    course: &Course,
    selected_tile: &SelectedTile,
) {
    let cursor_position = if let Some(cursor_pointer) = window.cursor_position() {
        cursor_pointer
    } else {
        return;
    };

    let grid_pos = cursor_to_grid(cursor_position, camera_query, window);
    if !course.tiles.contains_key(&grid_pos) {
        spawn_tile_events.send(SpawnTileEvent {
            tile_variant: selected_tile.0.clone(),
            grid_pos,
        });
    }
}

fn send_despawn_tile(
    window: &Window,
    camera_query: &Query<(&Transform, &Camera), With<Frustum>>,
    mut despawn_tile_events: EventWriter<DespawnTileEvent>,
    course: &Course,
) {
    let cursor_position = if let Some(cursor_pointer) = window.cursor_position() {
        cursor_pointer
    } else {
        return;
    };

    let grid_pos = cursor_to_grid(cursor_position, camera_query, window);
    if course.tiles.contains_key(&grid_pos) {
        despawn_tile_events.send(DespawnTileEvent { grid_pos });
    }
}

fn cursor_to_grid(
    cursor: Vec2,
    camera_query: &Query<(&Transform, &Camera), With<Frustum>>,
    window: &Window,
) -> [i32; 2] {
    let cursor = cursor / Vec2::new(window.width(), window.height());

    let (transform, camera) = camera_query.single();

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
