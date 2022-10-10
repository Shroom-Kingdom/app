use crate::Player;
use app_config::*;
use app_core::{GameMode, GameModeToggleEvent};
use bevy::prelude::*;

pub struct KillEvent(Entity);

pub fn below_surface(
    mut query: Query<(Entity, &mut Transform), With<Player>>,
    mut game_mode: ResMut<GameMode>,
    mut kill_event: EventWriter<KillEvent>,
    mut game_mode_toggle_event: EventWriter<GameModeToggleEvent>,
) {
    for (entity, mut transform) in query.iter_mut() {
        if transform.translation.y >= BELOW_SURFACE_THRESHOLD {
            return;
        }

        match &mut *game_mode {
            GameMode::Build { is_editing } => {
                transform.translation.y = BELOW_SURFACE_RESPAWN_Y;

                if !*is_editing {
                    *is_editing = true;
                    game_mode_toggle_event.send(GameModeToggleEvent { is_editing: true });
                }
            }
            GameMode::Play => todo!(),
        }
        kill_event.send(KillEvent(entity));
    }
}
