use bevy::{prelude::*, utils::HashMap};

#[derive(Default)]
pub struct CourseSpriteHandles(pub HashMap<CourseTile, Handle<Image>>);

#[derive(Eq, Hash, PartialEq)]
pub enum CourseTile {
    Block,
}

pub(crate) fn load_course_sprites(
    mut sprite_handles: ResMut<CourseSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    sprite_handles.0 = HashMap::default();
    sprite_handles.0.insert(
        CourseTile::Block,
        asset_server.load("MW_Field_plain_0_6.png"),
    );
}
