use crate::{ThemeVariant, TileVariant};
use anyhow::Result;
use brotli::{
    enc::{backward_references::BrotliEncoderMode, writer::CompressorWriter, BrotliEncoderParams},
    DecompressorWriter,
};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::io::Write;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Course {
    pub tiles: HashMap<[i32; 2], TileVariant>,
    pub theme: ThemeVariant,
    pub goal_pos_x: i32,
}

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

#[wasm_bindgen(js_name = isCourse)]
pub fn is_course(buf: Vec<u8>) -> bool {
    Course::deserialize(buf).is_ok()
}
