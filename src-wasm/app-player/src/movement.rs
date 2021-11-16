use crate::{Player, PlayerState, PlayerStateEnum};
use app_config::{
    LINVEL_CAP_RUN, LINVEL_CAP_STOOP, LINVEL_CAP_WALK, MOVE_IMPULSE_MULTIPLIER_AIR,
    MOVE_IMPULSE_MULTIPLIER_AIR_RUN, MOVE_IMPULSE_MULTIPLIER_GROUND,
    MOVE_IMPULSE_MULTIPLIER_GROUND_RUN, RUN_THRESHOLD,
};
use bevy::prelude::*;
use bevy_rapier::{na::Vector2, prelude::*};

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
        &mut TextureAtlasSprite,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if let Ok((mut player, mut rb_vel, rb_mprops, mut c_mat, mut sprite)) = query.single_mut() {
        match player.state {
            PlayerState {
                is_stooping: false,
                is_running,
                state:
                    PlayerStateEnum::Jump { .. }
                    | PlayerStateEnum::Fall
                    | PlayerStateEnum::Wait
                    | PlayerStateEnum::Walk { .. },
                ..
            }
            | PlayerState {
                is_stooping: true,
                is_running,
                state: PlayerStateEnum::Jump { .. } | PlayerStateEnum::Fall,
                ..
            } => {
                if let PlayerState {
                    state: PlayerStateEnum::Wait | PlayerStateEnum::Walk { .. },
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
                match x_axis {
                    _ if x_axis > 0 => {
                        sprite.flip_x = false;
                        if rb_vel.linvel.data.0[0][0] > cap {
                            return;
                        }
                    }
                    _ if x_axis < 0 => {
                        sprite.flip_x = true;
                        if rb_vel.linvel.data.0[0][0] < -cap {
                            return;
                        }
                    }
                    _ => {}
                }
                if x_axis != 0 {
                    let move_delta = Vector2::new(x_axis as f32, 0.);
                    let multiplier = match (&player.state.state, is_running) {
                        (PlayerStateEnum::Jump { .. } | PlayerStateEnum::Fall, false) => {
                            MOVE_IMPULSE_MULTIPLIER_AIR
                        }
                        (PlayerStateEnum::Jump { .. } | PlayerStateEnum::Fall, true) => {
                            MOVE_IMPULSE_MULTIPLIER_AIR_RUN
                        }
                        (PlayerStateEnum::Wait | PlayerStateEnum::Walk { .. }, false) => {
                            MOVE_IMPULSE_MULTIPLIER_GROUND
                        }
                        (PlayerStateEnum::Wait | PlayerStateEnum::Walk { .. }, true) => {
                            MOVE_IMPULSE_MULTIPLIER_GROUND_RUN
                        }
                    };
                    rb_vel.apply_impulse(rb_mprops, move_delta * multiplier);
                }
                match player.state.state {
                    PlayerStateEnum::Walk {
                        is_turning: false,
                        frame,
                    } if (x_axis == 1 && rb_vel.linvel.data.0[0][0] < 0.)
                        || (x_axis == -1 && rb_vel.linvel.data.0[0][0] > 0.) =>
                    {
                        player.state.state = PlayerStateEnum::Walk {
                            frame,
                            is_turning: true,
                        };
                        c_mat.friction = 0.
                    }
                    PlayerStateEnum::Walk {
                        is_turning: true,
                        frame,
                    } if (x_axis == 1 && rb_vel.linvel.data.0[0][0] > 0.)
                        || (x_axis == -1 && rb_vel.linvel.data.0[0][0] < 0.) =>
                    {
                        player.state.state = PlayerStateEnum::Walk {
                            frame,
                            is_turning: false,
                        };
                        c_mat.friction = 1.
                    }
                    _ => c_mat.friction = 1.,
                }
            }
            PlayerState {
                is_stooping: true,
                state: PlayerStateEnum::Wait | PlayerStateEnum::Walk { .. },
                ..
            } => {}
        }
    }
}
