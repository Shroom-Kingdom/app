mod camera;
mod game_mode;

use app_core::AppState;
use bevy::prelude::*;
use camera::{move_player_on_goal_pole_drag, position_camera};
use game_mode::toggle_game_mode;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(clear_input))
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_update(AppState::Game).with_system(toggle_game_mode),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(AppState::Game).with_system(position_camera),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(AppState::Game).with_system(move_player_on_goal_pole_drag),
            );
    }
}

fn clear_input(mut mouse_input: ResMut<Input<MouseButton>>) {
    mouse_input.reset(MouseButton::Left);
    mouse_input.reset(MouseButton::Right);
}
