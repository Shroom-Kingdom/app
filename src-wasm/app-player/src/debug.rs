use bevy::prelude::*;
// use bevy_rapier::prelude::*;

// use crate::Player;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![TextSection {
                value: "asdasdasd".to_string(),
                style: TextStyle {
                    font_size: 15.0,
                    color: Color::BLACK,
                    font,
                },
            }],
            ..Default::default()
        },
        ..Default::default()
    });
}

// pub fn text_update_system(
//     mut query: Query<&mut Text>,
//     query_player: Query<(&Player, &RigidBodyVelocity)>,
// ) {
//     if let Ok((_, rb_vel)) = query_player.single() {
//         let profile_string = format!(
//             r#"x: {:.2}ms
// y: {:.2}ms"#,
//             rb_vel.linvel.data.0[0][0], rb_vel.linvel.data.0[0][1],
//         );
//         // web_sys::console::log_1(&profile_string.clone().into());

//         for mut text in query.iter_mut() {
//             text.sections[0].value = profile_string.clone();
//         }
//     }
// }
