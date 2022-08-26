use app_config::*;
use app_core::{
    Course, GameMode, GameModeToggleEvent, GroundVariant, SelectedTile, TilePlacePreview,
    TileSpriteHandles, TileVariant, UiButtonSpriteHandles, UiButtonVariant,
};
use bevy::{prelude::*, ui::FocusPolicy};

macro_rules! add_tile_button {
    ( $parent:expr, $color:expr, $sprite_handles:expr, $tile:expr, $is_selected:expr ) => {
        $parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(48.), Val::Px(48.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: $color.into(),
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
                            margin: UiRect::all(Val::Auto),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(FocusPolicy::Pass);
            })
            .insert($tile)
            .insert(SelectedTileButton($is_selected));
    };
}

#[derive(Component)]
pub struct SelectedTileButton(pub bool);

pub struct SelectTileEvent(pub Entity);

#[derive(Component)]
pub struct GameModeEdit;

#[derive(Component)]
pub struct GameModeToggleButton {
    pub is_editing: bool,
}

#[derive(Component)]
pub struct GameModeToggleButtonImage;

pub fn setup_game_ui(
    mut commands: Commands,
    tile_sprite_handles: Res<TileSpriteHandles>,
    ui_button_sprite_handles: Res<UiButtonSpriteHandles>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect {
                    top: Val::Px(6.),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    right: Val::Auto,
                },
                padding: UiRect::all(Val::Px(6.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(GameModeEdit)
        .with_children(|parent| {
            add_tile_button!(
                parent,
                SELECTED_BUTTON_COLOR,
                tile_sprite_handles,
                TileVariant::Ground(GroundVariant::default()),
                true
            );
            add_tile_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                tile_sprite_handles,
                TileVariant::HardBlock,
                false
            );
            add_tile_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                tile_sprite_handles,
                TileVariant::RotatingBlock,
                false
            );
            add_tile_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                tile_sprite_handles,
                TileVariant::DonutBlock,
                false
            );
            add_tile_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                tile_sprite_handles,
                TileVariant::CloudBlock,
                false
            );
        });
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: UiRect {
                    top: Val::Px(6.),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    right: Val::Px(6.),
                },
                padding: UiRect::all(Val::Px(6.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(48.), Val::Px(48.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: NORMAL_BUTTON_COLOR.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ImageBundle {
                            image: UiImage(
                                ui_button_sprite_handles
                                    .0
                                    .get(&UiButtonVariant::GameModeSwitch { is_editing: true })
                                    .unwrap()
                                    .clone(),
                            ),
                            transform: Transform {
                                scale: Vec3::new(TILE_SIZE, TILE_SIZE, 0.),
                                ..Default::default()
                            },
                            style: Style {
                                margin: UiRect::all(Val::Auto),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(FocusPolicy::Pass)
                        .insert(GameModeToggleButtonImage);
                })
                .insert(GameModeToggleButton { is_editing: true });
        });
}

#[allow(clippy::type_complexity)]
pub fn select_tile(
    mut query: Query<
        (
            Entity,
            &Interaction,
            &TileVariant,
            &mut UiColor,
            &mut SelectedTileButton,
        ),
        Changed<Interaction>,
    >,
    mut selected_tile: ResMut<SelectedTile>,
    mut select_tile_event: EventWriter<SelectTileEvent>,
    mut tile_place_preview: ResMut<TilePlacePreview>,
    mut commands: Commands,
) {
    for (entity, interaction, tile_variant, mut color, mut is_selected) in query.iter_mut() {
        if *interaction == Interaction::Clicked {
            selected_tile.0 = Some(tile_variant.clone());
            *color = SELECTED_BUTTON_COLOR.into();
            is_selected.0 = true;
            select_tile_event.send(SelectTileEvent(entity));

            if let Some((entity, _)) = tile_place_preview.0 {
                commands.entity(entity).despawn_recursive();
                tile_place_preview.0 = None;
            }
        }
    }
}

pub fn change_after_tile_select(
    mut query: Query<(Entity, &Interaction, &mut SelectedTileButton, &mut UiColor)>,
    mut select_tile_event: EventReader<SelectTileEvent>,
) {
    for SelectTileEvent(selected_entity) in select_tile_event.iter() {
        for (entity, interaction, mut tile_button, mut color) in query.iter_mut() {
            if tile_button.0 && selected_entity != &entity {
                match interaction {
                    Interaction::Hovered => {
                        *color = HOVERED_BUTTON_COLOR.into();
                    }
                    Interaction::None => {
                        *color = NORMAL_BUTTON_COLOR.into();
                    }
                    _ => {}
                }
                tile_button.0 = false;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn toggle_game_mode(
    mut query: Query<(&Interaction, &mut GameModeToggleButton), Changed<Interaction>>,
    mut game_mode_edit_query: Query<&mut Style, With<GameModeEdit>>,
    mut game_mode_button_query: Query<&mut UiImage, With<GameModeToggleButtonImage>>,
    mut course: ResMut<Course>,
    mut game_mode_toggle_event: EventWriter<GameModeToggleEvent>,
    ui_button_sprite_handles: Res<UiButtonSpriteHandles>,
    mut tile_place_preview: ResMut<TilePlacePreview>,
    mut commands: Commands,
) {
    for (interaction, mut button) in query.iter_mut() {
        if *interaction == Interaction::Clicked {
            let is_editing = !button.is_editing;
            button.is_editing = is_editing;
            course.game_mode = GameMode::Build { is_editing };
            let mut game_mode_button = game_mode_button_query.single_mut();
            *game_mode_button = UiImage(
                ui_button_sprite_handles
                    .0
                    .get(&UiButtonVariant::GameModeSwitch { is_editing })
                    .unwrap()
                    .clone(),
            );
            game_mode_toggle_event.send(GameModeToggleEvent { is_editing });

            for mut style in game_mode_edit_query.iter_mut() {
                style.display = if is_editing {
                    Display::Flex
                } else {
                    Display::None
                };
            }

            if let Some((entity, _)) = tile_place_preview.0 {
                commands.entity(entity).despawn_recursive();
                tile_place_preview.0 = None;
            }
        }
    }
}
