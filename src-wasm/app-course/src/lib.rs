mod grid;

use app_config::*;
use app_core::{
    AppLabel, AppState, Course, DespawnTileEvent, GroundTileUpdateEvent, GroundVariant,
    ObjectSpriteHandles, SelectedTile, SpawnTileEvent, ThemeSpriteHandles, ThemeVariant, Tile,
    TileNotEditable, TileVariant,
};
use bevy::prelude::*;
use grid::{setup_grid, toggle_grid};

pub struct CoursePlugin;

impl Plugin for CoursePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .label(AppLabel::InsertCourse)
                .with_system(setup)
                .with_system(setup_grid),
        )
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(toggle_grid))
        .add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_tile)
                .label(AppLabel::SpawnTile)
                .after(AppLabel::DespawnTile),
        )
        .add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::on_update(AppState::Game)
                .with_system(despawn_tile)
                .label(AppLabel::DespawnTile),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut selected_tile: ResMut<SelectedTile>,
    theme_sprite_handles: Res<ThemeSpriteHandles>,
    object_sprite_handles: Res<ObjectSpriteHandles>,
    mut ground_tile_update_events: EventWriter<GroundTileUpdateEvent>,
) {
    let course = Course::empty(
        &mut commands,
        ThemeVariant::Plain,
        &asset_server,
        &mut texture_atlases,
        object_sprite_handles,
        &mut Some(&mut ground_tile_update_events),
    );

    commands.insert_resource(course);
    selected_tile.0 = Some(TileVariant::Ground(GroundVariant::default()));

    let texture = theme_sprite_handles
        .0
        .get(&ThemeVariant::Plain)
        .unwrap()
        .clone();

    let scale = 2.5;
    let image_size = 512.;
    let offset = -50.;
    for i in (1..=15).step_by(2) {
        commands.spawn_bundle(SpriteBundle {
            texture: texture.clone(),
            transform: Transform {
                translation: Vec3::new(
                    scale / 2. * image_size * (i as f32) + offset,
                    image_size,
                    Z_INDEX_BACKGROUND,
                ),
                scale: Vec3::new(scale, scale, 0.),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn spawn_tile(
    mut commands: Commands,
    mut course: ResMut<Course>,
    mut spawn_tile_events: EventReader<SpawnTileEvent>,
    mut ground_tile_update_events: EventWriter<GroundTileUpdateEvent>,
) {
    for SpawnTileEvent {
        grid_pos,
        tile_variant,
    } in spawn_tile_events.iter()
    {
        course.spawn_tile(
            &mut commands,
            grid_pos,
            tile_variant,
            &mut Some(&mut ground_tile_update_events),
            true,
        );
    }
}

fn despawn_tile(
    mut commands: Commands,
    mut course: ResMut<Course>,
    mut despawn_tile_events: EventReader<DespawnTileEvent>,
    mut test_query: Query<Entity, Without<TileNotEditable>>,
    mut ground_tile_update_events: EventWriter<GroundTileUpdateEvent>,
) {
    for DespawnTileEvent { grid_pos, force } in despawn_tile_events.iter() {
        if let Some(tile) = course.tiles.remove(grid_pos) {
            if !force && test_query.get_mut(tile.entity).is_err() {
                course.tiles.insert(*grid_pos, tile);
                continue;
            }
            for x in grid_pos[0] - 1..=grid_pos[0] + 1 {
                for y in grid_pos[1] - 1..=grid_pos[1] + 1 {
                    let pos = [x, y];
                    if &pos == grid_pos {
                        continue;
                    }
                    if let Some(Tile {
                        entity,
                        variant: TileVariant::Ground(ground_variant),
                        mtrx: Some(mtrx),
                    }) = course.tiles.get_mut(&pos)
                    {
                        mtrx.0[(y - grid_pos[1] + 1) as usize][(grid_pos[0] - x + 1) as usize] =
                            false;
                        *ground_variant = GroundVariant::from_surrounding_matrix(&mtrx.0);
                        ground_tile_update_events.send(GroundTileUpdateEvent {
                            entity: *entity,
                            index: ground_variant.get_sprite_sheet_index(),
                        });
                    }
                }
            }
            commands.entity(tile.entity).despawn_recursive();
        }
    }
}
