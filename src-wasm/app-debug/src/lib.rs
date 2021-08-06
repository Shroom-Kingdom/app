mod plugin;

use wasm_bindgen::prelude::*;

pub use plugin::{DebugPlugin, DebugPluginState};

#[wasm_bindgen]
extern "C" {
    pub type DebugState;

    #[wasm_bindgen(method, getter, js_name = stepTime)]
    pub fn step_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = stepTime)]
    pub fn set_step_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = collisionDetectionTime)]
    pub fn collision_detection_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = collisionDetectionTime)]
    pub fn set_collision_detection_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = broadPhaseTime)]
    pub fn broad_phase_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = broadPhaseTime)]
    pub fn set_broad_phase_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = narrowPhaseTime)]
    pub fn narrow_phase_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = narrowPhaseTime)]
    pub fn set_narrow_phase_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = islandConstructionTime)]
    pub fn island_construction_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = islandConstructionTime)]
    pub fn set_island_construction_time(this: &DebugState, val: f64);

    // pipeline.counters.solver_time(),
    // pipeline.counters.solver.velocity_assembly_time.time(),
    // pipeline.counters.velocity_resolution_time(),
    // pipeline.counters.solver.velocity_update_time.time(),
    // pipeline.counters.solver.position_assembly_time.time(),
    // pipeline.counters.position_resolution_time(),
    // pipeline.counters.ccd_time(),
    // pipeline.counters.ccd.num_substeps,
    // pipeline.counters.ccd.toi_computation_time.time(),
    // pipeline.counters.ccd.broad_phase_time.time(),
    // pipeline.counters.ccd.narrow_phase_time.time(),
    // pipeline.counters.ccd.solver_time.time(),
}
