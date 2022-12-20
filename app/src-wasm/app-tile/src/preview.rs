use app_config::*;
use app_core::{
    cursor_to_world, grid_to_world, world_to_grid, CourseRes, GameMode, GroundSurroundingMatrix,
    MainCameraQuery, SelectedTile, TilePlacePreview, TilePreview,
};
use bevy::{prelude::*, render::primitives::Frustum};
use bevy_rapier::prelude::*;
use shrm_core::{GroundVariant, TileVariant};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub(crate) fn spawn_tile_preview(
    mut cursor_events: EventReader<CursorMoved>,
    windows: Res<Windows>,
    camera_query: MainCameraQuery,
    course: Res<CourseRes>,
    game_mode: Res<GameMode>,
    mut commands: Commands,
    selected_tile: Res<SelectedTile>,
    mut tile_place_preview: ResMut<TilePlacePreview>,
    mut query: Query<
        (&mut Transform, &mut TextureAtlasSprite),
        (With<TilePreview>, Without<Frustum>),
    >,
) {
    if let Some(tile_variant) = &selected_tile.0 {
        if let GameMode::Build { is_editing: true } = *game_mode {
            let window = windows.get_primary().unwrap();
            for cursor_moved in cursor_events.iter() {
                let world_pos = cursor_to_world(cursor_moved.position, &camera_query, window);
                let grid_pos = world_to_grid(&world_pos);

                if grid_pos[0] < 0
                    || grid_pos[1] < 0
                    || grid_pos[0] >= MAX_COURSE_X
                    || grid_pos[1] >= MAX_COURSE_Y
                {
                    if let Some((entity, _)) = tile_place_preview.0 {
                        commands.entity(entity).despawn_recursive();
                        tile_place_preview.0 = None;
                    }
                } else if course.tiles.get(&grid_pos).is_none() {
                    let world_pos = grid_to_world(&grid_pos);
                    let surrounding_matrix: Option<GroundSurroundingMatrix> =
                        if let TileVariant::Ground(_) = tile_variant {
                            // let surrounding_matrix =
                            //     get_surrounding_matrix(&grid_pos, &mut course.tiles);
                            // Some(GroundSurroundingMatrix(surrounding_matrix))
                            // TODO tile preview not working anyway
                            None
                        } else {
                            None
                        };
                    if let Some((entity, tile_pos)) = &mut tile_place_preview.0 {
                        if *tile_pos != grid_pos {
                            if let Ok(mut transform) = query.get_component_mut::<Transform>(*entity)
                            {
                                transform.translation.x = world_pos.x;
                                transform.translation.y = world_pos.y;
                                *tile_pos = grid_pos;
                                if let Some(surrounding_matrix) = surrounding_matrix {
                                    let sprite = TextureAtlasSprite::new(
                                        GroundVariant::from_surrounding_matrix(
                                            &surrounding_matrix.0,
                                        )
                                        .get_sprite_sheet_index(),
                                    );
                                    let mut texture_atlas_sprite = query
                                        .get_component_mut::<TextureAtlasSprite>(*entity)
                                        .unwrap();
                                    *texture_atlas_sprite = sprite;
                                }
                            }
                        }
                    } else {
                        let sprite = if let Some(surrounding_matrix) = &surrounding_matrix {
                            TextureAtlasSprite::new(
                                GroundVariant::from_surrounding_matrix(&surrounding_matrix.0)
                                    .get_sprite_sheet_index(),
                            )
                        } else {
                            TextureAtlasSprite::new(tile_variant.get_sprite_sheet_index())
                        };
                        let entity = commands
                            .spawn((
                                RigidBody::Fixed,
                                SpriteSheetBundle {
                                    transform: Transform {
                                        translation: Vec3::new(
                                            world_pos.x,
                                            world_pos.y,
                                            Z_INDEX_TILE,
                                        ),
                                        scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.),
                                        ..Default::default()
                                    },
                                    texture_atlas: course.texture_atlas_handle_transparent.clone(),
                                    sprite,
                                    ..Default::default()
                                },
                                TilePreview,
                            ))
                            .id();
                        tile_place_preview.0 = Some((entity, grid_pos));
                    }
                } else if let Some((entity, _)) = tile_place_preview.0 {
                    commands.entity(entity).despawn_recursive();
                    tile_place_preview.0 = None;
                }
            }
        }
    }
}
