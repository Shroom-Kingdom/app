pub(crate) mod goal_pole;
pub(crate) mod object;
pub(crate) mod resource;
pub(crate) mod sprites;
pub(crate) mod theme;
pub(crate) mod tile;
pub(crate) mod ui_button;

use crate::{GroundTileUpdateEvent, GroundVariant, ThemeVariant, Tile, TileVariant};
use anyhow::Result;
use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use brotli::{
    enc::{backward_references::BrotliEncoderMode, writer::CompressorWriter, BrotliEncoderParams},
    DecompressorWriter,
};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Course {
    pub tiles: HashMap<[i32; 2], TileVariant>,
    pub theme: ThemeVariant,
    pub goal_pos_x: i32,
}

#[derive(Clone, Debug, TypeUuid)]
#[uuid = "81a23571-1f35-4f20-b1ea-30e5c2612049"]
pub struct CourseRes {
    pub texture_atlas_handle: Handle<TextureAtlas>,
    pub texture_atlas_handle_transparent: Handle<TextureAtlas>,
    pub tiles: HashMap<[i32; 2], Tile>,
    pub theme: ThemeVariant,
    pub goal_pos_x: i32,
}

pub struct CourseLoading(pub Option<Course>);

impl Course {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let course_as_str = ron::to_string(self)?;
        let mut res = vec![];
        let params = BrotliEncoderParams {
            mode: BrotliEncoderMode::BROTLI_MODE_TEXT,
            quality: 11,
            ..Default::default()
        };
        let mut writer = CompressorWriter::with_params(&mut res, 4096, &params);
        writer.write_all(course_as_str.as_bytes())?;
        writer.flush()?;
        drop(writer);
        Ok(res)
    }

    pub fn deserialize(buf: Vec<u8>) -> Result<Self> {
        let mut decompressed = vec![];
        let mut writer = DecompressorWriter::new(&mut decompressed, 4096);
        writer.write_all(&buf)?;
        writer.flush()?;
        drop(writer);
        let course_as_str = String::from_utf8(decompressed)?;
        Ok(ron::from_str(&course_as_str)?)
    }
}

impl From<&CourseRes> for Course {
    fn from(course: &CourseRes) -> Self {
        Self {
            tiles: {
                let mut tiles = HashMap::new();
                for (pos, tile) in course.tiles.clone() {
                    tiles.insert(pos, tile.variant);
                }
                tiles
            },
            theme: course.theme.clone(),
            goal_pos_x: course.goal_pos_x,
        }
    }
}

pub fn get_surrounding_matrix(
    grid_pos: &[i32; 2],
    tiles: &mut HashMap<[i32; 2], Tile>,
    events: &mut HashMap<Entity, GroundTileUpdateEvent>,
) -> [[bool; 3]; 3] {
    let mut surrounding_matrix = [
        [false, false, false],
        [false, false, false],
        [false, false, false],
    ];
    for x in grid_pos[0] - 1..=grid_pos[0] + 1 {
        for y in grid_pos[1] - 1..=grid_pos[1] + 1 {
            let pos = [x, y];
            if &pos == grid_pos {
                continue;
            }
            if x < 0 || y < 0 {
                surrounding_matrix[(grid_pos[1] - y + 1) as usize]
                    [(x - grid_pos[0] + 1) as usize] = true;
                continue;
            }

            if let Some(Tile {
                entity,
                variant: TileVariant::Ground(ground_variant),
                mtrx,
            }) = tiles.get_mut(&pos)
            {
                surrounding_matrix[(grid_pos[1] - y + 1) as usize]
                    [(x - grid_pos[0] + 1) as usize] = true;
                if let Some(mtrx) = mtrx {
                    mtrx.0[(y - grid_pos[1] + 1) as usize][(grid_pos[0] - x + 1) as usize] = true;
                    *ground_variant = GroundVariant::from_surrounding_matrix(&mtrx.0);
                    events.insert(
                        *entity,
                        GroundTileUpdateEvent {
                            entity: *entity,
                            index: ground_variant.get_sprite_sheet_index(),
                        },
                    );
                }
            }
        }
    }
    surrounding_matrix
}
