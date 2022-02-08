pub(crate) mod sprites;
pub(crate) mod tile;

use crate::{grid_to_world, Ground, GroundSurroundingMatrix, GroundVariant, Tile, TileVariant};
use app_config::{
    GRID_MARGIN, GROUND_FRICTION, GROUND_MARGIN_MULTIPLIER, TILE_COLLIDER_SUB, TILE_SIZE,
};
use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use bevy_rapier::{na::Point2, prelude::*};

#[derive(Debug, TypeUuid)]
#[uuid = "81a23571-1f35-4f20-b1ea-30e5c2612049"]
pub struct Course {
    pub texture_atlas_handle: Handle<TextureAtlas>,
    pub tiles: HashMap<[i32; 2], Tile>,
    pub theme: CourseTheme,
}

#[derive(Debug)]
pub enum CourseTheme {
    Plain,
}

impl CourseTheme {
    pub fn get_asset_str(&self) -> &str {
        match self {
            CourseTheme::Plain => "MW_Field_plain_0.png",
        }
    }
}

impl Course {
    pub fn empty(
        commands: &mut Commands,
        theme: CourseTheme,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load(theme.get_asset_str());
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 48);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let mut course = Course {
            texture_atlas_handle,
            tiles: HashMap::default(),
            theme,
        };

        for x in 0..7 {
            course.spawn_tile(
                commands,
                &[x, 0],
                &TileVariant::Ground(GroundVariant::Full0),
                None,
                Some([[true, true, true], [true, false, true], [true, true, true]]),
            );
            course.spawn_tile(
                commands,
                &[x, 1],
                &TileVariant::Ground(GroundVariant::Top0),
                None,
                Some([
                    [x == 0, false, false],
                    [true, false, true],
                    [true, true, true],
                ]),
            );
        }
        course.spawn_tile(
            commands,
            &[7, 0],
            &TileVariant::Ground(GroundVariant::Right0),
            None,
            Some([
                [true, true, false],
                [true, false, false],
                [true, true, true],
            ]),
        );
        course.spawn_tile(
            commands,
            &[7, 1],
            &TileVariant::Ground(GroundVariant::TopRight0),
            None,
            Some([
                [false, false, false],
                [true, false, false],
                [true, true, false],
            ]),
        );

        course
    }

    #[allow(clippy::type_complexity)]
    pub fn spawn_tile(
        &mut self,
        commands: &mut Commands,
        grid_pos: &[i32; 2],
        tile_variant: &TileVariant,
        mut queries: Option<(
            &mut Query<(&Children, &mut GroundSurroundingMatrix)>,
            &mut Query<&mut TextureAtlasSprite>,
        )>,
        surrounding_matrix: Option<[[bool; 3]; 3]>,
    ) {
        let world_pos = grid_to_world(grid_pos);
        if self.tiles.contains_key(grid_pos) {
            return;
        }

        // TODO min max course pos
        if grid_pos[0] < 0 || grid_pos[1] < 0 || grid_pos[0] > 150 || grid_pos[1] > 24 {
            return;
        }

        let surrounding_matrix = if let Some(surrounding_matrix) = surrounding_matrix {
            Some(GroundSurroundingMatrix(surrounding_matrix))
        } else if let TileVariant::Ground(_) = tile_variant {
            let mut surrounding_matrix = [
                [false, false, false],
                [false, false, false],
                [false, false, false],
            ];
            for x in grid_pos[0] - 1..=grid_pos[0] + 1 {
                for y in grid_pos[1] - 1..=grid_pos[1] + 1 {
                    let pos = [x, y];
                    if &pos == grid_pos {
                        continue;
                    }
                    if x < 0 || y < 0 {
                        surrounding_matrix[(grid_pos[1] - y + 1) as usize]
                            [(x - grid_pos[0] + 1) as usize] = true;
                        continue;
                    }
                    if let Some(Tile {
                        entity,
                        variant: TileVariant::Ground(ground_variant),
                    }) = self.tiles.get_mut(&pos)
                    {
                        surrounding_matrix[(grid_pos[1] - y + 1) as usize]
                            [(x - grid_pos[0] + 1) as usize] = true;
                        if let Some(queries) = &mut queries {
                            let (children, mut mtrx) = queries.0.get_mut(*entity).unwrap();
                            mtrx.0[(y - grid_pos[1] + 1) as usize]
                                [(grid_pos[0] - x + 1) as usize] = true;
                            let child = children[1];
                            *ground_variant = GroundVariant::from_surrounding_matrix(&mtrx.0);
                            let mut sprite = queries.1.get_mut(child).unwrap();
                            *sprite =
                                TextureAtlasSprite::new(ground_variant.get_sprite_sheet_index());
                        }
                    }
                }
            }
            Some(GroundSurroundingMatrix(surrounding_matrix))
        } else {
            None
        };

        let mut entity_commands = commands.spawn_bundle(RigidBodyBundle {
            position: world_pos.into(),
            body_type: RigidBodyTypeComponent::from(RigidBodyType::Static),
            ..Default::default()
        });
        entity_commands.with_children(|parent| {
            parent
                .spawn_bundle(ColliderBundle {
                    collider_type: ColliderTypeComponent::from(ColliderType::Sensor),
                    shape: ColliderShapeComponent::from(ColliderShape::polyline(
                        vec![
                            Point2::new(
                                -TILE_SIZE + TILE_COLLIDER_SUB - GRID_MARGIN + 0.01,
                                TILE_SIZE - TILE_COLLIDER_SUB
                                    + GROUND_MARGIN_MULTIPLIER * GRID_MARGIN
                                    + 0.02,
                            ),
                            Point2::new(
                                TILE_SIZE - TILE_COLLIDER_SUB + GRID_MARGIN - 0.01,
                                TILE_SIZE - TILE_COLLIDER_SUB
                                    + GROUND_MARGIN_MULTIPLIER * GRID_MARGIN
                                    + 0.02,
                            ),
                        ],
                        None,
                    )),
                    material: ColliderMaterialComponent::from(ColliderMaterial {
                        friction: GROUND_FRICTION,
                        ..Default::default()
                    }),
                    flags: ActiveEvents::INTERSECTION_EVENTS.into(),
                    ..Default::default()
                })
                .insert(Ground)
                .insert(ColliderPositionSync::Discrete);
            let sprite = if let Some(surrounding_matrix) = &surrounding_matrix {
                TextureAtlasSprite::new(
                    GroundVariant::from_surrounding_matrix(&surrounding_matrix.0)
                        .get_sprite_sheet_index(),
                )
            } else {
                TextureAtlasSprite::new(tile_variant.get_sprite_sheet_index())
            };
            parent
                .spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.),
                        ..Default::default()
                    },
                    texture_atlas: self.texture_atlas_handle.clone(),
                    sprite,
                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    shape: ColliderShapeComponent::from(ColliderShape::cuboid(
                        TILE_SIZE - TILE_COLLIDER_SUB + GRID_MARGIN,
                        TILE_SIZE - TILE_COLLIDER_SUB + GRID_MARGIN,
                    )),
                    material: ColliderMaterialComponent::from(ColliderMaterial {
                        friction: 0.,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .insert(ColliderPositionSync::Discrete);
        });
        if let Some(surrounding_matrix) = surrounding_matrix {
            entity_commands.insert(surrounding_matrix);
        }

        let entity = entity_commands.id();

        let tile = Tile {
            entity,
            variant: tile_variant.clone(),
        };
        self.tiles.insert(*grid_pos, tile);
    }
}
