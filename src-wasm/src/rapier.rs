use crate::plugins::{DebugUiPlugin, DebugUiState};
use bevy::{prelude::*, render::pass::ClearColor};
use bevy_rapier::prelude::*;
use nalgebra::Isometry2;
use rapier::pipeline::PhysicsPipeline;
use wasm_bindgen::prelude::*;

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

#[wasm_bindgen]
pub fn main(debug_state: DebugState, set_debug_state: js_sys::Function) {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .insert_non_send_resource(DebugUiState {
            debug_state: debug_state.into(),
            set_debug_state,
        })
        .add_plugins(bevy_webgl2::DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(DebugUiPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_startup_system(enable_physics_profiling.system())
        .run();
}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

fn setup_graphics(mut commands: Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 10.0;

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 200.0, 0.0));
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(1000.0, 10.0, 2000.0)),
        point_light: PointLight {
            intensity: 100_000_000.0,
            range: 6000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(camera);
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 25.0;

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size, 1.0),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size * 2.0, 1.2),
        position: Isometry2::new(
            [ground_size, ground_size * 2.0].into(),
            std::f32::consts::FRAC_PI_2,
        )
        .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size * 2.0, 1.2),
        position: Isometry2::new(
            [-ground_size, ground_size * 2.0].into(),
            std::f32::consts::FRAC_PI_2,
        )
        .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);

    /*
     * Create the cubes
     */
    let num = 5;
    let rad = 0.5;

    let shift = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;
    let mut color = 0;

    for i in 0..num {
        for j in 0usize..num * 5 {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift + centery + 2.0;
            color += 1;

            // Build the rigid body.
            let body = RigidBodyBundle {
                position: [x, y].into(),
                ..Default::default()
            };
            let collider = ColliderBundle {
                shape: ColliderShape::cuboid(rad, rad),
                ..Default::default()
            };
            commands
                .spawn_bundle(body)
                .insert_bundle(collider)
                .insert(ColliderDebugRender::with_id(color))
                .insert(ColliderPositionSync::Discrete);
        }
    }
}
