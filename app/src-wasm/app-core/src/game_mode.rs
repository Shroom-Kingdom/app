use bevy::prelude::*;

#[derive(Debug, Resource)]
pub enum GameMode {
    Build { is_editing: bool },
    Play,
}

pub struct GameModeToggleEvent {
    pub is_editing: bool,
}

#[derive(Component)]
pub struct GameModeEdit;

#[derive(Component)]
pub struct GameModeToggleButton {
    pub is_editing: bool,
}

#[derive(Component)]
pub struct GameModeToggleButtonImage;
