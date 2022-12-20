use bevy::prelude::*;
use shrm_core::TileVariant;

#[derive(Component)]
#[repr(transparent)]
pub struct TileComponent(pub TileVariant);

#[derive(Component)]
pub struct TilePreview;

#[derive(Component)]
pub struct TileNotEditable;

#[derive(Resource)]
pub struct TilePlacePreview(pub Option<(Entity, [i32; 2])>);

#[derive(Clone, Debug)]
pub struct Tile {
    pub entity: Entity,
    pub variant: TileVariant,
    pub mtrx: Option<GroundSurroundingMatrix>,
}

#[derive(Clone, Debug)]
pub struct GroundSurroundingMatrix(pub [[bool; 3]; 3]);

#[derive(Debug, Default, Resource)]
pub struct SelectedTile(pub Option<TileVariant>);
