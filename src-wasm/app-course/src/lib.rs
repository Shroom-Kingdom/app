use app_config::{GRID_SIZE, GROUND_FRICTION, TILE_COLLIDER_SUB, TILE_SIZE};
use app_core::{Course, CourseTheme};
use app_ground::{Ground, Grounds};
use app_tile::SpawnTileEvent;
use bevy::prelude::*;
use bevy_rapier::{na::Point2, prelude::*};

pub struct CoursePlugin;

impl Plugin for CoursePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_to_stage(CoreStage::Last, spawn_tile);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(Course::new(
        CourseTheme::Plain,
        &asset_server,
        &mut texture_atlases,
    ));
}

fn spawn_tile(
    mut commands: Commands,
    mut course: ResMut<Course>,
    mut grounds: ResMut<Grounds>,
    mut spawn_tile_events: EventReader<SpawnTileEvent>,
) {
    for SpawnTileEvent { grid_pos, tile } in spawn_tile_events.iter() {
        let world_pos = grid_to_world(grid_pos);
        if course.tiles.contains_key(grid_pos) {
            return;
        }
        course.tiles.insert(*grid_pos, tile.clone());

        commands
            .spawn_bundle(RigidBodyBundle {
                position: world_pos.into(),
                body_type: RigidBodyType::Static,
                ..Default::default()
            })
            .with_children(|parent| {
                let ground = parent
                    .spawn_bundle(ColliderBundle {
                        collider_type: ColliderType::Sensor,
                        shape: ColliderShape::polyline(
                            vec![
                                Point2::new(
                                    -TILE_SIZE + TILE_COLLIDER_SUB + 0.01,
                                    TILE_SIZE - TILE_COLLIDER_SUB,
                                ),
                                Point2::new(
                                    TILE_SIZE - TILE_COLLIDER_SUB - 0.01,
                                    TILE_SIZE - TILE_COLLIDER_SUB,
                                ),
                            ],
                            None,
                        ),
                        flags: ActiveEvents::INTERSECTION_EVENTS.into(),
                        ..Default::default()
                    })
                    .insert(Ground)
                    .insert(ColliderPositionSync::Discrete)
                    .id();
                // TODO on entity despawn?
                grounds.insert(ground);
                parent.spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.),
                        ..Default::default()
                    },
                    texture_atlas: course.texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(6),
                    ..Default::default()
                });
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(
                    TILE_SIZE - TILE_COLLIDER_SUB,
                    TILE_SIZE - TILE_COLLIDER_SUB,
                ),
                material: ColliderMaterial {
                    friction: GROUND_FRICTION,
                    friction_combine_rule: CoefficientCombineRule::Multiply,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(ColliderPositionSync::Discrete);
    }
}

fn grid_to_world(grid_pos: &[i32; 2]) -> Vec2 {
    [
        grid_pos[0] as f32 * GRID_SIZE,
        grid_pos[1] as f32 * GRID_SIZE,
    ]
    .into()
}
