use crate::UiButtonVariant;
use bevy::{prelude::*, utils::HashMap};
use enum_iterator::all;
use shrm_core::{ThemeVariant, TileVariant};

use super::object::ObjectVariant;

#[derive(Default, Resource)]
pub struct TileSpriteHandles(pub HashMap<TileVariant, Handle<Image>>);

#[derive(Default, Resource)]
pub struct TileSpriteHandlesTransparent(pub HashMap<TileVariant, Handle<Image>>);

#[derive(Default, Resource)]
pub struct ThemeSpriteHandles(pub HashMap<ThemeVariant, Handle<Image>>);

#[derive(Default, Resource)]
pub struct UiButtonSpriteHandles(pub HashMap<UiButtonVariant, Handle<Image>>);

#[derive(Default, Resource)]
pub struct ObjectSpriteHandles(pub HashMap<ObjectVariant, Handle<Image>>);

pub(crate) fn load_course_sprites(
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    mut tile_sprite_handles_transparent: ResMut<TileSpriteHandlesTransparent>,
    mut theme_sprite_handles: ResMut<ThemeSpriteHandles>,
    mut ui_button_sprite_handles: ResMut<UiButtonSpriteHandles>,
    mut object_sprite_handles: ResMut<ObjectSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    tile_sprite_handles.0 = HashMap::default();
    for tile_variant in all::<TileVariant>().collect::<Vec<_>>().into_iter() {
        let index = tile_variant.get_sprite_sheet_index();
        tile_sprite_handles.0.insert(
            tile_variant,
            asset_server.load(format!("MW_Field_plain_0_{index}.png")),
        );
    }

    tile_sprite_handles_transparent.0 = HashMap::default();
    for tile_variant in all::<TileVariant>().collect::<Vec<_>>().into_iter() {
        let index = tile_variant.get_sprite_sheet_index();
        tile_sprite_handles_transparent.0.insert(
            tile_variant,
            asset_server.load(format!("0MW_Field_plain_0_{index}.png")),
        );
    }

    theme_sprite_handles.0 = HashMap::default();
    for theme_variant in all::<ThemeVariant>().collect::<Vec<_>>().into_iter() {
        let name = theme_variant.get_name().to_string();
        theme_sprite_handles.0.insert(
            theme_variant,
            asset_server.load(format!("MW_DV_{name}_V.00_0.png")),
        );
    }

    ui_button_sprite_handles.0 = HashMap::default();
    for ui_button_variant in all::<UiButtonVariant>().collect::<Vec<_>>().into_iter() {
        let name = ui_button_variant.get_path().to_string();
        ui_button_sprite_handles
            .0
            .insert(ui_button_variant, asset_server.load(name));
    }

    object_sprite_handles.0 = HashMap::default();
    for object_variant in all::<ObjectVariant>().collect::<Vec<_>>().into_iter() {
        let name = object_variant.get_name().to_string();
        object_sprite_handles.0.insert(
            object_variant,
            asset_server.load(format!("MW_Object_{name}_0.png")),
        );
    }
}
