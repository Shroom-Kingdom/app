use crate::{ThemeVariant, TileVariant, UiButtonVariant};
use bevy::{prelude::*, utils::HashMap};
use enum_iterator::all;

#[derive(Default)]
pub struct TileSpriteHandles(pub HashMap<TileVariant, Handle<Image>>);

#[derive(Default)]
pub struct TileSpriteHandlesTransparent(pub HashMap<TileVariant, Handle<Image>>);

#[derive(Default)]
pub struct ThemeSpriteHandles(pub HashMap<ThemeVariant, Handle<Image>>);

#[derive(Default)]
pub struct UiButtonSpriteHandles(pub HashMap<UiButtonVariant, Handle<Image>>);

pub(crate) fn load_course_sprites(
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    mut tile_sprite_handles_transparent: ResMut<TileSpriteHandlesTransparent>,
    mut theme_sprite_handles: ResMut<ThemeSpriteHandles>,
    mut ui_button_sprite_handles: ResMut<UiButtonSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    tile_sprite_handles.0 = HashMap::default();
    for tile_variant in all::<TileVariant>().collect::<Vec<_>>().into_iter() {
        let index = tile_variant.get_sprite_sheet_index();
        tile_sprite_handles.0.insert(
            tile_variant,
            asset_server.load(&format!("MW_Field_plain_0_{}.png", index)),
        );
    }

    tile_sprite_handles_transparent.0 = HashMap::default();
    for tile_variant in all::<TileVariant>().collect::<Vec<_>>().into_iter() {
        let index = tile_variant.get_sprite_sheet_index();
        tile_sprite_handles_transparent.0.insert(
            tile_variant,
            asset_server.load(&format!("0MW_Field_plain_0_{}.png", index)),
        );
    }

    theme_sprite_handles.0 = HashMap::default();
    for theme_variant in all::<ThemeVariant>().collect::<Vec<_>>().into_iter() {
        let name = theme_variant.get_name().to_string();
        theme_sprite_handles.0.insert(
            theme_variant,
            asset_server.load(&format!("MW_DV_{}_V.00_0.png", name)),
        );
    }

    ui_button_sprite_handles.0 = HashMap::default();
    for ui_button_variant in all::<UiButtonVariant>().collect::<Vec<_>>().into_iter() {
        let name = ui_button_variant.get_path().to_string();
        ui_button_sprite_handles
            .0
            .insert(ui_button_variant, asset_server.load(&name));
    }
}
