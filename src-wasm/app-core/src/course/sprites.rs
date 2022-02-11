use crate::{ThemeVariant, TileVariant};
use bevy::{prelude::*, utils::HashMap};
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct TileSpriteHandles(pub HashMap<TileVariant, Handle<Image>>);

#[derive(Default)]
pub struct ThemeSpriteHandles(pub HashMap<ThemeVariant, Handle<Image>>);

pub(crate) fn load_course_sprites(
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    mut theme_sprite_handles: ResMut<ThemeSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    tile_sprite_handles.0 = HashMap::default();
    for tile_variant in TileVariant::iter() {
        let index = tile_variant.get_sprite_sheet_index();
        tile_sprite_handles.0.insert(
            tile_variant,
            asset_server.load(&format!("MW_Field_plain_0_{}.png", index)),
        );
    }

    theme_sprite_handles.0 = HashMap::default();
    for theme_variant in ThemeVariant::iter() {
        let name = theme_variant.get_name().to_string();
        theme_sprite_handles.0.insert(
            theme_variant,
            asset_server.load(&format!("MW_DV_{}_V.00_0.png", name)),
        );
    }
}
