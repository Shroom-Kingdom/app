use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};

#[derive(Debug, TypeUuid)]
#[uuid = "81a23571-1f35-4f20-b1ea-30e5c2612049"]
pub struct Course {
    pub texture_atlas_handle: Handle<TextureAtlas>,
    pub tiles: HashMap<[i32; 2], Tile>,
    pub theme: CourseTheme,
}

impl Course {
    pub fn new(
        theme: CourseTheme,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load(theme.get_asset_str());
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 48);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Course {
            texture_atlas_handle,
            tiles: HashMap::default(),
            theme,
        }
    }
}

#[derive(Debug)]
pub enum CourseTheme {
    Plain,
}

impl CourseTheme {
    pub fn get_asset_str(&self) -> &str {
        match self {
            CourseTheme::Plain => "MW_Field_plain_0.png",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub variant: TileVariant,
}

#[derive(Clone, Debug)]
pub enum TileVariant {
    Block,
}
