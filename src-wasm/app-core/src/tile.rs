use crate::TileVariant;

pub struct SpawnTileEvent {
    pub tile_variant: TileVariant,
    pub grid_pos: [i32; 2],
}

#[derive(Default)]
pub struct DespawnTileEvent {
    pub grid_pos: [i32; 2],
    pub force: bool,
}
