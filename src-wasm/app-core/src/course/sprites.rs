use crate::TileVariant;
use bevy::{prelude::*, utils::HashMap};
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct CourseSpriteHandles(pub HashMap<TileVariant, Handle<Image>>);

pub(crate) fn load_course_sprites(
    mut sprite_handles: ResMut<CourseSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    sprite_handles.0 = HashMap::default();
    for tile_variant in TileVariant::iter() {
        let index = tile_variant.get_sprite_sheet_index();
        sprite_handles.0.insert(
            tile_variant,
            asset_server.load(&format!("MW_Field_plain_0_{}.png", index)),
        );
    }
}
