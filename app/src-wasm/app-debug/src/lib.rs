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

    #[wasm_bindgen(method, getter, js_name = solverTime)]
    pub fn solver_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = solverTime)]
    pub fn set_solver_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = velocityAssemblyTime)]
    pub fn velocity_assembly_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = velocityAssemblyTime)]
    pub fn set_velocity_assembly_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = velocityResolutionTime)]
    pub fn velocity_resolution_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = velocityResolutionTime)]
    pub fn set_velocity_resolution_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = velocityUpdateTime)]
    pub fn velocity_update_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = velocityUpdateTime)]
    pub fn set_velocity_update_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = positionAssemblyTime)]
    pub fn position_assembly_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = positionAssemblyTime)]
    pub fn set_position_assembly_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = positionResolutionTime)]
    pub fn position_resolution_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = positionResolutionTime)]
    pub fn set_position_resolution_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = ccdTime)]
    pub fn ccd_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = ccdTime)]
    pub fn set_ccd_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = numSubsteps)]
    pub fn num_substeps(this: &DebugState) -> usize;

    #[wasm_bindgen(method, setter, js_name = numSubsteps)]
    pub fn set_num_substeps(this: &DebugState, val: usize);

    #[wasm_bindgen(method, getter, js_name = toiComputationTime)]
    pub fn toi_computation_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = toiComputationTime)]
    pub fn set_toi_computation_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = ccdBroadPhaseTime)]
    pub fn ccd_broad_phase_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = ccdBroadPhaseTime)]
    pub fn set_ccd_broad_phase_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = ccdNarrowPhaseTime)]
    pub fn ccd_narrow_phase_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = ccdNarrowPhaseTime)]
    pub fn set_ccd_narrow_phase_time(this: &DebugState, val: f64);

    #[wasm_bindgen(method, getter, js_name = ccdSolverTime)]
    pub fn ccd_solver_time(this: &DebugState) -> f64;

    #[wasm_bindgen(method, setter, js_name = ccdSolverTime)]
    pub fn set_ccd_solver_time(this: &DebugState, val: f64);
}
