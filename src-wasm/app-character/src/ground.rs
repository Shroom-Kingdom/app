use crate::{Player, PlayerState, PlayerStateChangeEvent};
use app_ground::Grounds;
use bevy::prelude::*;
use bevy_rapier::prelude::*;

pub fn ground_intersect(
    mut query: Query<(&mut Player, Entity, &RigidBodyVelocity)>,
    grounds: Res<Grounds>,
    mut psc_event: EventWriter<PlayerStateChangeEvent>,
    mut intersection_events: EventReader<IntersectionEvent>,
) {
    if let Ok((mut player, player_entity, rb_vel)) = query.single_mut() {
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
                            player.state = PlayerState::Wait;
                            psc_event.send(PlayerStateChangeEvent {
                                state: PlayerState::Wait,
                            });
                        } else {
                            player.state = PlayerState::Walk;
                            psc_event.send(PlayerStateChangeEvent {
                                state: PlayerState::Walk,
                            });
                        };
                    }
                }
                IntersectionEvent {
                    collider1,
                    collider2,
                    intersecting: false,
                } => {
                    if !grounds.contains(&collider1.entity())
                        && !grounds.contains(&collider2.entity())
                    {
                        return;
                    }
                    if collider1.entity() == player_entity || collider2.entity() == player_entity {
                        player.state = PlayerState::Jump;
                        psc_event.send(PlayerStateChangeEvent {
                            state: PlayerState::Jump,
                        });
                    }
                }
            }
        }
    }
}
