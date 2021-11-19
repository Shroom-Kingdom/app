use crate::{
    DashTurnEvent, FacingDirection, FallEvent, GroundIntersectEvent, JumpEvent, MovementEvent,
    Player, PlayerStateChangeEvent, PlayerStateEnum, StoopEvent, WalkEvent,
};
use bevy::prelude::*;
use bevy_rapier::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn state_change(
    mut query: Query<(&mut Player, &RigidBodyVelocity)>,
    mut fall_events: EventReader<FallEvent>,
    mut jump_events: EventReader<JumpEvent>,
    mut ground_intersect_events: EventReader<GroundIntersectEvent>,
    mut walk_events: EventReader<WalkEvent>,
    mut stoop_events: EventReader<StoopEvent>,
    mut movement_events: EventReader<MovementEvent>,
    mut dash_turn_events: EventReader<DashTurnEvent>,
    mut psc_events: EventWriter<PlayerStateChangeEvent>,
) {
    if let Ok((mut player, rb_vel)) = query.single_mut() {
        let mut state = match (
            fall_events.iter().next(),
            jump_events.iter().next(),
            ground_intersect_events.iter().next(),
            walk_events.iter().next(),
        ) {
            (Some(_), _, _, _) => Some(PlayerStateEnum::Fall),
            (_, Some(JumpEvent { high_jump_tick }), _, _) => Some(PlayerStateEnum::Jump {
                tick: 0,
                high_jump_tick: *high_jump_tick,
                impulse: false,
                released: false,
            }),
            (_, _, Some(GroundIntersectEvent::Start), _) => {
                if rb_vel.linvel.data.0[0][0] == 0. {
                    Some(PlayerStateEnum::Wait)
                } else {
                    Some(PlayerStateEnum::Walk {
                        frame: 1,
                        is_turning: false,
                    })
                }
            }
            (_, _, Some(GroundIntersectEvent::Stop), _) => Some(PlayerStateEnum::Fall),
            (_, _, _, Some(WalkEvent::Start)) => Some(PlayerStateEnum::Walk {
                frame: 0,
                is_turning: false,
            }),
            (_, _, _, Some(WalkEvent::Stop)) => Some(PlayerStateEnum::Wait),
            (_, _, _, Some(WalkEvent::Advance)) => {
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
            (None, None, None, None) => None,
        };
        if let PlayerStateEnum::Jump { .. } | PlayerStateEnum::Fall = player.state.state {
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
            (Some(StoopEvent { is_stooping: false }), PlayerStateEnum::Fall) => {
                send_state_update = true;
                player.state.is_stooping = false;
            }
            (Some(StoopEvent { is_stooping: true }), PlayerStateEnum::Fall)
            | (Some(StoopEvent { .. }), PlayerStateEnum::Jump { .. })
            | (None, _) => {}
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
