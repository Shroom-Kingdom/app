use bevy::prelude::*;
use bevy_rapier::{
    na::Vector2,
    physics::{ColliderBundle, ColliderPositionSync, RapierConfiguration, RigidBodyBundle},
    prelude::{ColliderShape, RigidBodyMassProps, RigidBodyMassPropsFlags, RigidBodyVelocity},
    render::ColliderDebugRender,
};

pub fn setup_character(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.scale = 10.0;

    let material = materials.add(assets.load("MW_Player_MarioMdl_walk.1_0.png").into());
    let body = RigidBodyBundle {
        position: [0., 20.].into(),
        mass_properties: RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            ..Default::default()
        },
        ..Default::default()
    };
    let sprite_size_x = 4.0;
    let sprite_size_y = 5.6;
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;
    commands
        .spawn_bundle(body)
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_x, collider_size_y),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            material,
            sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
            ..Default::default()
        })
        .insert(Player {})
        .insert(ColliderDebugRender::with_id(0))
        .insert(ColliderPositionSync::Discrete);
}

#[derive(Debug)]
pub struct Player;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut player_info: Query<(&Player, &mut RigidBodyVelocity, &RigidBodyMassProps)>,
) {
    for (_, mut rb_vels, rb_mprops) in player_info.iter_mut() {
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;

        let mut move_delta = Vector2::new(x_axis as f32, 0.);
        if move_delta != Vector2::zeros() {
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        rb_vels.apply_impulse(rb_mprops, move_delta * 5.);
    }
}
