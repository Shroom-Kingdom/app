use crate::{Player, PlayerState, PlayerStateChangeEvent, PlayerVelocity};
use app_core::GameModeToggleEvent;
use bevy::prelude::*;

pub fn toggle_game_mode(
    mut query: Query<(&mut Player, &Children, &mut PlayerVelocity)>,
    mut child_query: Query<&mut TextureAtlasSprite>,
    mut game_mode_toggle_event: EventReader<GameModeToggleEvent>,
    mut psc_events: EventWriter<PlayerStateChangeEvent>,
) {
    for _ in game_mode_toggle_event.iter() {
        let (mut player, children, mut vel) = query.single_mut();
        vel.0.x = 0.;
        vel.0.y = 0.;
        let child = children.first().unwrap();
        let mut sprite = child_query.get_mut(*child).unwrap();
        sprite.flip_x = false;

        player.state = PlayerState::float();
        psc_events.send(PlayerStateChangeEvent {
            state: player.state.clone(),
        });
    }
}
