use app_config::*;
use app_core::GameModeToggleEvent;
use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Grid;

pub fn setup_grid(mut commands: Commands) {
    for x in 0..MAX_COURSE_X {
        for y in 0..MAX_COURSE_Y {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.25, 0.25, 0.25, 0.7),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            x as f32 * GRID_SIZE * RAPIER_SCALE + GRID_SIZE * RAPIER_SCALE / 2.,
                            y as f32 * GRID_SIZE * RAPIER_SCALE + GRID_SIZE * RAPIER_SCALE / 2.,
                            Z_INDEX_GRID,
                        ),
                        ..default()
                    },
                    ..default()
                },
                Grid,
            ));
        }
    }
}

pub(crate) fn toggle_grid(
    mut query: Query<&mut Visibility, With<Grid>>,
    mut game_mode_toggle_event: EventReader<GameModeToggleEvent>,
) {
    for event in game_mode_toggle_event.iter() {
        for mut visibility in query.iter_mut() {
            visibility.is_visible = event.is_editing;
        }
    }
}
