use crate::{Player, PlayerState};
use app_config::{HIGH_JUMP_TICK, JUMP_FORCE};
use bevy::prelude::*;
use bevy_rapier::prelude::*;

pub struct JumpEvent(f32);

pub fn jump(
    mut query: Query<(&Player, &mut RigidBodyVelocity)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut psc_event: EventWriter<JumpEvent>,
) {
    for (player, mut rb_vel) in query.iter_mut() {
        let jump = keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::W);
        if !jump {
            return;
        }
        match player.state {
            PlayerState::Wait | PlayerState::Walk { .. } => {
                rb_vel.linvel.data.0[0][1] = JUMP_FORCE;
                psc_event.send(JumpEvent(rb_vel.linvel.data.0[0][0]))
            }
            _ => {}
        }
    }
}

pub fn high_jump(
    mut query: Query<(&mut Player, &mut RigidBodyVelocity)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut player, mut rb_vel) in query.iter_mut() {
        match player.state {
            PlayerState::Jump {
                tick,
                released: false,
                impulse,
                ..
            } if tick < HIGH_JUMP_TICK => {
                let released = keyboard_input.just_released(KeyCode::Space)
                    || keyboard_input.just_released(KeyCode::Up)
                    || keyboard_input.just_released(KeyCode::W);
                let jump = keyboard_input.pressed(KeyCode::Space)
                    || keyboard_input.pressed(KeyCode::Up)
                    || keyboard_input.pressed(KeyCode::W);

                rb_vel.linvel.data.0[0][1] = JUMP_FORCE;
                if released {
                    player.state = PlayerState::Jump {
                        tick: 0,
                        released,
                        impulse,
                    };
                } else if jump {
                    player.state = PlayerState::Jump {
                        tick: tick + 1,
                        released: false,
                        impulse,
                    };
                }
            }
            PlayerState::Jump {
                tick,
                released: false,
                impulse,
            } if tick < HIGH_JUMP_TICK => {
                rb_vel.linvel.data.0[0][1] = JUMP_FORCE;
                player.state = PlayerState::Jump {
                    tick: 0,
                    released: true,
                    impulse,
                };
            }
            _ => {}
        }
    }
}
