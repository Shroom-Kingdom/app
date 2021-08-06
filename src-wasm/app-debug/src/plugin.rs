use crate::DebugState;
use bevy::prelude::*;
use rapier::pipeline::PhysicsPipeline;
use wasm_bindgen::{JsCast, JsValue};

pub struct DebugPlugin;

struct DebugPluginCount(u32);

pub struct DebugPluginState {
    pub debug_state: JsValue,
    pub set_debug_state: js_sys::Function,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Update, text_update_system.system())
            .insert_resource(DebugPluginCount(0));
    }
}

fn text_update_system(
    pipeline: Res<PhysicsPipeline>,
    state: NonSend<DebugPluginState>,
    mut count: ResMut<DebugPluginCount>,
) {
    if count.0 < 30 {
        count.0 += 1;
        return;
    }
    count.0 = 0;
    let debug_state: &DebugState = state.debug_state.unchecked_ref();
    debug_state.set_step_time(pipeline.counters.step_time());
    debug_state.set_collision_detection_time(pipeline.counters.collision_detection_time());
    debug_state.set_broad_phase_time(pipeline.counters.broad_phase_time());
    debug_state.set_narrow_phase_time(pipeline.counters.narrow_phase_time());
    debug_state.set_island_construction_time(pipeline.counters.island_construction_time());
    debug_state.set_solver_time(pipeline.counters.solver_time());
    debug_state.set_velocity_assembly_time(pipeline.counters.solver.velocity_assembly_time.time());
    debug_state.set_velocity_resolution_time(pipeline.counters.velocity_resolution_time());
    debug_state.set_velocity_update_time(pipeline.counters.solver.velocity_update_time.time());
    debug_state.set_position_assembly_time(pipeline.counters.solver.position_assembly_time.time());
    debug_state.set_position_resolution_time(pipeline.counters.position_resolution_time());
    debug_state.set_ccd_time(pipeline.counters.ccd_time());
    debug_state.set_num_substeps(pipeline.counters.ccd.num_substeps);
    debug_state.set_toi_computation_time(pipeline.counters.ccd.toi_computation_time.time());
    debug_state.set_ccd_broad_phase_time(pipeline.counters.ccd.broad_phase_time.time());
    debug_state.set_ccd_narrow_phase_time(pipeline.counters.ccd.narrow_phase_time.time());
    debug_state.set_ccd_solver_time(pipeline.counters.ccd.solver_time.time());

    let new_debug_state = js_sys::Object::new();
    js_sys::Object::assign(&new_debug_state, debug_state.unchecked_ref());
    state
        .set_debug_state
        .call1(&JsValue::NULL, &new_debug_state)
        .unwrap();
}
