#[derive(Debug)]
pub enum GameMode {
    Build { is_editing: bool },
    Play,
}
