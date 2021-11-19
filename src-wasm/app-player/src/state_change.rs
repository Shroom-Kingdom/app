use crate::{
    DashTurnEvent, FacingDirection, GroundIntersectEvent, GroundIntersections, JumpEvent,
    MovementEvent, Player, PlayerStateChangeEvent, PlayerStateEnum, StoopEvent, WalkEvent,
};
use bevy::prelude::*;
use bevy_rapier::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn state_change(
    mut query: Query<(&mut Player, &mut GroundIntersections, &RigidBodyVelocity)>,
    mut jump_events: EventReader<JumpEvent>,
    ground_intersect_events: EventReader<GroundIntersectEvent>,
    walk_events: EventReader<WalkEvent>,
    mut stoop_events: EventReader<StoopEvent>,
    mut movement_events: EventReader<MovementEvent>,
    mut dash_turn_events: EventReader<DashTurnEvent>,
    mut psc_events: EventWriter<PlayerStateChangeEvent>,
) {
    if let Ok((mut player, mut ground_intersections, rb_vel)) = query.single_mut() {
        let mut state = handle_walk_events(&mut player, walk_events);

        state = handle_ground_intersect_events(
            state,
            rb_vel,
            ground_intersect_events,
            &mut ground_intersections,
        );

        state = if let Some(jump_event) = jump_events.iter().next() {
            let JumpEvent { high_jump_tick } = jump_event;
            Some(PlayerStateEnum::Jump {
                tick: 0,
                high_jump_tick: *high_jump_tick,
                impulse: false,
                released: false,
            })
        } else {
            state
        };

        if let PlayerStateEnum::Jump { .. } = player.state.state {
            if let Some(is_touching_ground) = player.state.is_touching_ground {
                let is_touching_ground = is_touching_ground + 1;
                if is_touching_ground > 5 {
                    state = if rb_vel.linvel.data.0[0][0] == 0. {
                        Some(PlayerStateEnum::Wait)
                    } else {
                        Some(PlayerStateEnum::Walk {
                            frame: 1,
                            is_turning: false,
                        })
                    };
                } else {
                    player.state.is_touching_ground = Some(is_touching_ground);
                }
            }
        }

        let mut send_state_update = false;
        match (stoop_events.iter().next(), &player.state.state) {
            (
                Some(StoopEvent { is_stooping }),
                PlayerStateEnum::Wait | PlayerStateEnum::Walk { .. },
            ) => {
                send_state_update = true;
                player.state.is_stooping = *is_stooping;
            }
            (Some(StoopEvent { is_stooping: false }), PlayerStateEnum::Jump { .. })
                if rb_vel.linvel.data.0[0][1] <= 0. =>
            {
                send_state_update = true;
                player.state.is_stooping = false;
            }
            (Some(StoopEvent { .. }), PlayerStateEnum::Jump { .. }) | (None, _) => {}
        }
        if let Some(DashTurnEvent { is_dash_turning }) = dash_turn_events.iter().next() {
            send_state_update = true;
            player.state.is_dash_turning = *is_dash_turning;
        }
        match movement_events.iter().next() {
            Some(MovementEvent::Left) => {
                send_state_update = true;
                player.state.facing_direction = FacingDirection::Left;
            }
            Some(MovementEvent::Right) => {
                send_state_update = true;
                player.state.facing_direction = FacingDirection::Right;
            }
            None => {}
        }
        if let Some(state) = state {
            send_state_update = true;
            player.state.state = state
        }

        if send_state_update {
            psc_events.send(PlayerStateChangeEvent {
                state: player.state.clone(),
            });
        }
    }
}

fn handle_walk_events(
    player: &mut Player,
    mut walk_events: EventReader<WalkEvent>,
) -> Option<PlayerStateEnum> {
    if let Some(walk_event) = walk_events.iter().next() {
        match walk_event {
            WalkEvent::Start => Some(PlayerStateEnum::Walk {
                frame: 0,
                is_turning: false,
            }),
            WalkEvent::Stop => Some(PlayerStateEnum::Wait),
            WalkEvent::Advance => {
                if let PlayerStateEnum::Walk {
                    frame,
                    is_turning: false,
                } = player.state.state
                {
                    let frame = match frame {
                        0 => 1,
                        _ => 0,
                    };
                    Some(PlayerStateEnum::Walk {
                        frame,
                        is_turning: false,
                    })
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}

fn handle_ground_intersect_events(
    prev_state: Option<PlayerStateEnum>,
    rb_vel: &RigidBodyVelocity,
    mut ground_intersect_events: EventReader<GroundIntersectEvent>,
    ground_intersections: &mut GroundIntersections,
) -> Option<PlayerStateEnum> {
    for ground_intersect_event in ground_intersect_events.iter() {
        match ground_intersect_event {
            GroundIntersectEvent::Start(entity) => {
                ground_intersections.0.insert(*entity);
            }
            GroundIntersectEvent::Stop(entity) => {
                ground_intersections.0.remove(entity);
            }
        }
    }
    if ground_intersect_events.iter().next().is_some() {
        if !ground_intersections.0.is_empty() {
            if rb_vel.linvel.data.0[0][0] == 0. {
                Some(PlayerStateEnum::Wait)
            } else {
                Some(PlayerStateEnum::Walk {
                    frame: 1,
                    is_turning: false,
                })
            }
        } else {
            Some(PlayerStateEnum::Jump {
                tick: 0,
                high_jump_tick: 0,
                impulse: true,
                released: true,
            })
        }
    } else {
        prev_state
    }
}
