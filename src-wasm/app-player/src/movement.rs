use crate::{Player, PlayerState, PlayerStateEnum};
use app_config::{
    LINVEL_CAP_RUN, LINVEL_CAP_STOOP, LINVEL_CAP_WALK, MOVE_IMPULSE_MULTIPLIER_AIR,
    MOVE_IMPULSE_MULTIPLIER_AIR_RUN, MOVE_IMPULSE_MULTIPLIER_GROUND,
    MOVE_IMPULSE_MULTIPLIER_GROUND_RUN, RUN_THRESHOLD,
};
use bevy::prelude::*;
use bevy_rapier::{na::Vector2, prelude::*};

pub enum MovementEvent {
    Left,
    Right,
}

pub struct DashTurnEvent {
    pub is_dash_turning: bool,
}

pub fn run(mut query: Query<&mut Player>, keyboard_input: Res<Input<KeyCode>>) {
    if let Ok(mut player) = query.single_mut() {
        let run =
            keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);
        player.state.is_running = run;
    }
}

pub fn movement(
    mut query: Query<(
        &mut Player,
        &mut RigidBodyVelocity,
        &RigidBodyMassProps,
        &mut ColliderMaterial,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
    mut movement_events: EventWriter<MovementEvent>,
    mut dash_turn_events: EventWriter<DashTurnEvent>,
) {
    if let Ok((mut player, mut rb_vel, rb_mprops, mut c_mat)) = query.single_mut() {
        match player.state {
            PlayerState {
                is_stooping: false,
                is_running,
                state: PlayerStateEnum::Air { .. } | PlayerStateEnum::Ground { .. },
                ..
            }
            | PlayerState {
                is_stooping: true,
                is_running,
                state: PlayerStateEnum::Air { .. },
                ..
            } => {
                if let PlayerState {
                    state: PlayerStateEnum::Ground { .. },
                    ..
                } = player.state
                {
                    player.state.is_dashing = rb_vel.linvel.data.0[0][0].abs() > RUN_THRESHOLD;
                }

                let left =
                    keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
                let right =
                    keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

                let x_axis = -(left as i8) + right as i8;
                let cap = match (player.state.is_stooping, is_running) {
                    (true, _) => LINVEL_CAP_STOOP,
                    (false, true) => LINVEL_CAP_RUN,
                    (false, false) => LINVEL_CAP_WALK,
                };
                match (&player.state, rb_vel.linvel.data.0[0][0]) {
                    (
                        PlayerState {
                            is_dashing: true,
                            is_dash_turning: false,
                            state: PlayerStateEnum::Ground { .. },
                            ..
                        },
                        linvel,
                    ) if (linvel > 0. && x_axis < 0) || (linvel < 0. && x_axis > 0) => {
                        dash_turn_events.send(DashTurnEvent {
                            is_dash_turning: true,
                        });
                    }
                    (
                        PlayerState {
                            is_dash_turning: true,
                            ..
                        },
                        linvel,
                    ) if x_axis == 0
                        || (linvel > 0. && x_axis > 0)
                        || (linvel < 0. && x_axis < 0) =>
                    {
                        dash_turn_events.send(DashTurnEvent {
                            is_dash_turning: false,
                        });
                    }
                    _ => {}
                }
                match x_axis {
                    _ if x_axis > 0 => {
                        movement_events.send(MovementEvent::Right);
                        if rb_vel.linvel.data.0[0][0] > cap {
                            return;
                        }
                    }
                    _ if x_axis < 0 => {
                        movement_events.send(MovementEvent::Left);
                        if rb_vel.linvel.data.0[0][0] < -cap {
                            return;
                        }
                    }
                    _ => {}
                }
                if x_axis != 0 {
                    let move_delta = Vector2::new(x_axis as f32, 0.);
                    let multiplier = match (&player.state.state, is_running) {
                        (PlayerStateEnum::Air { .. }, false) => MOVE_IMPULSE_MULTIPLIER_AIR,
                        (PlayerStateEnum::Air { .. }, true) => MOVE_IMPULSE_MULTIPLIER_AIR_RUN,
                        (PlayerStateEnum::Ground { .. }, false) => MOVE_IMPULSE_MULTIPLIER_GROUND,
                        (PlayerStateEnum::Ground { .. }, true) => {
                            MOVE_IMPULSE_MULTIPLIER_GROUND_RUN
                        }
                    };
                    rb_vel.apply_impulse(rb_mprops, move_delta * multiplier);
                }
                match player.state.state {
                    PlayerStateEnum::Ground {
                        is_turning: false,
                        is_walking,
                        frame,
                    } if (x_axis == 1 && rb_vel.linvel.data.0[0][0] < 0.)
                        || (x_axis == -1 && rb_vel.linvel.data.0[0][0] > 0.) =>
                    {
                        player.state.state = PlayerStateEnum::Ground {
                            frame,
                            is_walking,
                            is_turning: true,
                        };
                        c_mat.friction = 0.
                    }
                    PlayerStateEnum::Ground {
                        is_turning: true,
                        is_walking,
                        frame,
                    } if (x_axis == 1 && rb_vel.linvel.data.0[0][0] > 0.)
                        || (x_axis == -1 && rb_vel.linvel.data.0[0][0] < 0.) =>
                    {
                        player.state.state = PlayerStateEnum::Ground {
                            frame,
                            is_walking,
                            is_turning: false,
                        };
                        c_mat.friction = 1.
                    }
                    _ => c_mat.friction = 1.,
                }
            }
            PlayerState {
                is_stooping: true,
                state: PlayerStateEnum::Ground { .. },
                ..
            } => {}
        }
    }
}
