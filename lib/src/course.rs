use crate::{ThemeVariant, TileVariant};
use anyhow::Result;
use brotli::{
    enc::{backward_references::BrotliEncoderMode, BrotliEncoderParams},
    BrotliCompress, BrotliDecompress,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Cursor};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Course {
    pub tiles: HashMap<[i32; 2], TileVariant>,
    pub theme: ThemeVariant,
    pub goal_pos_x: i32,
}

impl Course {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let course_as_str = ron::to_string(self)?;
        let mut input = Cursor::new(course_as_str.as_bytes());
        let mut compressed = vec![];
        let params = BrotliEncoderParams {
            mode: BrotliEncoderMode::BROTLI_MODE_TEXT,
            quality: 11,
            ..Default::default()
        };
        BrotliCompress(&mut input, &mut compressed, &params)?;
        Ok(compressed)
    }

    pub fn deserialize(buf: Vec<u8>) -> Result<Self> {
        let mut input = Cursor::new(buf);
        let mut decompressed = vec![];
        BrotliDecompress(&mut input, &mut decompressed)?;
        let course_as_str = String::from_utf8(decompressed)?;
        Ok(ron::from_str(&course_as_str)?)
    }
}
