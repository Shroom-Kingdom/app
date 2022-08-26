use app_config::*;
use app_core::{
    get_surrounding_matrix, grid_to_world, AppState, Course, GameMode, GroundSurroundingMatrix,
    GroundVariant, SelectedTile, TilePlacePreview, TilePreview, TileVariant,
};
use bevy::{
    prelude::*,
    render::{camera::Camera, primitives::Frustum},
};
use bevy_rapier::prelude::RigidBody;
use either::Either;
use std::cell::RefCell;

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
                SystemSet::on_update(AppState::Game)
                    .with_system(spawn_tile)
                    .with_system(spawn_tile_preview),
            );
    }
}

type CameraQuery<'w, 's, 'q> = Query<'w, 's, (&'q Transform, &'q Camera), With<Frustum>>;

#[allow(clippy::too_many_arguments)]
fn spawn_tile(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_query: CameraQuery,
    button_query: Query<&Interaction, With<Button>>,
    spawn_tile_events: EventWriter<SpawnTileEvent>,
    despawn_tile_events: EventWriter<DespawnTileEvent>,
    course: Res<Course>,
    selected_tile: Res<SelectedTile>,
) {
    if let GameMode::Build { is_editing: true } = course.game_mode {
        for interaction in button_query.iter() {
            if interaction == &Interaction::Hovered || interaction == &Interaction::Clicked {
                return;
            }
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
    camera_query: &CameraQuery,
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
    camera_query: &CameraQuery,
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

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn spawn_tile_preview(
    mut cursor_events: EventReader<CursorMoved>,
    windows: Res<Windows>,
    camera_query: CameraQuery,
    course: Res<Course>,
    mut commands: Commands,
    selected_tile: Res<SelectedTile>,
    mut tile_place_preview: ResMut<TilePlacePreview>,
    mut query: Query<
        (&mut Transform, &mut TextureAtlasSprite),
        (With<TilePreview>, Without<Frustum>),
    >,
) {
    if let Some(tile_variant) = &selected_tile.0 {
        if let GameMode::Build { is_editing: true } = course.game_mode {
            let window = windows.get_primary().unwrap();
            for cursor_moved in cursor_events.iter() {
                let grid_pos = cursor_to_grid(cursor_moved.position, &camera_query, window);

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
                    let surrounding_matrix = if let TileVariant::Ground(_) = tile_variant {
                        let surrounding_matrix = get_surrounding_matrix(
                            &grid_pos,
                            RefCell::new(Either::Left(&course.tiles)),
                        );
                        Some(GroundSurroundingMatrix(surrounding_matrix))
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
                            .spawn()
                            .insert(RigidBody::Fixed)
                            .insert_bundle(SpriteSheetBundle {
                                transform: Transform {
                                    translation: Vec3::new(world_pos.x, world_pos.y, Z_INDEX_TILE),
                                    scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.),
                                    ..Default::default()
                                },
                                texture_atlas: course.texture_atlas_handle_transparent.clone(),
                                sprite,
                                ..Default::default()
                            })
                            .insert(TilePreview)
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

fn cursor_to_grid(cursor: Vec2, camera_query: &CameraQuery, window: &Window) -> [i32; 2] {
    let cursor = cursor / Vec2::new(window.width(), window.height());

    let (transform, camera) = camera_query.single();

    let camera_position = transform.compute_matrix();
    let projection_matrix = camera.projection_matrix();

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
