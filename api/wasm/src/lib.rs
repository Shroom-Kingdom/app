use shrm_core::Course;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = isCourse)]
pub fn is_course(buf: Vec<u8>) -> bool {
    Course::deserialize(buf).is_ok()
}
