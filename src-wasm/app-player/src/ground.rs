use crate::{Player, PlayerStateEnum};
use app_ground::Grounds;
use bevy::{prelude::*, utils::HashSet};
use bevy_rapier::prelude::*;

pub enum GroundIntersectEvent {
    Start(Entity),
    Stop(Entity),
}

#[derive(Default)]
pub struct GroundIntersections(pub HashSet<Entity>);

pub fn ground_intersect(
    mut query: Query<(&mut Player, Entity, &mut Timer, &mut RigidBodyVelocity)>,
    grounds: Res<Grounds>,
    mut ground_intersect_events: EventWriter<GroundIntersectEvent>,
    mut intersection_events: EventReader<IntersectionEvent>,
) {
    if let Ok((mut player, player_entity, mut timer, mut rb_vel)) = query.single_mut() {
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
                        rb_vel.linvel.data.0[0][1] = 0.;
                        ground_intersect_events.send(GroundIntersectEvent::Start(collider_entity));
                        player.state.is_touching_ground = Some(0);
                    }
                }
                IntersectionEvent {
                    collider1,
                    collider2,
                    intersecting: false,
                } => {
                    if let PlayerStateEnum::Jump { .. } = player.state.state {
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
                        player.state.is_touching_ground = None;
                    }
                }
            }
        }
    }
}
