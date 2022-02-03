use bevy::prelude::*;

#[derive(Clone, Debug)]
pub struct Tile {
    pub entity: Entity,
    pub variant: TileVariant,
}

#[derive(Clone, Component, Debug, Eq, Hash, PartialEq)]
pub enum TileVariant {
    Ground,
    HardBlock,
    RotatingBlock,
    DonutBlock,
    CloudBlock,
}

pub struct SelectedTile(pub TileVariant);

impl TileVariant {
    pub fn get_sprite_sheet_index(&self) -> usize {
        match self {
            Self::Ground => 193,
            Self::HardBlock => 6,
            Self::RotatingBlock => 1,
            Self::DonutBlock => 64,
            Self::CloudBlock => 102,
        }
    }
}
