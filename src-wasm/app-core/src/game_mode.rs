#[derive(Debug)]
pub enum GameMode {
    Build { is_editing: bool },
    Play,
}

pub struct GameModeToggleEvent {
    pub is_editing: bool,
}
