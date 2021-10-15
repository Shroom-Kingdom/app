use bevy::prelude::*;
use bevy_rapier::prelude::*;
use nalgebra::Isometry2;
use rapier::pipeline::PhysicsPipeline;

pub fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

pub fn setup_graphics(
    mut commands: Commands,
    mut configuration: ResMut<RapierConfiguration>,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    // let texture_handle = assets.load("MW_Player_MarioMdl_walk.1_0.png");
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // commands.spawn_bundle(SpriteBundle {
    //     material: materials.add(texture_handle.into()),
    //     ..Default::default()
    // });
}

pub fn setup_physics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
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

    let material = materials.add(assets.load("MW_Player_MarioMdl_walk.1_0.png").into());

    /*
     * Create the cubes
     */
    let num = 5;
    let rad = 0.5;

    let shift = rad * 4.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;
    let mut color = 0;

    let sprite_size_x = 4.0;
    let sprite_size_y = 8.0;
    rapier_config.scale = 20.0;
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    for i in 0..num {
        for j in 0usize..num * 5 {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift + centery + 4.0;
            color += 1;

            // Build the rigid body.
            let body = RigidBodyBundle {
                position: [x, y].into(),
                ..Default::default()
            };
            // let collider = ColliderBundle {
            //     shape: ColliderShape::cuboid(rad, rad),
            //     ..Default::default()
            // };
            commands
                .spawn_bundle(body)
                // .insert_bundle(collider)
                .insert_bundle(ColliderBundle {
                    // position: [collider_size_x / 2.0, collider_size_y / 2.0].into(),
                    shape: ColliderShape::cuboid(collider_size_x, collider_size_y),
                    ..Default::default()
                })
                .insert_bundle(SpriteBundle {
                    material: material.clone(),
                    sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
                    ..Default::default()
                })
                .insert(ColliderDebugRender::with_id(color))
                .insert(ColliderPositionSync::Discrete);
        }
    }
}
