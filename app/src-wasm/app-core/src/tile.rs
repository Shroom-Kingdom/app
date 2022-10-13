use bevy::prelude::*;
use shrm_core::TileVariant;

pub struct SpawnTileEvent {
    pub tile_variant: TileVariant,
    pub grid_pos: [i32; 2],
}

#[derive(Default)]
pub struct DespawnTileEvent {
    pub grid_pos: [i32; 2],
    pub force: bool,
}

pub struct GroundTileUpdateEvent {
    pub entity: Entity,
    pub index: usize,
}

pub(crate) fn update_ground_tile(
    mut query: Query<&mut TextureAtlasSprite>,
    mut child_query: Query<&Children>,
    mut events: EventReader<GroundTileUpdateEvent>,
) {
    for GroundTileUpdateEvent { entity, index } in events.iter() {
        if let Ok(children) = child_query.get_mut(*entity) {
            let child = children[1];
            let mut sprite = query.get_mut(child).unwrap();
            *sprite = TextureAtlasSprite::new(*index);
        }
    }
}
