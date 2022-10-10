use app_core::{
    GameMode, GameModeEdit, GameModeToggleButton, GameModeToggleButtonImage, GameModeToggleEvent,
    TilePlacePreview, UiButtonSpriteHandles, UiButtonVariant,
};
use app_player::{Player, PlayerState, PlayerStateChangeEvent, PlayerVelocity};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn toggle_game_mode(
    mut query: Query<(&mut Player, &Children, &mut PlayerVelocity)>,
    mut child_query: Query<&mut TextureAtlasSprite>,
    mut edit_query: Query<&mut Style, With<GameModeEdit>>,
    mut button_query: Query<&mut GameModeToggleButton>,
    mut button_image_query: Query<&mut UiImage, With<GameModeToggleButtonImage>>,
    mut game_mode: ResMut<GameMode>,
    ui_button_sprite_handles: Res<UiButtonSpriteHandles>,
    mut tile_place_preview: ResMut<TilePlacePreview>,
    mut game_mode_toggle_event: EventReader<GameModeToggleEvent>,
    mut psc_events: EventWriter<PlayerStateChangeEvent>,
    mut commands: Commands,
) {
    for GameModeToggleEvent { is_editing } in game_mode_toggle_event.iter() {
        let (mut player, children, mut vel) = query.single_mut();
        vel.0.x = 0.;
        vel.0.y = 0.;
        let child = children.first().unwrap();
        let mut sprite = child_query.get_mut(*child).unwrap();
        sprite.flip_x = false;

        *game_mode = GameMode::Build {
            is_editing: *is_editing,
        };
        let mut button = button_query.single_mut();
        button.is_editing = *is_editing;
        let mut game_mode_button = button_image_query.single_mut();
        *game_mode_button = UiImage(
            ui_button_sprite_handles
                .0
                .get(&UiButtonVariant::GameModeSwitch {
                    is_editing: *is_editing,
                })
                .unwrap()
                .clone(),
        );

        for mut style in edit_query.iter_mut() {
            style.display = if *is_editing {
                Display::Flex
            } else {
                Display::None
            };
        }

        if let Some((entity, _)) = tile_place_preview.0 {
            commands.entity(entity).despawn_recursive();
            tile_place_preview.0 = None;
        }

        player.state = PlayerState::float();
        psc_events.send(PlayerStateChangeEvent {
            state: player.state.clone(),
        });
    }
}
