use app_config::{
    GRID_MARGIN, GROUND_FRICTION, GROUND_MARGIN_MULTIPLIER, TILE_COLLIDER_SUB, TILE_SIZE,
};
use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use bevy_rapier::{na::Point2, prelude::*};

use crate::{grid_to_world, Ground};

#[derive(Debug, TypeUuid)]
#[uuid = "81a23571-1f35-4f20-b1ea-30e5c2612049"]
pub struct Course {
    pub texture_atlas_handle: Handle<TextureAtlas>,
    pub tiles: HashMap<[i32; 2], Tile>,
    pub theme: CourseTheme,
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

        for x in 0..8 {
            for y in 0..2 {
                course.spawn_tile(commands, &[x, y], &TileVariant::Block);
            }
        }

        course
    }

    pub fn spawn_tile(
        &mut self,
        commands: &mut Commands,
        grid_pos: &[i32; 2],
        tile_variant: &TileVariant,
    ) {
        let world_pos = grid_to_world(grid_pos);
        if self.tiles.contains_key(grid_pos) {
            return;
        }

        // TODO min max course pos
        if grid_pos[0] < 0 || grid_pos[1] < 0 || grid_pos[0] > 150 || grid_pos[1] > 24 {
            return;
        }

        let entity = commands
            .spawn_bundle(RigidBodyBundle {
                position: world_pos.into(),
                body_type: RigidBodyTypeComponent::from(RigidBodyType::Static),
                ..Default::default()
            })
            .with_children(|parent| {
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
                parent
                    .spawn_bundle(SpriteSheetBundle {
                        transform: Transform {
                            scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.),
                            ..Default::default()
                        },
                        texture_atlas: self.texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(6),
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
            })
            .id();

        let tile = Tile {
            entity,
            variant: tile_variant.clone(),
        };
        self.tiles.insert(*grid_pos, tile);
    }
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

#[derive(Clone, Debug)]
pub struct Tile {
    pub entity: Entity,
    pub variant: TileVariant,
}

#[derive(Clone, Debug)]
pub enum TileVariant {
    Block,
}
