use crate::Player;
use app_config::{
    COLLIDER_MIN_TOI, GROUND_FRICTION_KINETIC_MULTIPLIER, GROUND_FRICTION_MIN_VEL,
    GROUND_FRICTION_STATIC_MULTIPLIER, RAPIER_GRAVITY_VECTOR, RAPIER_SCALE,
};
use app_core::Ground;
use bevy::{prelude::*, utils::HashSet};
use bevy_rapier::prelude::*;

#[derive(Component, Default, Debug)]
pub struct PlayerVelocity(pub Vector<Real>);

#[derive(Debug)]
pub enum GroundIntersectEvent {
    Start(Entity),
    Stop(Entity),
}

#[derive(Component, Default)]
pub struct GroundIntersections(pub HashSet<Entity>);

pub fn physics(
    mut query: Query<
        (
            Entity,
            &mut Timer,
            &mut RigidBodyPositionComponent,
            &mut PlayerVelocity,
            &RigidBodyMassPropsComponent,
            &ColliderShapeComponent,
            &ColliderMaterialComponent,
            &mut GroundIntersections,
        ),
        With<Player>,
    >,
    ground_query: Query<(&Ground, &ColliderMaterialComponent)>,
    query_pipeline: Res<QueryPipeline>,
    colliders: QueryPipelineColliderComponentsQuery,
    ground_intersect_events: EventWriter<GroundIntersectEvent>,
) {
    if let Ok((
        entity,
        mut timer,
        mut rb_pos,
        mut vel,
        rb_mprops,
        shape,
        c_mat,
        mut ground_intersections,
    )) = query.get_single_mut()
    {
        let colliders = QueryPipelineColliderComponentsSet(&colliders);

        let (ground_friction, ground_colliders) = ground_collision(
            &query_pipeline,
            &colliders,
            &mut rb_pos,
            c_mat.friction,
            &ground_query,
            &mut ground_intersections,
            entity,
            &*shape.0,
            &mut timer,
        );

        update_ground_intersections(
            ground_colliders,
            &mut ground_intersections,
            ground_intersect_events,
            &mut timer,
        );

        set_pos_to_closest_ground_collider(
            &query_pipeline,
            &colliders,
            &mut rb_pos,
            &mut ground_intersections,
        );

        collision_detection(
            &query_pipeline,
            &colliders,
            &mut rb_pos,
            &mut vel,
            &ground_query,
            entity,
            &*shape.0,
        );

        ground_friction_or_gravity(ground_friction, &mut vel, rb_mprops);
    }
}

fn collision_detection(
    query_pipeline: &QueryPipeline,
    colliders: &QueryPipelineColliderComponentsSet,
    rb_pos: &mut RigidBodyPosition,
    vel: &mut PlayerVelocity,
    ground_query: &Query<(&Ground, &ColliderMaterialComponent)>,
    entity: Entity,
    shape: &dyn Shape,
) {
    if rb_pos.next_position.translation.vector.data.0[0][0] <= 0. && vel.0[0] < 0. {
        vel.0[0] = 0.;
        rb_pos.position.translation.vector.data.0[0][0] = 0.;
    }

    if let Some((collider, _)) = query_pipeline.cast_shape(
        colliders,
        &rb_pos.position,
        &vel.0,
        shape,
        COLLIDER_MIN_TOI,
        InteractionGroups::default(),
        Some(&|collider| {
            collider.entity() != entity && ground_query.get(collider.entity()).is_err()
        }),
    ) {
        let mut vel_x = vel.0;
        vel_x.data.0[0][1] = 0.;
        let mut vel_y = vel.0;
        vel_y.data.0[0][0] = 0.;
        if query_pipeline
            .cast_shape(
                colliders,
                &rb_pos.position,
                &vel_x,
                shape,
                COLLIDER_MIN_TOI,
                InteractionGroups::default(),
                Some(&|c| c.entity() == collider.entity()),
            )
            .is_some()
        {
            rb_pos.position.translation.vector.data.0[0][0] +=
                if vel_x.data.0[0][0] > 0. { -0.02 } else { 0.02 };
            vel.0[0] = 0.;
        }
        if query_pipeline
            .cast_shape(
                colliders,
                &rb_pos.position,
                &vel_y,
                shape,
                COLLIDER_MIN_TOI,
                InteractionGroups::default(),
                Some(&|c| c.entity() == collider.entity()),
            )
            .is_some()
        {
            vel.0[1] = 0.;
        } else {
            vel.0[0] = 0.;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn ground_collision(
    query_pipeline: &QueryPipeline,
    colliders: &QueryPipelineColliderComponentsSet,
    rb_pos: &mut RigidBodyPosition,
    friction: f32,
    ground_query: &Query<(&Ground, &ColliderMaterialComponent)>,
    ground_intersections: &mut GroundIntersections,
    entity: Entity,
    shape: &dyn Shape,
    timer: &mut Timer,
) -> (Option<f32>, HashSet<Entity>) {
    let mut ground_friction = None;
    let mut ground_colliders = HashSet::default();
    query_pipeline.intersections_with_shape(
        colliders,
        &rb_pos.position,
        shape,
        InteractionGroups::default(),
        Some(&|collider: ColliderHandle| {
            collider.entity() != entity && ground_query.get(collider.entity()).is_ok()
        }),
        |collider| {
            let entity = collider.entity();
            let (_, material) = ground_query.get(collider.entity()).unwrap();
            ground_friction = Some(material.friction * friction);
            ground_colliders.insert(entity);
            if !ground_intersections.0.contains(&entity) {
                timer.reset();
                ground_intersections.0.insert(entity);
            }
            true
        },
    );
    (ground_friction, ground_colliders)
}

fn update_ground_intersections(
    ground_colliders: HashSet<Entity>,
    ground_intersections: &mut GroundIntersections,
    mut ground_intersect_events: EventWriter<GroundIntersectEvent>,
    timer: &mut Timer,
) {
    #[allow(clippy::needless_collect)]
    let start_intersect_colliders: Vec<_> = ground_colliders
        .difference(&ground_intersections.0)
        .cloned()
        .collect();
    #[allow(clippy::needless_collect)]
    let stop_intersect_colliders: Vec<_> = ground_intersections
        .0
        .difference(&ground_colliders)
        .cloned()
        .collect();
    for collider in start_intersect_colliders.into_iter() {
        timer.reset();
        ground_intersect_events.send(GroundIntersectEvent::Start(collider));
        ground_intersections.0.insert(collider);
    }
    for collider in stop_intersect_colliders.into_iter() {
        ground_intersect_events.send(GroundIntersectEvent::Stop(collider));
        ground_intersections.0.remove(&collider);
    }
}

fn set_pos_to_closest_ground_collider(
    query_pipeline: &QueryPipeline,
    colliders: &QueryPipelineColliderComponentsSet,
    rb_pos: &mut RigidBodyPosition,
    ground_intersections: &mut GroundIntersections,
) {
    if let Some((_, projection)) = query_pipeline.project_point(
        colliders,
        &Point::from(rb_pos.position.translation.vector),
        true,
        InteractionGroups::default(),
        Some(&|collider| ground_intersections.0.contains(&collider.entity())),
    ) {
        rb_pos.position.translation.vector.data.0[0][1] =
            projection.point.coords.data.0[0][1] + 1.9;
    }
}

fn ground_friction_or_gravity(
    ground_friction: Option<f32>,
    vel: &mut PlayerVelocity,
    rb_mprops: &RigidBodyMassProps,
) {
    if let Some(friction) = ground_friction {
        if vel.0[1] < 0. {
            vel.0[1] = 0.;
        }
        if vel.0[0].abs() > f32::EPSILON {
            vel.0[0] += if vel.0[0] > 0. {
                -GROUND_FRICTION_STATIC_MULTIPLIER * friction
            } else {
                GROUND_FRICTION_STATIC_MULTIPLIER * friction
            };
            vel.0[0] *= 1.0 / (1.0 + GROUND_FRICTION_KINETIC_MULTIPLIER * friction);
            if vel.0[0].abs() < GROUND_FRICTION_MIN_VEL {
                vel.0[0] = 0.
            }
        }
    } else {
        vel.0 += RAPIER_GRAVITY_VECTOR.component_mul(&rb_mprops.effective_inv_mass) * RAPIER_SCALE;
    }
}

pub fn apply_vel(
    mut query: Query<(&mut RigidBodyVelocityComponent, &PlayerVelocity), With<Player>>,
) {
    if let Ok((mut rb_vel, vel)) = query.get_single_mut() {
        rb_vel.linvel = vel.0;
    }
}
