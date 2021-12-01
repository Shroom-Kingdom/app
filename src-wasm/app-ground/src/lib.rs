use app_config::GROUND_FRICTION;
use bevy::prelude::*;
use bevy_rapier::{na::Point2, prelude::*};

#[derive(Debug)]
pub struct Ground;

#[derive(Debug)]
pub struct GroundProximity;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ground);
    }
}

pub fn setup_ground(mut commands: Commands) {
    let ground_size = 50.0;
    let ground_height = 1.0;

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ColliderBundle {
                    collider_type: ColliderType::Sensor,
                    shape: ColliderShape::polyline(
                        vec![
                            Point2::new(-ground_size, ground_height),
                            Point2::new(ground_size, ground_height),
                        ],
                        None,
                    ),
                    flags: ActiveEvents::INTERSECTION_EVENTS.into(),
                    ..Default::default()
                })
                .insert(Ground)
                .insert(ColliderPositionSync::Discrete);
            parent
                .spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(ground_size, 1.0),
                    material: ColliderMaterial {
                        friction: GROUND_FRICTION,
                        friction_combine_rule: CoefficientCombineRule::Multiply,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ColliderDebugRender::default())
                .insert(ColliderPositionSync::Discrete);
        });
}
