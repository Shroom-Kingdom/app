mod preview;

use app_core::{
    cursor_to_world, world_to_grid, AppState, Course, DespawnTileEvent, Dragging, GameMode,
    GroundTileUpdateEvent, MainCameraQuery, SelectedTile, SpawnTileEvent,
};
use bevy::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTileEvent>()
            .add_event::<DespawnTileEvent>()
            .add_event::<GroundTileUpdateEvent>()
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(AppState::Game)
                    .with_system(spawn_tile)
                    .with_system(preview::spawn_tile_preview),
            );
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_tile(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_query: MainCameraQuery,
    button_query: Query<&Interaction, With<Button>>,
    spawn_tile_events: EventWriter<SpawnTileEvent>,
    despawn_tile_events: EventWriter<DespawnTileEvent>,
    course: Res<Course>,
    game_mode: Res<GameMode>,
    selected_tile: Res<SelectedTile>,
    dragging: Res<Option<Dragging>>,
) {
    if let GameMode::Build { is_editing: true } = *game_mode {
        for interaction in button_query.iter() {
            if interaction == &Interaction::Hovered || interaction == &Interaction::Clicked {
                return;
            }
        }
        if dragging.is_some() {
            return;
        }
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
}

fn send_spawn_tile(
    window: &Window,
    camera_query: &MainCameraQuery,
    mut spawn_tile_events: EventWriter<SpawnTileEvent>,
    course: &Course,
    selected_tile: &SelectedTile,
) {
    let cursor_position = if let Some(cursor_pointer) = window.cursor_position() {
        cursor_pointer
    } else {
        return;
    };

    let world_pos = cursor_to_world(cursor_position, camera_query, window);
    let grid_pos = world_to_grid(&world_pos);
    if let Some(selected_tile) = &selected_tile.0 {
        if !course.tiles.contains_key(&grid_pos) {
            spawn_tile_events.send(SpawnTileEvent {
                tile_variant: selected_tile.clone(),
                grid_pos,
            });
        }
    }
}

fn send_despawn_tile(
    window: &Window,
    camera_query: &MainCameraQuery,
    mut despawn_tile_events: EventWriter<DespawnTileEvent>,
    course: &Course,
) {
    let cursor_position = if let Some(cursor_pointer) = window.cursor_position() {
        cursor_pointer
    } else {
        return;
    };

    let world_pos = cursor_to_world(cursor_position, camera_query, window);
    let grid_pos = world_to_grid(&world_pos);
    if course.tiles.contains_key(&grid_pos) {
        despawn_tile_events.send(DespawnTileEvent {
            grid_pos,
            ..Default::default()
        });
    }
}
