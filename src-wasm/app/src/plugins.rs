use crate::rapier::DebugState;
use bevy::prelude::*;
use rapier::pipeline::PhysicsPipeline;
use wasm_bindgen::{JsCast, JsValue};

pub struct DebugUiPlugin;

struct DebugUiCount(u32);

pub struct DebugUiState {
    pub debug_state: JsValue,
    pub set_debug_state: js_sys::Function,
}

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Update, text_update_system.system())
            .insert_resource(DebugUiCount(0));
    }
}

fn text_update_system(
    pipeline: Res<PhysicsPipeline>,
    state: NonSend<DebugUiState>,
    mut count: ResMut<DebugUiCount>,
    // mut query: Query<&mut Text>,
) {
    if count.0 < 30 {
        count.0 += 1;
        return;
    }
    count.0 = 0;
    //     let profile_string = format!(
    //         r#"Total: {:.2}ms
    // Collision detection: {:.2}ms
    // |_ Broad-phase: {:.2}ms
    //    Narrow-phase: {:.2}ms
    // Island computation: {:.2}ms
    // Solver: {:.2}ms
    // |_ Velocity assembly: {:.2}ms
    //    Velocity resolution: {:.2}ms
    //    Velocity integration: {:.2}ms
    //    Position assembly: {:.2}ms
    //    Position resolution: {:.2}ms
    // CCD: {:.2}ms
    // |_ # of substeps: {}
    //    TOI computation: {:.2}ms
    //    Broad-phase: {:.2}ms
    //    Narrow-phase: {:.2}ms
    //    Solver: {:.2}ms"#,
    //         pipeline.counters.step_time(),
    //         pipeline.counters.collision_detection_time(),
    //         pipeline.counters.broad_phase_time(),
    //         pipeline.counters.narrow_phase_time(),
    //         pipeline.counters.island_construction_time(),
    //         pipeline.counters.solver_time(),
    //         pipeline.counters.solver.velocity_assembly_time.time(),
    //         pipeline.counters.velocity_resolution_time(),
    //         pipeline.counters.solver.velocity_update_time.time(),
    //         pipeline.counters.solver.position_assembly_time.time(),
    //         pipeline.counters.position_resolution_time(),
    //         pipeline.counters.ccd_time(),
    //         pipeline.counters.ccd.num_substeps,
    //         pipeline.counters.ccd.toi_computation_time.time(),
    //         pipeline.counters.ccd.broad_phase_time.time(),
    //         pipeline.counters.ccd.narrow_phase_time.time(),
    //         pipeline.counters.ccd.solver_time.time(),
    //     );
    let debug_state: &DebugState = state.debug_state.unchecked_ref();
    debug_state.set_step_time(pipeline.counters.step_time());
    debug_state.set_collision_detection_time(pipeline.counters.collision_detection_time());
    debug_state.set_broad_phase_time(pipeline.counters.broad_phase_time());
    debug_state.set_narrow_phase_time(pipeline.counters.narrow_phase_time());
    debug_state.set_island_construction_time(pipeline.counters.island_construction_time());

    let new_debug_state = js_sys::Object::new();
    js_sys::Object::assign(&new_debug_state, debug_state.unchecked_ref());
    state
        .set_debug_state
        .call1(&JsValue::NULL, &new_debug_state)
        .unwrap();

    // for mut text in query.iter_mut() {
    //     text.sections[0].value = profile_string.clone();
    // }
}
