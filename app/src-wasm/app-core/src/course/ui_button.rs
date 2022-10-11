use bevy::prelude::*;
use enum_iterator::Sequence;

#[derive(Clone, Component, Debug, Eq, Hash, PartialEq, Sequence)]
pub enum UiButtonVariant {
    GameModeSwitch { is_editing: bool },
    Build,
    Export,
    Import,
}

impl UiButtonVariant {
    pub fn get_path(&self) -> &str {
        match self {
            Self::GameModeSwitch { is_editing } => {
                if *is_editing {
                    "MW_Field_anime_block_on_N_wait.1_0.png"
                } else {
                    "MW_Field_anime_block_off_W_wait.1_0.png"
                }
            }
            Self::Build => "icons/build.png",
            Self::Export => "icons/export.png",
            Self::Import => "icons/import.png",
        }
    }
}
