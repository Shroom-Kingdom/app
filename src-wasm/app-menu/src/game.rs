pub(crate) mod tiles;

use crate::GameModeEdit;
use app_config::*;
use app_core::{
    Course, CourseRes, GameMode, GameModeToggleEvent, TilePlacePreview, TileSpriteHandles,
    UiButtonSpriteHandles, UiButtonVariant,
};
use bevy::{prelude::*, ui::FocusPolicy};
use js_sys::{Array, Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Blob, HtmlElement, MouseEvent, Url};

#[derive(Component)]
pub struct GameModeToggleButton {
    pub is_editing: bool,
}

#[derive(Component)]

pub struct ExportButton;

#[derive(Component)]
pub struct GameModeToggleButtonImage;

pub fn setup_game_ui(
    mut commands: Commands,
    tile_sprite_handles: Res<TileSpriteHandles>,
    ui_button_sprite_handles: Res<UiButtonSpriteHandles>,
) {
    tiles::spawn_tile_buttons(&mut commands, &tile_sprite_handles);

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
            spawn_export_button(parent, &ui_button_sprite_handles);
            spawn_game_mode_toggle_button(parent, &ui_button_sprite_handles);
        });
}

fn spawn_export_button(
    parent: &mut ChildBuilder,
    ui_button_sprite_handles: &UiButtonSpriteHandles,
) {
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
                            .get(&UiButtonVariant::Export)
                            .unwrap()
                            .clone(),
                    ),
                    transform: Transform {
                        scale: Vec3::new(0.6, 0.6, 0.),
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
        .insert(ExportButton);
}

fn spawn_game_mode_toggle_button(
    parent: &mut ChildBuilder,
    ui_button_sprite_handles: &UiButtonSpriteHandles,
) {
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
}

pub fn export(
    mut query: Query<&Interaction, (With<ExportButton>, Changed<Interaction>)>,
    course: Res<CourseRes>,
) {
    for interaction in query.iter_mut() {
        if *interaction == Interaction::Clicked {
            let course: Course = course.as_ref().into();
            let data = course.serialize().unwrap();
            let js_data = unsafe { Uint8Array::view(&data[..]) };

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let anchor = document.create_element("a").unwrap();
            let blob = Blob::new_with_u8_array_sequence(&Array::of1(&js_data)).unwrap();
            let obj_url = Url::create_object_url_with_blob(&blob).unwrap();
            anchor.set_attribute("href", &obj_url).unwrap();
            anchor.set_attribute("download", "course.ron.br").unwrap();
            anchor
                .set_attribute("style", "{\"display\": \"none\"}")
                .unwrap();

            // TODO enable weakrefs to prevent memory leak
            // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html#method.into_js_value
            let click_closure = Closure::<dyn FnMut(MouseEvent)>::new(|event: MouseEvent| {
                event.stop_propagation();
            });
            anchor
                .add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())
                .unwrap();
            click_closure.forget();
            document.body().unwrap().append_child(&anchor).unwrap();
            anchor.unchecked_ref::<HtmlElement>().click();
            
            let closure = Closure::<dyn FnMut()>::new(move || {
                let body = web_sys::window().unwrap().document().unwrap().body();
                body.unwrap().remove_child(&anchor).unwrap();
                Url::revoke_object_url(&anchor.get_attribute("href").unwrap()).unwrap();
            });
            window
                .set_timeout_with_callback(closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn toggle_game_mode(
    mut query: Query<(&Interaction, &mut GameModeToggleButton), Changed<Interaction>>,
    mut game_mode_edit_query: Query<&mut Style, With<GameModeEdit>>,
    mut game_mode_button_query: Query<&mut UiImage, With<GameModeToggleButtonImage>>,
    mut game_mode: ResMut<GameMode>,
    mut game_mode_toggle_event: EventWriter<GameModeToggleEvent>,
    ui_button_sprite_handles: Res<UiButtonSpriteHandles>,
    mut tile_place_preview: ResMut<TilePlacePreview>,
    mut commands: Commands,
) {
    for (interaction, mut button) in query.iter_mut() {
        if *interaction == Interaction::Clicked {
            let is_editing = !button.is_editing;
            button.is_editing = is_editing;
            *game_mode = GameMode::Build { is_editing };
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
