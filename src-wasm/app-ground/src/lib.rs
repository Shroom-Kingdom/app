use app_config::GROUND_FRICTION;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    utils::{AHashExt, HashSet},
};
use bevy_rapier::{na::Point2, prelude::*};

#[derive(Debug, TypeUuid)]
#[uuid = "dd6e046a-e4ed-4c39-a796-ae9477c6d912"]
pub struct Ground;

#[derive(Debug, TypeUuid)]
#[uuid = "0665b49a-c081-454b-bd56-8943726640b0"]
pub struct Grounds(HashSet<Entity>);

impl Grounds {
    pub fn contains(&self, value: &Entity) -> bool {
        self.0.contains(value)
    }
}

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ground);
    }
}

pub fn setup_ground(mut commands: Commands) {
    let ground_size = 50.0;
    let ground_height = 1.0;

    let mut grounds = HashSet::with_capacity(100);

    commands
        .spawn_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .with_children(|parent| {
            let ground = parent
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
                .insert(ColliderPositionSync::Discrete)
                .id();
            // TODO on entity despawn?
            grounds.insert(ground);
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

    commands.insert_resource(Grounds(grounds));
}
