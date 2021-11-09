use crate::{Player, PlayerState, PlayerStateChangeEvent};
use app_ground::Grounds;
use bevy::prelude::*;
use bevy_rapier::prelude::*;

pub fn ground_intersect(
    query: Query<(&Player, Entity, &RigidBodyVelocity)>,
    grounds: Res<Grounds>,
    mut psc_event: EventWriter<PlayerStateChangeEvent>,
    mut intersection_events: EventReader<IntersectionEvent>,
) {
    if let Ok((player, player_entity, rb_vel)) = query.single() {
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
                        if rb_vel
                            .linvel
                            .data
                            .0
                            .get(0)
                            .map(|x| x[0] == 0.)
                            .unwrap_or_default()
                        {
                            psc_event.send(PlayerStateChangeEvent {
                                state: PlayerState::Wait,
                            });
                        } else {
                            psc_event.send(PlayerStateChangeEvent {
                                state: PlayerState::Walk(0),
                            });
                        };
                    }
                }
                IntersectionEvent {
                    collider1,
                    collider2,
                    intersecting: false,
                } => {
                    if let PlayerState::Jump { .. } = player.state {
                        return;
                    }
                    if !grounds.contains(&collider1.entity())
                        && !grounds.contains(&collider2.entity())
                    {
                        return;
                    }
                    if collider1.entity() == player_entity || collider2.entity() == player_entity {
                        psc_event.send(PlayerStateChangeEvent {
                            // TODO PlayerState::Fall
                            state: PlayerState::Jump {
                                tick: 0,
                                linvel_x: 0.,
                            },
                        });
                    }
                }
            }
        }
    }
}
