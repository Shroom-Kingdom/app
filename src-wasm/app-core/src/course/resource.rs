use crate::{
    get_surrounding_matrix, grid_to_world, CourseRes, Ground, GroundSurroundingMatrix,
    GroundTileUpdateEvent, GroundVariant, ObjectSpriteHandles, ThemeVariant, Tile, TileNotEditable,
    TileVariant,
};
use app_config::*;
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier::prelude::*;

impl CourseRes {
    pub fn empty(
        commands: &mut Commands,
        theme: ThemeVariant,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
        object_sprite_handles: Res<ObjectSpriteHandles>,
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
                false,
            );
            course.spawn_tile(
                commands,
                &[x, 1],
                &TileVariant::Ground(GroundVariant::Top0),
                &mut events,
                false,
            );
        }
        course.spawn_tile(
            commands,
            &[7, 0],
            &TileVariant::Ground(GroundVariant::Right0),
            &mut events,
            false,
        );
        course.spawn_tile(
            commands,
            &[7, 1],
            &TileVariant::Ground(GroundVariant::TopRight0),
            &mut events,
            false,
        );
        for event in events.into_values() {
            ground_tile_update_events.send(event);
        }

        course.spawn_goal(commands, &object_sprite_handles, ground_tile_update_events);
        course.spawn_goal_drag(commands, asset_server);

        course
    }

    fn load_handles(
        theme: &ThemeVariant,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> (Handle<TextureAtlas>, Handle<TextureAtlas>) {
        let texture_handle = asset_server.load(&format!("MW_Field_{}_0.png", theme.get_name()));
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 48);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let texture_handle_transparent =
            asset_server.load(&format!("0MW_Field_{}_0.png", theme.get_name()));
        let texture_atlas_transparent =
            TextureAtlas::from_grid(texture_handle_transparent, Vec2::new(16.0, 16.0), 16, 48);
        let texture_atlas_handle_transparent = texture_atlases.add(texture_atlas_transparent);
        (texture_atlas_handle, texture_atlas_handle_transparent)
    }

    pub fn spawn_tile(
        &mut self,
        commands: &mut Commands,
        grid_pos: &[i32; 2],
        tile_variant: &TileVariant,
        events: &mut HashMap<Entity, GroundTileUpdateEvent>,
        is_editable: bool,
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

        let surrounding_matrix = if let TileVariant::Ground(_) = tile_variant {
            let surrounding_matrix = get_surrounding_matrix(grid_pos, &mut self.tiles, events);
            Some(GroundSurroundingMatrix(surrounding_matrix))
        } else {
            None
        };

        let sprite = if let Some(surrounding_matrix) = &surrounding_matrix {
            TextureAtlasSprite::new(
                GroundVariant::from_surrounding_matrix(&surrounding_matrix.0)
                    .get_sprite_sheet_index(),
            )
        } else {
            TextureAtlasSprite::new(tile_variant.get_sprite_sheet_index())
        };
        let mut entity_commands = commands.spawn();
        entity_commands
            .insert(RigidBody::Fixed)
            .insert_bundle(SpatialBundle {
                transform: Transform::from_xyz(world_pos.x, world_pos.y, Z_INDEX_TILE),
                visibility: Visibility { is_visible: true },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert(Collider::polyline(
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
                    ))
                    .insert(Friction::new(GROUND_FRICTION))
                    .insert(Ground);
                parent.spawn().insert_bundle(SpriteSheetBundle {
                    transform: Transform {
                        scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.),
                        ..Default::default()
                    },
                    texture_atlas: self.texture_atlas_handle.clone(),
                    sprite,
                    ..Default::default()
                });
                parent
                    .spawn()
                    .insert(Collider::cuboid(
                        TILE_GRID_SIZE * TILE_SIZE,
                        TILE_GRID_SIZE * TILE_SIZE,
                    ))
                    .insert(Friction::new(0.));
            });
        if !is_editable {
            entity_commands.insert(TileNotEditable);
        }

        let entity = entity_commands.id();

        let tile = Tile {
            entity,
            variant: tile_variant.clone(),
            mtrx: surrounding_matrix,
        };
        self.tiles.insert(*grid_pos, tile);
    }
}
