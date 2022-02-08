use app_config::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, SELECTED_BUTTON_COLOR, TILE_SIZE};
use app_core::{CourseSpriteHandles, GroundVariant, SelectedTile, TileVariant};
use bevy::{prelude::*, ui::FocusPolicy};

macro_rules! add_button {
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
                            margin: Rect::all(Val::Auto),
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

pub fn setup_game_ui(mut commands: Commands, sprite_handles: Res<CourseSpriteHandles>) {
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
            add_button!(
                parent,
                SELECTED_BUTTON_COLOR,
                sprite_handles,
                TileVariant::Ground(GroundVariant::default()),
                true
            );
            add_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                sprite_handles,
                TileVariant::HardBlock,
                false
            );
            add_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                sprite_handles,
                TileVariant::RotatingBlock,
                false
            );
            add_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                sprite_handles,
                TileVariant::DonutBlock,
                false
            );
            add_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                sprite_handles,
                TileVariant::CloudBlock,
                false
            );
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
) {
    for (entity, interaction, tile_variant, mut color, mut is_selected) in query.iter_mut() {
        if *interaction == Interaction::Clicked {
            selected_tile.0 = Some(tile_variant.clone());
            *color = SELECTED_BUTTON_COLOR.into();
            is_selected.0 = true;
            select_tile_event.send(SelectTileEvent(entity));
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
