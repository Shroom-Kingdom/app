use crate::{Player, PlayerState};
use bevy::prelude::*;
use bevy_rapier::prelude::*;

pub enum WalkEvent {
    Start,
    Advance,
    Stop,
}

pub fn walk_animation(
    mut query: Query<(&Player, &mut Timer, &RigidBodyVelocity)>,
    time: Res<Time>,
    mut walk_event: EventWriter<WalkEvent>,
) {
    for (player, mut timer, rb_vel) in query.iter_mut() {
        if let PlayerState::Walk { .. } = player.state {
            if rb_vel.linvel.data.0[0][0].abs() <= f32::EPSILON {
                timer.reset();
                walk_event.send(WalkEvent::Stop);
                return;
            }
            timer.tick(time.delta() * rb_vel.linvel.data.0[0][0].abs() as u32);
            if timer.finished() {
                walk_event.send(WalkEvent::Advance);
            }
        }
    }
}

pub fn walk_start(
    mut query: Query<(
        &Player,
        &mut Timer,
        &RigidBodyVelocity,
        &mut TextureAtlasSprite,
    )>,
    mut walk_event: EventWriter<WalkEvent>,
) {
    for (player, mut timer, rb_vel, mut sprite) in query.iter_mut() {
        if rb_vel.linvel.data.0[0][0] > f32::EPSILON {
            sprite.flip_x = false;
        } else if rb_vel.linvel.data.0[0][0] < -f32::EPSILON {
            sprite.flip_x = true;
        }
        if let PlayerState::Wait = player.state {
            if rb_vel.linvel.data.0[0][0].abs() > f32::EPSILON {
                timer.reset();
                walk_event.send(WalkEvent::Start);
            }
        }
    }
}
