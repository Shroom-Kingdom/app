use crate::TileVariant;
use bevy::{prelude::*, utils::HashMap};

#[derive(Default)]
pub struct CourseSpriteHandles(pub HashMap<TileVariant, Handle<Image>>);

pub(crate) fn load_course_sprites(
    mut sprite_handles: ResMut<CourseSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    sprite_handles.0 = HashMap::default();
    sprite_handles.0.insert(
        TileVariant::Ground,
        asset_server.load("MW_Field_plain_0_193.png"),
    );
    sprite_handles.0.insert(
        TileVariant::HardBlock,
        asset_server.load("MW_Field_plain_0_6.png"),
    );
    sprite_handles.0.insert(
        TileVariant::RotatingBlock,
        asset_server.load("MW_Field_plain_0_1.png"),
    );
    sprite_handles.0.insert(
        TileVariant::DonutBlock,
        asset_server.load("MW_Field_plain_0_64.png"),
    );
    sprite_handles.0.insert(
        TileVariant::CloudBlock,
        asset_server.load("MW_Field_plain_0_102.png"),
    );
}
