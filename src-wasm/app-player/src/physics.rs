use crate::{Player, WalkAnimationTimer};
use app_config::{
    COLLIDER_MAX_TOI, GROUND_FRICTION_KINETIC_MULTIPLIER, GROUND_FRICTION_MIN_VEL,
    GROUND_FRICTION_STATIC_MULTIPLIER, RAPIER_GRAVITY, RAPIER_SCALE,
};
use app_core::{Ground, Course, GameMode};
use bevy::{math::Vec3Swizzles, prelude::*, utils::HashSet};
use bevy_rapier::prelude::*;

#[derive(Component, Default, Debug)]
pub struct PlayerVelocity(pub Vect);

#[derive(Debug)]
pub enum GroundIntersectEvent {
    Start(Entity),
    Stop(Entity),
}

#[derive(Component, Default)]
pub struct GroundIntersections(pub HashSet<Entity>);

#[allow(clippy::type_complexity)]
pub fn physics(
    mut query: Query<
        (
            &mut WalkAnimationTimer,
            &mut Transform,
            &mut PlayerVelocity,
            &MassProperties,
            &Children,
            &Friction,
            &mut GroundIntersections,
        ),
        (With<Player>, With<RigidBody>),
    >,
    child_query: Query<(Entity, &Collider)>,
    ground_query: Query<(&Ground, &Friction)>,
    ctx: Res<RapierContext>,
    ground_intersect_events: EventWriter<GroundIntersectEvent>,
    course: Res<Course>,
) {
    if let GameMode::Build { is_editing: true } = course.game_mode {
        return
    }
    if let Ok((
        mut timer,
        mut rb_transform,
        mut vel,
        rb_mprops,
        children,
        friction,
        mut ground_intersections,
    )) = query.get_single_mut()
    {
        let child = children.get(1).unwrap();
        let (entity, collider) = child_query.get(*child).unwrap();
        let (ground_friction, ground_colliders) = ground_collision(
            &ctx,
            &mut rb_transform,
            friction.coefficient,
            &ground_query,
            &mut ground_intersections,
            entity,
            collider,
            &mut timer,
        );

        update_ground_intersections(
            ground_colliders,
            &mut ground_intersections,
            ground_intersect_events,
            &mut timer,
        );

        set_pos_to_closest_ground_collider(&ctx, &mut rb_transform, &mut ground_intersections);

        collision_detection(
            &ctx,
            &mut rb_transform,
            &mut vel,
            &ground_query,
            entity,
            collider,
        );

        ground_friction_or_gravity(ground_friction, &mut vel, rb_mprops);
    }
}

fn collision_detection(
    ctx: &RapierContext,
    rb_transform: &mut Transform,
    vel: &mut PlayerVelocity,
    ground_query: &Query<(&Ground, &Friction)>,
    entity: Entity,
    collider: &Collider,
) {
    if rb_transform.translation.x <= 0. && vel.0.x < 0. {
        vel.0.x = 0.;
        rb_transform.translation.x = 0.;
    }

    if let Some((collider_entity, _)) = ctx.cast_shape(
        rb_transform.translation.xy(),
        rb_transform.rotation.to_axis_angle().1,
        vel.0,
        collider,
        COLLIDER_MAX_TOI,
        InteractionGroups::default(),
        Some(&|collider_entity| {
            collider_entity != entity && ground_query.get(collider_entity).is_err()
        }),
    ) {
        let mut vel_x = vel.0;
        vel_x.y = 0.;
        let mut vel_y = vel.0;
        vel_y.x = 0.;
        while ctx
            .cast_shape(
                rb_transform.translation.xy(),
                rb_transform.rotation.to_axis_angle().1,
                vel_x,
                collider,
                COLLIDER_MAX_TOI,
                InteractionGroups::default(),
                Some(&|c| c == collider_entity),
            )
            .is_some()
        {
            if let Some((_, projection)) = ctx.project_point(
                rb_transform.translation.xy(),
                true,
                InteractionGroups::default(),
                Some(&|c| c == collider_entity),
            ) {
                rb_transform.translation.x += if rb_transform.translation.x < projection.point.x {
                    -0.02
                } else {
                    0.02
                };
                vel_x.x /= 2.;
                vel.0.x = vel_x.x;
            } else {
                break;
            }
        }
        if ctx
            .cast_shape(
                rb_transform.translation.xy(),
                rb_transform.rotation.to_axis_angle().1,
                vel_y,
                collider,
                COLLIDER_MAX_TOI,
                InteractionGroups::default(),
                Some(&|c| c == collider_entity),
            )
            .is_some()
        {
            vel.0.y = 0.;
        } else {
            vel.0.x = 0.;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn ground_collision(
    ctx: &RapierContext,
    rb_transform: &mut Transform,
    friction: f32,
    ground_query: &Query<(&Ground, &Friction)>,
    ground_intersections: &mut GroundIntersections,
    entity: Entity,
    shape: &Collider,
    timer: &mut Timer,
) -> (Option<f32>, HashSet<Entity>) {
    let mut ground_friction = None;
    let mut ground_colliders = HashSet::default();
    ctx.intersections_with_shape(
        rb_transform.translation.xy(),
        rb_transform.rotation.to_axis_angle().1,
        shape,
        InteractionGroups::default(),
        Some(&|collider_entity| {
            collider_entity != entity && ground_query.get(collider_entity).is_ok()
        }),
        |collider_entity| {
            let entity = collider_entity;
            let (_, collider_friction) = ground_query.get(collider_entity).unwrap();
            ground_friction = Some(collider_friction.coefficient * friction);
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
    ctx: &RapierContext,
    rb_transform: &mut Transform,
    ground_intersections: &mut GroundIntersections,
) {
    if let Some((_, projection)) = ctx.project_point(
        rb_transform.translation.xy(),
        true,
        InteractionGroups::default(),
        Some(&|collider_entity| ground_intersections.0.contains(&collider_entity)),
    ) {
        rb_transform.translation.y = projection.point.y + 1.9 * RAPIER_SCALE;
    }
}

fn ground_friction_or_gravity(
    ground_friction: Option<f32>,
    vel: &mut PlayerVelocity,
    rb_mprops: &MassProperties,
) {
    if let Some(friction) = ground_friction {
        if vel.0.y < 0. {
            vel.0.y = 0.;
        }
        if vel.0.x.abs() > f32::EPSILON {
            vel.0.x += if vel.0.x > 0. {
                -GROUND_FRICTION_STATIC_MULTIPLIER * friction
            } else {
                GROUND_FRICTION_STATIC_MULTIPLIER * friction
            };
            vel.0.x *= 1.0 / (1.0 + GROUND_FRICTION_KINETIC_MULTIPLIER * friction);
            if vel.0.x.abs() < GROUND_FRICTION_MIN_VEL {
                vel.0.x = 0.
            }
        }
    } else {
        vel.0.y -= RAPIER_GRAVITY * rb_mprops.into_rapier(RAPIER_SCALE).inv_mass;
    }
}

pub fn apply_vel(mut query: Query<(&mut Velocity, &PlayerVelocity), With<Player>>) {
    if let Ok((mut rb_vel, vel)) = query.get_single_mut() {
        rb_vel.linvel = vel.0;
    }
}
