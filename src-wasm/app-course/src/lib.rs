use app_core::{Course, CourseTheme};
use app_tile::{DespawnTileEvent, SpawnTileEvent};
use bevy::prelude::*;

pub struct CoursePlugin;

impl Plugin for CoursePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_to_stage(CoreStage::Last, spawn_tile)
            .add_system_to_stage(CoreStage::Last, despawn_tile);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let course = Course::empty(
        &mut commands,
        CourseTheme::Plain,
        &asset_server,
        &mut texture_atlases,
    );

    commands.insert_resource(course);
}

fn spawn_tile(
    mut commands: Commands,
    mut course: ResMut<Course>,
    mut spawn_tile_events: EventReader<SpawnTileEvent>,
) {
    for SpawnTileEvent {
        grid_pos,
        tile_variant,
    } in spawn_tile_events.iter()
    {
        course.spawn_tile(&mut commands, grid_pos, tile_variant);
    }
}

fn despawn_tile(
    mut commands: Commands,
    mut course: ResMut<Course>,
    mut despawn_tile_events: EventReader<DespawnTileEvent>,
) {
    for DespawnTileEvent { grid_pos } in despawn_tile_events.iter() {
        if let Some(tile) = course.tiles.remove(grid_pos) {
            commands.entity(tile.entity).despawn_recursive();
        }
    }
}
