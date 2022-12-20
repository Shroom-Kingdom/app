use crate::{
    get_surrounding_matrix, grid_to_world, CourseRes, Ground, GroundSurroundingMatrix,
    GroundTileUpdateEvent, ObjectSpriteHandles, Tile, TileNotEditable,
};
use app_config::*;
use bevy::prelude::*;
use bevy_rapier::prelude::*;
use shrm_core::{Course, GroundVariant, ThemeVariant, TileVariant};
use std::collections::HashMap;

impl CourseRes {
    pub fn empty(
        commands: &mut Commands,
        theme: ThemeVariant,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
        object_sprite_handles: &ObjectSpriteHandles,
        ground_tile_update_events: &mut EventWriter<GroundTileUpdateEvent>,
    ) -> Self {
        let (texture_atlas_handle, texture_atlas_handle_transparent) =
            Self::load_handles(&theme, asset_server, texture_atlases);
        let mut course = CourseRes {
            texture_atlas_handle,
            texture_atlas_handle_transparent,
            tiles: HashMap::default(),
            theme,
            goal_pos_x: 32,
        };

        let mut events = HashMap::new();
        for x in 0..7 {
            course.spawn_tile(
                commands,
                &[x, 0],
                &TileVariant::Ground(GroundVariant::Full0),
                &mut events,
            );
            course.spawn_tile(
                commands,
                &[x, 1],
                &TileVariant::Ground(GroundVariant::Top0),
                &mut events,
            );
        }
        course.spawn_tile(
            commands,
            &[7, 0],
            &TileVariant::Ground(GroundVariant::Right0),
            &mut events,
        );
        course.spawn_tile(
            commands,
            &[7, 1],
            &TileVariant::Ground(GroundVariant::TopRight0),
            &mut events,
        );
        for event in events.into_values() {
            ground_tile_update_events.send(event);
        }

        course.spawn_goal(commands, object_sprite_handles, ground_tile_update_events);
        course.spawn_goal_drag(commands, asset_server);

        course
    }

    pub fn load(
        commands: &mut Commands,
        course: &Course,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
        object_sprite_handles: &ObjectSpriteHandles,
        ground_tile_update_events: &mut EventWriter<GroundTileUpdateEvent>,
    ) -> Self {
        let (texture_atlas_handle, texture_atlas_handle_transparent) =
            Self::load_handles(&course.theme, asset_server, texture_atlases);
        let mut course_res = CourseRes {
            texture_atlas_handle,
            texture_atlas_handle_transparent,
            tiles: HashMap::default(),
            theme: course.theme.clone(),
            goal_pos_x: course.goal_pos_x,
        };

        let mut events = HashMap::new();
        for (grid_pos, tile) in course.tiles.clone() {
            course_res.spawn_tile(commands, &grid_pos, &tile, &mut events);
        }

        course_res.spawn_goal(commands, object_sprite_handles, ground_tile_update_events);
        course_res.spawn_goal_drag(commands, asset_server);

        course_res
    }

    fn load_handles(
        theme: &ThemeVariant,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> (Handle<TextureAtlas>, Handle<TextureAtlas>) {
        let texture_handle = asset_server.load(format!("MW_Field_{}_0.png", theme.get_name()));
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 48, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let texture_handle_transparent =
            asset_server.load(format!("0MW_Field_{}_0.png", theme.get_name()));
        let texture_atlas_transparent = TextureAtlas::from_grid(
            texture_handle_transparent,
            Vec2::new(16.0, 16.0),
            16,
            48,
            None,
            None,
        );
        let texture_atlas_handle_transparent = texture_atlases.add(texture_atlas_transparent);
        (texture_atlas_handle, texture_atlas_handle_transparent)
    }

    pub fn spawn_tile(
        &mut self,
        commands: &mut Commands,
        grid_pos: &[i32; 2],
        tile_variant: &TileVariant,
        events: &mut HashMap<Entity, GroundTileUpdateEvent>,
    ) {
        if self.tiles.contains_key(grid_pos) {
            return;
        }

        let world_pos = grid_to_world(grid_pos);
        if grid_pos[0] < 0
            || grid_pos[1] < 0
            || grid_pos[0] > self.goal_pos_x + MAX_COURSE_GOAL_OFFSET_X
            || grid_pos[1] > MAX_COURSE_Y
        {
            return;
        }

        let mut tile_variant = tile_variant.clone();
        let (surrounding_matrix, ground_variant) =
            if let TileVariant::Ground(ground_variant) = &mut tile_variant {
                let surrounding_matrix = get_surrounding_matrix(grid_pos, &mut self.tiles, events);
                *ground_variant = GroundVariant::from_surrounding_matrix(&surrounding_matrix);
                (
                    Some(GroundSurroundingMatrix(surrounding_matrix)),
                    Some(ground_variant.clone()),
                )
            } else {
                (None, None)
            };

        let sprite = if let Some(ground_variant) = ground_variant {
            TextureAtlasSprite::new(ground_variant.get_sprite_sheet_index())
        } else {
            TextureAtlasSprite::new(tile_variant.get_sprite_sheet_index())
        };
        let mut entity_commands = commands.spawn_empty();
        entity_commands
            .insert((
                RigidBody::Fixed,
                SpatialBundle {
                    transform: Transform::from_xyz(world_pos.x, world_pos.y, Z_INDEX_TILE),
                    visibility: Visibility { is_visible: true },
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    Collider::polyline(
                        vec![
                            Vec2::new(
                                (-TILE_SIZE + TILE_COLLIDER_SUB - GRID_MARGIN) * RAPIER_SCALE
                                    + GROUND_PADDING,
                                (TILE_SIZE - TILE_COLLIDER_SUB
                                    + GROUND_MARGIN_MULTIPLIER * GRID_MARGIN
                                    + 0.02)
                                    * RAPIER_SCALE,
                            ),
                            Vec2::new(
                                (TILE_SIZE - TILE_COLLIDER_SUB + GRID_MARGIN) * RAPIER_SCALE
                                    - GROUND_PADDING,
                                (TILE_SIZE - TILE_COLLIDER_SUB
                                    + GROUND_MARGIN_MULTIPLIER * GRID_MARGIN
                                    + 0.02)
                                    * RAPIER_SCALE,
                            ),
                        ],
                        None,
                    ),
                    Friction::new(GROUND_FRICTION),
                    Ground,
                ));
                parent.spawn(SpriteSheetBundle {
                    transform: Transform {
                        scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.),
                        ..Default::default()
                    },
                    texture_atlas: self.texture_atlas_handle.clone(),
                    sprite,
                    ..Default::default()
                });
                parent.spawn((
                    Collider::cuboid(TILE_GRID_SIZE * TILE_SIZE, TILE_GRID_SIZE * TILE_SIZE),
                    Friction::new(0.),
                ));
            });
        let is_goal = grid_pos[0] >= self.goal_pos_x && grid_pos[1] <= 1;
        let is_start = grid_pos[0] < 8 && grid_pos[1] <= 1;
        if is_goal || is_start {
            entity_commands.insert(TileNotEditable);
        }

        let entity = entity_commands.id();

        let tile = Tile {
            entity,
            variant: tile_variant,
            mtrx: surrounding_matrix,
        };
        self.tiles.insert(*grid_pos, tile);
    }
}
