use crate::{Player, PlayerState, PlayerStateEnum, PlayerVelocity};
use app_config::*;
use app_core::{GameMode, Ground};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier::prelude::*;

pub enum FacingDirectionEvent {
    Left,
    Right,
}

pub struct DashTurnEvent {
    pub is_dash_turning: bool,
}

pub fn run(mut query: Query<&mut Player>, keyboard_input: Res<Input<KeyCode>>) {
    if let Ok(mut player) = query.get_single_mut() {
        let run =
            keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);
        player.state.is_running = run;
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn movement(
    mut query: Query<
        (
            &mut Player,
            &mut PlayerVelocity,
            &ReadMassProperties,
            &mut Friction,
            &Transform,
            &Children,
        ),
        With<RigidBody>,
    >,
    child_query: Query<(Entity, &Collider)>,
    ground_query: Query<&Ground>,
    keyboard_input: Res<Input<KeyCode>>,
    mut facing_direction_events: EventWriter<FacingDirectionEvent>,
    dash_turn_events: EventWriter<DashTurnEvent>,
    ctx: Res<RapierContext>,
    game_mode: Res<GameMode>,
) {
    if let GameMode::Build { is_editing: true } = *game_mode {
        if let Ok((_, mut vel, _, _, _, _)) = query.get_single_mut() {
            let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
            let right =
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
            let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
            let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);

            let x_axis = -(left as i8) + right as i8;
            let y_axis = -(down as i8) + up as i8;

            vel.0.x = x_axis as f32 * LINVEL_BUILD;
            vel.0.y = y_axis as f32 * LINVEL_BUILD;
        }
    } else if let Ok((mut player, mut vel, mprops, mut friction, transform, children)) =
        query.get_single_mut()
    {
        let child = children.get(1).unwrap();
        let (entity, collider) = child_query.get(*child).unwrap();
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
                    player.state.is_dashing = vel.0.x.abs() > RUN_THRESHOLD;
                }

                let left =
                    keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
                let right =
                    keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
                let x_axis = -(left as i8) + right as i8;

                send_dash_turn_event(&player, &vel, dash_turn_events, x_axis);

                let cap = match (player.state.is_stooping, is_running) {
                    (true, _) => LINVEL_CAP_STOOP,
                    (false, true) => LINVEL_CAP_RUN,
                    (false, false) => LINVEL_CAP_WALK,
                };
                match x_axis {
                    _ if x_axis > 0 => {
                        facing_direction_events.send(FacingDirectionEvent::Right);
                        if vel.0.x > cap {
                            return;
                        }
                    }
                    _ if x_axis < 0 => {
                        facing_direction_events.send(FacingDirectionEvent::Left);
                        if vel.0.x < -cap {
                            return;
                        }
                    }
                    _ => {}
                }

                update_velocity(
                    &mut vel,
                    &player,
                    entity,
                    mprops,
                    transform,
                    collider,
                    &ctx,
                    &ground_query,
                    x_axis,
                    is_running,
                );

                update_friction_on_turn(&mut player, &mut vel, &mut friction, x_axis);
            }
            PlayerState {
                is_stooping: true,
                state: PlayerStateEnum::Ground { .. },
                ..
            } => {}
        }
    }
}

fn send_dash_turn_event(
    player: &Player,
    vel: &PlayerVelocity,
    mut dash_turn_events: EventWriter<DashTurnEvent>,
    x_axis: i8,
) {
    match (&player.state, vel.0.x) {
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
        ) if x_axis == 0 || (linvel > 0. && x_axis > 0) || (linvel < 0. && x_axis < 0) => {
            dash_turn_events.send(DashTurnEvent {
                is_dash_turning: false,
            });
        }
        _ => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn update_velocity(
    vel: &mut PlayerVelocity,
    player: &Player,
    entity: Entity,
    mprops: &ReadMassProperties,
    transform: &Transform,
    collider: &Collider,
    ctx: &RapierContext,
    ground_query: &Query<&Ground>,
    x_axis: i8,
    is_running: bool,
) {
    if x_axis != 0 {
        let move_delta = Vec2::new(x_axis as f32, 0.);
        let multiplier = match (&player.state.state, is_running) {
            (PlayerStateEnum::Air { .. }, false) => MOVE_IMPULSE_MULTIPLIER_AIR,
            (PlayerStateEnum::Air { .. }, true) => MOVE_IMPULSE_MULTIPLIER_AIR_RUN,
            (PlayerStateEnum::Ground { .. }, false) => MOVE_IMPULSE_MULTIPLIER_GROUND,
            (PlayerStateEnum::Ground { .. }, true) => MOVE_IMPULSE_MULTIPLIER_GROUND_RUN,
        };

        #[allow(clippy::blocks_in_if_conditions)]
        if ctx
            .cast_shape(
                transform.translation.xy(),
                transform.rotation.to_axis_angle().1,
                move_delta,
                collider,
                COLLIDER_TOI_THRESHOLD * 10.,
                QueryFilter {
                    predicate: Some(&|collider_entity| {
                        collider_entity != entity && ground_query.get(collider_entity).is_err()
                    }),
                    ..Default::default()
                }
                .exclude_sensors(),
            )
            .is_none()
        {
            vel.0.x += x_axis as f32 * mprops.0.into_rapier(RAPIER_SCALE).inv_mass * multiplier;
        }
    }
}

fn update_friction_on_turn(
    player: &mut Player,
    vel: &mut PlayerVelocity,
    friction: &mut Friction,
    x_axis: i8,
) {
    match player.state.state {
        PlayerStateEnum::Ground {
            is_turning: false,
            is_walking,
            frame,
        } if (x_axis == 1 && vel.0.x < 0.) || (x_axis == -1 && vel.0.x > 0.) => {
            player.state.state = PlayerStateEnum::Ground {
                frame,
                is_walking,
                is_turning: true,
            };
            friction.coefficient = 0.
        }
        PlayerStateEnum::Ground {
            is_turning: true,
            is_walking,
            frame,
        } if x_axis == 0 || (x_axis == 1 && vel.0.x > 0.) || (x_axis == -1 && vel.0.x < 0.) => {
            player.state.state = PlayerStateEnum::Ground {
                frame,
                is_walking,
                is_turning: false,
            };
            friction.coefficient = 1.
        }
        PlayerStateEnum::Ground { .. } => {}
        _ => friction.coefficient = 1.,
    }
}
