use bevy::{prelude::*, utils::HashMap};

#[derive(Default)]
pub struct CourseSpriteHandles(pub HashMap<CourseTile, Handle<Image>>);

#[derive(Eq, Hash, PartialEq)]
pub enum CourseTile {
    Ground,
    HardBlock,
    RotatingBlock,
    DonutBlock,
    CloudBlock,
}

pub(crate) fn load_course_sprites(
    mut sprite_handles: ResMut<CourseSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    sprite_handles.0 = HashMap::default();
    sprite_handles.0.insert(
        CourseTile::Ground,
        asset_server.load("MW_Field_plain_0_193.png"),
    );
    sprite_handles.0.insert(
        CourseTile::HardBlock,
        asset_server.load("MW_Field_plain_0_6.png"),
    );
    sprite_handles.0.insert(
        CourseTile::RotatingBlock,
        asset_server.load("MW_Field_plain_0_1.png"),
    );
    sprite_handles.0.insert(
        CourseTile::DonutBlock,
        asset_server.load("MW_Field_plain_0_64.png"),
    );
    sprite_handles.0.insert(
        CourseTile::CloudBlock,
        asset_server.load("MW_Field_plain_0_102.png"),
    );
}
