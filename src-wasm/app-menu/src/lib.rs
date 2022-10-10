mod game;

pub(crate) use game::tiles::SelectTileEvent;

use app_config::*;
use app_core::{
    AppLabel, AppStage, AppState, Course, CourseLoading, CourseRes, GroundTileUpdateEvent,
    ObjectSpriteHandles, SelectedTile, ThemeVariant, TileVariant, UiButtonSpriteHandles,
    UiButtonVariant,
};
use bevy::{prelude::*, ui::FocusPolicy};
use game::{
    export,
    tiles::{change_after_tile_select, select_tile},
    toggle_game_mode,
};
use js_sys::{ArrayBuffer, Uint8Array};
use std::sync::{Arc, RwLock};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, HtmlInputElement};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct MainMenuBuildButton;

#[derive(Component)]
struct MainMenuImportButton;

#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuLabel {
    SelectTile,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectTileEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(exit_menu))
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                    .after(AppLabel::InsertCourse)
                    .with_system(game::setup_game_ui),
            )
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(start_game))
            .add_system_set_to_stage(
                AppStage::PlayerInput,
                SystemSet::on_update(AppState::Game)
                    .with_system(select_tile.label(MenuLabel::SelectTile))
                    .with_system(change_after_tile_select.after(MenuLabel::SelectTile))
                    .with_system(export)
                    .with_system(toggle_game_mode),
            )
            .add_system(on_hover);
    }
}

#[allow(clippy::type_complexity)]
fn on_hover(
    mut query: Query<
        (&Interaction, &mut UiColor, Option<&TileVariant>),
        (Changed<Interaction>, With<Button>),
    >,
    selected_tile: Res<SelectedTile>,
) {
    for (interaction, mut color, tile_variant) in query.iter_mut() {
        if let Some(selected_tile) = &selected_tile.0 {
            if let Some(tile_variant) = tile_variant {
                if tile_variant == selected_tile {
                    *color = SELECTED_BUTTON_COLOR.into();
                    continue;
                }
            }
        }
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
            }
            _ => {}
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(unused_must_use)]
fn start_game(
    mut commands: Commands,
    build_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuBuildButton>)>,
    import_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuImportButton>)>,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    object_sprite_handles: Res<ObjectSpriteHandles>,
    mut ground_tile_update_events: EventWriter<GroundTileUpdateEvent>,
    course_loading: ResMut<Arc<RwLock<CourseLoading>>>,
) {
    if let Some(&Interaction::Clicked) = build_query.into_iter().next() {
        state.set(AppState::Game).unwrap();
        let course = CourseRes::empty(
            &mut commands,
            ThemeVariant::Plain,
            &asset_server,
            &mut texture_atlases,
            &object_sprite_handles,
            &mut ground_tile_update_events,
        );
        commands.insert_resource(course);
    }
    if let Some(&Interaction::Clicked) = import_query.into_iter().next() {
        state.set(AppState::Load).unwrap();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let file_input: HtmlInputElement =
            document.create_element("input").unwrap().unchecked_into();
        file_input.set_attribute("type", "file").unwrap();
        file_input
            .set_attribute("style", "{\"display\": \"none\"}")
            .unwrap();
        let this = file_input.clone();

        let course_res = course_loading.clone();
        // TODO enable weakrefs to prevent memory leak
        // https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html#method.into_js_value
        let input_closure = Closure::once(move || {
            if let Some(files) = this.files() {
                if files.length() == 0 {
                    return;
                }
                let file = files.item(0).unwrap();
                let read_closure = Closure::once(move |val: JsValue| {
                    let buffer: ArrayBuffer = val.unchecked_into();
                    let buffer = Uint8Array::new(&buffer);
                    let buffer = buffer.to_vec();

                    let course = Course::deserialize(buffer).unwrap();

                    course_res.write().unwrap().0 = Some(course);
                });

                // This is ok, because JS Promises don't need to be awaited
                #[allow(unused_must_use)]
                {
                    file.array_buffer().then(&read_closure);
                }
                read_closure.forget();
            }
        });
        file_input
            .add_event_listener_with_callback("change", input_closure.as_ref().unchecked_ref())
            .unwrap();
        document.body().unwrap().append_child(&file_input).unwrap();
        file_input.unchecked_ref::<HtmlElement>().click();
        input_closure.forget();

        let closure = Closure::<dyn FnMut()>::new(move || {
            let body = web_sys::window().unwrap().document().unwrap().body();
            body.unwrap().remove_child(&file_input).unwrap();
        });
        window
            .set_timeout_with_callback(closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }
}

macro_rules! add_menu_button {
    ( $text:expr, $component: expr, $button_variant:expr, $commands:expr, $button_handles:expr, $asset_server:expr ) => {
        $commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(220.0), Val::Px(65.0)),
                    margin: UiRect::all(Val::Auto),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: NORMAL_BUTTON_COLOR.into(),
                ..Default::default()
            })
            .insert(MainMenu)
            .insert($component)
            .with_children(|parent| {
                parent
                    .spawn_bundle(ImageBundle {
                        image: UiImage($button_handles.0.get(&$button_variant).unwrap().clone()),
                        transform: Transform {
                            scale: Vec3::new(0.6, 0.6, 0.),
                            ..Default::default()
                        },
                        style: Style {
                            margin: UiRect {
                                left: Val::Px(8.),
                                right: Val::Px(16.),
                                top: Val::Auto,
                                bottom: Val::Auto,
                            },
                            max_size: Size {
                                width: Val::Px(65.0),
                                height: Val::Px(65.0),
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(FocusPolicy::Pass);
                parent.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        $text,
                        TextStyle {
                            font: $asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ),
                    ..Default::default()
                });
            });
    };
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_button_sprite_handles: Res<UiButtonSpriteHandles>,
) {
    add_menu_button!(
        "Build",
        MainMenuBuildButton,
        UiButtonVariant::Build,
        commands,
        ui_button_sprite_handles,
        asset_server
    );
    add_menu_button!(
        "Import",
        MainMenuImportButton,
        UiButtonVariant::Import,
        commands,
        ui_button_sprite_handles,
        asset_server
    );
}

fn exit_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
