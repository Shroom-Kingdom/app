use crate::{Player, PlayerState, PlayerStateEnum};
use app_config::{
    LINVEL_CAP_AIR, LINVEL_CAP_GROUND, LINVEL_CAP_STOOP, MOVE_IMPULSE_MULTIPLIER_AIR,
    MOVE_IMPULSE_MULTIPLIER_GROUND,
};
use bevy::prelude::*;
use bevy_rapier::{na::Vector2, prelude::*};

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
                state:
                    PlayerStateEnum::Jump { .. }
                    | PlayerStateEnum::Fall
                    | PlayerStateEnum::Wait
                    | PlayerStateEnum::Walk { .. },
            }
            | PlayerState {
                is_stooping: true,
                state: PlayerStateEnum::Jump { .. } | PlayerStateEnum::Fall,
            } => {
                let left =
                    keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
                let right =
                    keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

                let x_axis = -(left as i8) + right as i8;
                match x_axis {
                    _ if x_axis > 0
                        && player.state.is_stooping
                        && rb_vel.linvel.data.0[0][0] > LINVEL_CAP_STOOP =>
                    {
                        return;
                    }
                    _ if x_axis > 0 => {
                        sprite.flip_x = false;
                    }
                    _ if x_axis < 0
                        && player.state.is_stooping
                        && rb_vel.linvel.data.0[0][0] < -LINVEL_CAP_STOOP =>
                    {
                        return;
                    }
                    _ if x_axis < 0 => {
                        sprite.flip_x = true;
                    }
                    _ => {}
                }
                if x_axis != 0 {
                    let move_delta = Vector2::new(x_axis as f32, 0.);
                    let multiplier = match player.state.state {
                        PlayerStateEnum::Jump { .. } | PlayerStateEnum::Fall => {
                            MOVE_IMPULSE_MULTIPLIER_AIR
                        }
                        PlayerStateEnum::Wait | PlayerStateEnum::Walk { .. } => {
                            MOVE_IMPULSE_MULTIPLIER_GROUND
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
            } => {}
        }
    }
}

pub fn movement_cap(mut query: Query<(&Player, &mut RigidBodyVelocity)>) {
    for (player, mut rb_vel) in query.iter_mut() {
        let (x_cap, y_cap) = match player.state.state {
            PlayerStateEnum::Jump { .. } | PlayerStateEnum::Fall => LINVEL_CAP_AIR,
            PlayerStateEnum::Wait | PlayerStateEnum::Walk { .. } => LINVEL_CAP_GROUND,
        };
        if rb_vel.linvel.data.0[0][0] > x_cap {
            rb_vel.linvel.data.0[0][0] = x_cap;
        } else if rb_vel.linvel.data.0[0][0] < -x_cap {
            rb_vel.linvel.data.0[0][0] = -x_cap;
        }
        if rb_vel.linvel.data.0[0][1] > y_cap {
            rb_vel.linvel.data.0[0][1] = y_cap;
        } else if rb_vel.linvel.data.0[0][1] < -y_cap {
            rb_vel.linvel.data.0[0][1] = -y_cap;
        }
    }
}
