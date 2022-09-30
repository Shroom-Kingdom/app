use app_core::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(clear_input));
    }
}

// TODO not yet working
fn clear_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut mouse_input: ResMut<Input<MouseButton>>,
) {
    keyboard_input.clear();
    mouse_input.clear();
}
