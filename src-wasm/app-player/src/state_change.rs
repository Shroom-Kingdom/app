use crate::{
    DashTurnEvent, FacingDirection, FacingDirectionEvent, GroundIntersectEvent,
    GroundIntersections, JumpEvent, Player, PlayerStateChangeEvent, PlayerStateEnum,
    PlayerVelocity, StoopEvent, TouchEvent, WalkEvent,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn state_change(
    mut query: Query<(&mut Player, &mut GroundIntersections, &PlayerVelocity)>,
    walk_events: EventReader<WalkEvent>,
    mut touch_events: EventReader<TouchEvent>,
    ground_intersect_events: EventReader<GroundIntersectEvent>,
    mut jump_events: EventReader<JumpEvent>,
    mut stoop_events: EventReader<StoopEvent>,
    mut facing_direction_events: EventReader<FacingDirectionEvent>,
    mut dash_turn_events: EventReader<DashTurnEvent>,
    mut psc_events: EventWriter<PlayerStateChangeEvent>,
) {
    if let Ok((mut player, mut ground_intersections, vel)) = query.get_single_mut() {
        let mut state = handle_walk_events(&mut player, walk_events);

        state = if touch_events.iter().next().is_some() {
            if let PlayerStateEnum::Air { fall, .. } = player.state.state {
                Some(PlayerStateEnum::Air {
                    tick: 0,
                    high_jump_tick: 0,
                    impulse: false,
                    released: true,
                    fall,
                })
            } else {
                state
            }
        } else {
            state
        };

        state = handle_ground_intersect_events(
            &player,
            state,
            vel,
            ground_intersect_events,
            &mut ground_intersections,
        );

        state = if let Some(jump_event) = jump_events.iter().next() {
            let JumpEvent {
                high_jump_tick,
                fall,
            } = jump_event;
            Some(PlayerStateEnum::Air {
                tick: 0,
                high_jump_tick: *high_jump_tick,
                impulse: false,
                released: false,
                fall: *fall,
            })
        } else {
            state
        };

        let mut send_state_update = false;
        match (stoop_events.iter().next(), &player.state.state) {
            (Some(StoopEvent { is_stooping }), PlayerStateEnum::Ground { .. }) => {
                send_state_update = true;
                player.state.is_stooping = *is_stooping;
            }
            (Some(StoopEvent { is_stooping: false }), PlayerStateEnum::Air { .. })
                if vel.0.y <= 0. =>
            {
                send_state_update = true;
                player.state.is_stooping = false;
            }
            (Some(StoopEvent { .. }), PlayerStateEnum::Air { .. }) | (None, _) => {}
        }
        if let Some(DashTurnEvent { is_dash_turning }) = dash_turn_events.iter().next() {
            send_state_update = true;
            player.state.is_dash_turning = *is_dash_turning;
        }
        match facing_direction_events.iter().next() {
            Some(FacingDirectionEvent::Left) => {
                send_state_update = true;
                player.state.facing_direction = FacingDirection::Left;
            }
            Some(FacingDirectionEvent::Right) => {
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
            WalkEvent::Start => Some(PlayerStateEnum::Ground {
                frame: 0,
                is_walking: true,
                is_turning: false,
            }),
            WalkEvent::Stop => Some(PlayerStateEnum::Ground {
                frame: 0,
                is_walking: false,
                is_turning: false,
            }),
            WalkEvent::Advance => {
                if let PlayerStateEnum::Ground {
                    frame,
                    is_walking,
                    is_turning: false,
                } = player.state.state
                {
                    let frame = match frame {
                        0 => 1,
                        _ => 0,
                    };
                    Some(PlayerStateEnum::Ground {
                        frame,
                        is_walking,
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
    player: &Player,
    prev_state: Option<PlayerStateEnum>,
    vel: &PlayerVelocity,
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
    if vel.0.y <= 0. {
        if !ground_intersections.0.is_empty() {
            match player.state.state {
                PlayerStateEnum::Ground { .. } => prev_state,
                _ => Some(PlayerStateEnum::Ground {
                    frame: 1,
                    is_walking: vel.0.x.abs() >= f32::EPSILON,
                    is_turning: false,
                }),
            }
        } else if let PlayerStateEnum::Air { .. } = player.state.state {
            prev_state
        } else {
            Some(PlayerStateEnum::Air {
                tick: 0,
                high_jump_tick: 0,
                impulse: true,
                released: true,
                fall: true,
            })
        }
    } else {
        prev_state
    }
}
