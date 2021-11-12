use crate::{
    FallEvent, GroundIntersectEvent, JumpEvent, Player, PlayerStateChangeEvent, PlayerStateEnum,
    StoopEvent, WalkEvent,
};
use bevy::prelude::*;
use bevy_rapier::prelude::*;

pub fn state_change(
    mut query: Query<(&mut Player, &RigidBodyVelocity)>,
    mut fall_events: EventReader<FallEvent>,
    mut jump_events: EventReader<JumpEvent>,
    mut ground_intersect_events: EventReader<GroundIntersectEvent>,
    mut walk_events: EventReader<WalkEvent>,
    mut stoop_events: EventReader<StoopEvent>,
    mut psc_events: EventWriter<PlayerStateChangeEvent>,
) {
    if let Ok((mut player, rb_vel)) = query.single_mut() {
        let state = match (
            fall_events.iter().next(),
            jump_events.iter().next(),
            ground_intersect_events.iter().next(),
            walk_events.iter().next(),
        ) {
            (Some(_), _, _, _) => Some(PlayerStateEnum::Fall),
            (_, Some(_), _, _) => Some(PlayerStateEnum::Jump {
                tick: 0,
                impulse: false,
                released: false,
            }),
            (_, _, Some(GroundIntersectEvent::Start), _) => {
                if rb_vel
                    .linvel
                    .data
                    .0
                    .get(0)
                    .map(|x| x[0] == 0.)
                    .unwrap_or_default()
                {
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
