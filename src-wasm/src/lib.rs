mod plugins;
pub mod rapier;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = setupPanicHook)]
pub fn setup_panic_hook() {
    console_error_panic_hook::set_once();
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
