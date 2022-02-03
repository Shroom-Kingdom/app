use crate::NORMAL_BUTTON;
use app_config::TILE_SIZE;
use app_core::{CourseSpriteHandles, DoneInsertCourse, SelectedTile, TileVariant};
use bevy::{prelude::*, ui::FocusPolicy};

macro_rules! add_button {
    ( $parent:expr, $sprite_handles:expr, $tile:expr ) => {
        $parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(48.), Val::Px(48.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: NORMAL_BUTTON.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(ImageBundle {
                        image: UiImage($sprite_handles.0.get(&$tile).unwrap().clone()),
                        transform: Transform {
                            scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.),
                            ..Default::default()
                        },
                        style: Style {
                            margin: Rect::all(Val::Auto),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(FocusPolicy::Pass);
            })
            .insert($tile);
    };
}

pub fn setup_game_ui(
    mut commands: Commands,
    mut done: ResMut<DoneInsertCourse>,
    sprite_handles: Res<CourseSpriteHandles>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect {
                    top: Val::Px(6.),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    right: Val::Auto,
                },
                padding: Rect::all(Val::Px(6.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            add_button!(parent, sprite_handles, TileVariant::Ground);
            add_button!(parent, sprite_handles, TileVariant::HardBlock);
            add_button!(parent, sprite_handles, TileVariant::RotatingBlock);
            add_button!(parent, sprite_handles, TileVariant::DonutBlock);
            add_button!(parent, sprite_handles, TileVariant::CloudBlock);
        });

    done.0 = false;
}

#[allow(clippy::type_complexity)]
pub fn select_tile(
    mut query: Query<(&Interaction, &TileVariant), (Changed<Interaction>, With<TileVariant>)>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    for (interaction, tile_variant) in query.iter_mut() {
        if *interaction == Interaction::Clicked {
            selected_tile.0 = tile_variant.clone();
        }
    }
}
