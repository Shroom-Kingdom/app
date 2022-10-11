#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::Player;
// use app_config::RAPIER_SCALE;
use bevy::prelude::*;
use bevy_rapier::{prelude::*, rapier::math::Isometry};

pub struct StoopEvent {
    pub is_stooping: bool,
}

// TODO unsupported by physics engine for now
pub fn stoop(
    mut query: Query<(&Player, &mut Children)>,
    // mut child_query: Query<(&mut Collider, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut stoop_events: EventWriter<StoopEvent>,
    // mut context: ResMut<RapierContext>,
) {
    if let Ok((player, children)) = query.get_single_mut() {
        // let child = children.get(1).unwrap();
        // let (mut collider, mut transform) = child_query.get_mut(*child).unwrap();
        let stooped = !player.state.is_stooping
            && (keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down));
        let unstooped = player.state.is_stooping
            && !keyboard_input.pressed(KeyCode::S)
            && !keyboard_input.pressed(KeyCode::Down);

        if stooped {
            // let shape = collider.raw.make_mut().as_round_cuboid_mut().unwrap();
            // web_sys::console::log_1(&format!("TRANSFORM {:?}", transform.translation).into());
            // shape.inner_shape.half_extents.data.0[0][1] = 9.68;
            // collider.promote_scaled_shape();

            // transform.translation.y = (9.68 - 15.2) / 2.;
            stoop_events.send(StoopEvent { is_stooping: true });
        } else if unstooped {
            // let shape = collider.raw.make_mut().as_round_cuboid_mut().unwrap();
            // web_sys::console::log_1(&format!("TRANSFORM {:?}", transform.translation).into());
            // shape.inner_shape.half_extents.data.0[0][1] = 15.2;
            // collider.promote_scaled_shape();

            // transform.translation.y = 0.;
            stoop_events.send(StoopEvent { is_stooping: false });
        }
    }
}
