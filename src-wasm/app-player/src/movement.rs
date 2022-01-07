use crate::{Player, PlayerState, PlayerStateEnum, PlayerVelocity};
use app_config::{
    COLLIDER_TOI_THRESHOLD, LINVEL_CAP_RUN, LINVEL_CAP_STOOP, LINVEL_CAP_WALK,
    MOVE_IMPULSE_MULTIPLIER_AIR, MOVE_IMPULSE_MULTIPLIER_AIR_RUN, MOVE_IMPULSE_MULTIPLIER_GROUND,
    MOVE_IMPULSE_MULTIPLIER_GROUND_RUN, RUN_THRESHOLD,
};
use app_core::Ground;
use bevy::prelude::*;
use bevy_rapier::{na::Vector2, prelude::*};

pub enum FacingDirectionEvent {
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
        Entity,
        &mut Player,
        &mut PlayerVelocity,
        &RigidBodyMassProps,
        &mut ColliderMaterial,
        &RigidBodyPosition,
        &ColliderShape,
    )>,
    ground_query: Query<&Ground>,
    colliders: QueryPipelineColliderComponentsQuery,
    keyboard_input: Res<Input<KeyCode>>,
    mut facing_direction_events: EventWriter<FacingDirectionEvent>,
    mut dash_turn_events: EventWriter<DashTurnEvent>,
    query_pipeline: Res<QueryPipeline>,
) {
    if let Ok((entity, mut player, mut vel, rb_mprops, mut c_mat, rb_pos, shape)) =
        query.single_mut()
    {
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
                    player.state.is_dashing = vel.0[0].abs() > RUN_THRESHOLD;
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
                match (&player.state, vel.0[0]) {
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
                        facing_direction_events.send(FacingDirectionEvent::Right);
                        if vel.0[0] > cap {
                            return;
                        }
                    }
                    _ if x_axis < 0 => {
                        facing_direction_events.send(FacingDirectionEvent::Left);
                        if vel.0[0] < -cap {
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
                    let colliders = QueryPipelineColliderComponentsSet(&colliders);

                    #[allow(clippy::blocks_in_if_conditions)]
                    if query_pipeline
                        .cast_shape(
                            &colliders,
                            &rb_pos.position,
                            &move_delta,
                            &*shape.0,
                            COLLIDER_TOI_THRESHOLD * 10.,
                            InteractionGroups::default(),
                            Some(&|collider| {
                                collider.entity() != entity
                                    && ground_query.get(collider.entity()).is_err()
                            }),
                        )
                        .is_none()
                    {
                        vel.0 += move_delta * multiplier * rb_mprops.effective_inv_mass;
                    }
                }
                match player.state.state {
                    PlayerStateEnum::Ground {
                        is_turning: false,
                        is_walking,
                        frame,
                    } if (x_axis == 1 && vel.0[0] < 0.) || (x_axis == -1 && vel.0[0] > 0.) => {
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
                    } if x_axis == 0
                        || (x_axis == 1 && vel.0[0] > 0.)
                        || (x_axis == -1 && vel.0[0] < 0.) =>
                    {
                        player.state.state = PlayerStateEnum::Ground {
                            frame,
                            is_walking,
                            is_turning: false,
                        };
                        c_mat.friction = 1.
                    }
                    PlayerStateEnum::Ground { .. } => {}
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
