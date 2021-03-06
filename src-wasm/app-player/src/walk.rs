use crate::{Player, PlayerStateEnum, PlayerVelocity};
use bevy::prelude::*;

#[derive(Debug)]
pub enum WalkEvent {
    Start,
    Advance,
    Stop,
}

#[derive(Component, Deref, DerefMut)]
pub struct WalkAnimationTimer(pub Timer);

pub fn walk_animation(
    mut query: Query<(&Player, &mut WalkAnimationTimer, &PlayerVelocity)>,
    time: Res<Time>,
    mut walk_event: EventWriter<WalkEvent>,
) {
    for (player, mut timer, vel) in query.iter_mut() {
        if let PlayerStateEnum::Ground { is_walking, .. } = player.state.state {
            let linvel = vel.0.x.abs();
            if is_walking && linvel < f32::EPSILON {
                timer.reset();
                walk_event.send(WalkEvent::Stop);
            } else {
                timer.tick(time.delta() * linvel as u32);
                if timer.finished() {
                    walk_event.send(WalkEvent::Advance);
                }
            }
        }
    }
}

pub fn walk_start(
    mut query: Query<(&Player, &mut WalkAnimationTimer, &PlayerVelocity)>,
    mut walk_event: EventWriter<WalkEvent>,
) {
    for (player, mut timer, vel) in query.iter_mut() {
        if let PlayerStateEnum::Ground {
            is_walking: false, ..
        } = player.state.state
        {
            if vel.0.x.abs() > f32::EPSILON {
                timer.reset();
                walk_event.send(WalkEvent::Start);
            }
        }
    }
}
