use app_core::{
    AppLabel, AppState, Course, CourseTheme, GroundSurroundingMatrix, GroundVariant, SelectedTile,
    Tile, TileVariant,
};
use app_tile::{DespawnTileEvent, SpawnTileEvent};
use bevy::prelude::*;

pub struct CoursePlugin;

impl Plugin for CoursePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .label(AppLabel::InsertCourse)
                .with_system(setup),
        )
        .add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::on_update(AppState::Game).with_system(spawn_tile),
        )
        .add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::on_update(AppState::Game).with_system(despawn_tile),
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    let course = Course::empty(
        &mut commands,
        CourseTheme::Plain,
        &asset_server,
        &mut texture_atlases,
    );

    commands.insert_resource(course);
    selected_tile.0 = Some(TileVariant::Ground(GroundVariant::default()));
}

fn spawn_tile(
    mut commands: Commands,
    mut course: ResMut<Course>,
    mut spawn_tile_events: EventReader<SpawnTileEvent>,
    mut query: Query<(&Children, &mut GroundSurroundingMatrix)>,
    mut child_query: Query<&mut TextureAtlasSprite>,
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
            Some((&mut query, &mut child_query)),
            None,
        );
    }
}

fn despawn_tile(
    mut commands: Commands,
    mut course: ResMut<Course>,
    mut despawn_tile_events: EventReader<DespawnTileEvent>,
    mut query: Query<(&Children, &mut GroundSurroundingMatrix)>,
    mut child_query: Query<&mut TextureAtlasSprite>,
) {
    for DespawnTileEvent { grid_pos } in despawn_tile_events.iter() {
        if let Some(tile) = course.tiles.remove(grid_pos) {
            for x in grid_pos[0] - 1..=grid_pos[0] + 1 {
                for y in grid_pos[1] - 1..=grid_pos[1] + 1 {
                    let pos = [x, y];
                    if &pos == grid_pos {
                        continue;
                    }
                    if let Some(Tile {
                        variant: TileVariant::Ground(ground_variant),
                        entity,
                    }) = course.tiles.get_mut(&pos)
                    {
                        let (children, mut mtrx) = query.get_mut(*entity).unwrap();
                        mtrx.0[(y - grid_pos[1] + 1) as usize][(grid_pos[0] - x + 1) as usize] =
                            false;
                        let child = children[1];
                        *ground_variant = GroundVariant::from_surrounding_matrix(&mtrx.0);
                        let mut sprite = child_query.get_mut(child).unwrap();
                        *sprite = TextureAtlasSprite::new(ground_variant.get_sprite_sheet_index());
                    }
                }
            }
            // TODO does not despawn rigid body. bevy_rapier crashes
            commands.entity(tile.entity).despawn_recursive();
        }
    }
}
