use crate::{Player, PlayerStateEnum};
use app_ground::Grounds;
use bevy::{prelude::*, utils::HashSet};
use bevy_rapier::prelude::*;

#[derive(Debug)]
pub enum GroundIntersectEvent {
    Start(Entity),
    Stop(Entity),
}

#[derive(Default)]
pub struct GroundIntersections(pub HashSet<Entity>);

pub fn ground_intersect(
    mut query: Query<(&Player, Entity, &mut Timer, &mut RigidBodyVelocity)>,
    grounds: Res<Grounds>,
    mut ground_intersect_events: EventWriter<GroundIntersectEvent>,
    mut intersection_events: EventReader<IntersectionEvent>,
) {
    if let Ok((player, player_entity, mut timer, mut rb_vel)) = query.single_mut() {
        for intersection_event in intersection_events.iter() {
            match intersection_event {
                IntersectionEvent {
                    collider1,
                    collider2,
                    intersecting: true,
                    ..
                } => {
                    if !grounds.contains(&collider1.entity())
                        && !grounds.contains(&collider2.entity())
                    {
                        return;
                    }
                    if collider1.entity() == player_entity || collider2.entity() == player_entity {
                        let collider_entity = if collider1.entity() != player_entity {
                            collider1.entity()
                        } else {
                            collider2.entity()
                        };
                        timer.reset();
                        if rb_vel.linvel.data.0[0][1] < 0. {
                            rb_vel.linvel.data.0[0][1] = 0.;
                        }
                        ground_intersect_events.send(GroundIntersectEvent::Start(collider_entity));
                    }
                }
                IntersectionEvent {
                    collider1,
                    collider2,
                    intersecting: false,
                } => {
                    if let PlayerStateEnum::Air { .. } = player.state.state {
                        return;
                    }
                    if !grounds.contains(&collider1.entity())
                        && !grounds.contains(&collider2.entity())
                    {
                        return;
                    }
                    if collider1.entity() == player_entity || collider2.entity() == player_entity {
                        let collider_entity = if collider1.entity() != player_entity {
                            collider1.entity()
                        } else {
                            collider2.entity()
                        };
                        ground_intersect_events.send(GroundIntersectEvent::Stop(collider_entity));
                    }
                }
            }
        }
    }
}
