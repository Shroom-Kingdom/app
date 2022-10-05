use app_core::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(clear_input));
    }
}

fn clear_input(mut mouse_input: ResMut<Input<MouseButton>>) {
    mouse_input.reset(MouseButton::Left);
    mouse_input.reset(MouseButton::Right);
}
