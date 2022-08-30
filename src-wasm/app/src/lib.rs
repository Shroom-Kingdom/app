pub mod app;
pub mod graphics;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = setupPanicHook)]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once();
}
