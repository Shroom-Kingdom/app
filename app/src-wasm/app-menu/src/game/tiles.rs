use app_config::*;
use app_core::{GameModeEdit, SelectedTile, TileComponent, TilePlacePreview, TileSpriteHandles};
use bevy::{prelude::*, ui::FocusPolicy};
use shrm_core::{GroundVariant, TileVariant};

#[derive(Component)]
pub struct SelectedTileButton(pub bool);

pub struct SelectTileEvent(pub Entity);

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
                        image: UiImage($sprite_handles.0.get(&$tile.0).unwrap().clone()),
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

pub(crate) fn spawn_tile_buttons(commands: &mut Commands, tile_sprite_handles: &TileSpriteHandles) {
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
                TileComponent(TileVariant::Ground(GroundVariant::default())),
                true
            );
            add_tile_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                tile_sprite_handles,
                TileComponent(TileVariant::HardBlock),
                false
            );
            add_tile_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                tile_sprite_handles,
                TileComponent(TileVariant::RotatingBlock),
                false
            );
            add_tile_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                tile_sprite_handles,
                TileComponent(TileVariant::DonutBlock),
                false
            );
            add_tile_button!(
                parent,
                NORMAL_BUTTON_COLOR,
                tile_sprite_handles,
                TileComponent(TileVariant::CloudBlock),
                false
            );
        });
}

pub fn select_tile(
    mut query: Query<
        (
            Entity,
            &Interaction,
            &TileComponent,
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
            selected_tile.0 = Some(tile_variant.0.clone());
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
