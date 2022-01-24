use crate::{Player, PlayerStateEnum, PlayerVelocity};
use app_config::{HIGH_JUMP_TICK, HIGH_JUMP_TICK_WALK, HIGH_JUMP_WALK_THRESHOLD, JUMP_FORCE};
use bevy::prelude::*;

pub struct JumpEvent {
    pub high_jump_tick: u8,
    pub fall: bool,
}

pub fn jump(
    mut query: Query<(&Player, &mut PlayerVelocity)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut psc_event: EventWriter<JumpEvent>,
) {
    if let Ok((player, mut vel)) = query.get_single_mut() {
        let jump = keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::W);
        if !jump {
            return;
        }
        if let PlayerStateEnum::Ground { .. } = player.state.state {
            vel.0[1] = JUMP_FORCE;
            let high_jump_tick = if vel.0[0].abs() > HIGH_JUMP_WALK_THRESHOLD {
                HIGH_JUMP_TICK_WALK
            } else {
                HIGH_JUMP_TICK
            };
            psc_event.send(JumpEvent {
                high_jump_tick,
                fall: false,
            })
        }
    }
}

pub fn high_jump(
    mut query: Query<(&mut Player, &mut PlayerVelocity)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut player, mut vel)) = query.get_single_mut() {
        match player.state.state {
            PlayerStateEnum::Air {
                tick,
                high_jump_tick,
                released: false,
                impulse,
                fall,
            } if tick < high_jump_tick => {
                let jump = keyboard_input.pressed(KeyCode::Space)
                    || keyboard_input.pressed(KeyCode::Up)
                    || keyboard_input.pressed(KeyCode::W);

                vel.0[1] = JUMP_FORCE;
                player.state.state = PlayerStateEnum::Air {
                    tick: tick + 1,
                    high_jump_tick,
                    released: !jump,
                    impulse,
                    fall,
                };
            }
            _ => {}
        }
    }
}

pub fn jump_to_fall(
    query: Query<(&Player, &PlayerVelocity)>,
    mut jump_events: EventWriter<JumpEvent>,
) {
    if let Ok((player, vel)) = query.get_single() {
        if let PlayerStateEnum::Air { .. } = player.state.state {
            if vel.0[1] < 0. {
                jump_events.send(JumpEvent {
                    high_jump_tick: 0,
                    fall: true,
                });
            }
        }
    }
}
