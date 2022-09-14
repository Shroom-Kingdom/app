mod game;

use app_config::*;
use app_core::{AppLabel, AppStage, AppState, SelectedTile, TileVariant};
use bevy::prelude::*;
use game::{change_after_tile_select, select_tile, toggle_game_mode, SelectTileEvent};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct MainMenuBuildButton;

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

fn start_game(
    mut query: Query<&Interaction, (Changed<Interaction>, With<MainMenuBuildButton>)>,
    mut state: ResMut<State<AppState>>,
) {
    for interaction in query.iter_mut() {
        if *interaction == Interaction::Clicked {
            state.set(AppState::Game).unwrap();
        }
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON_COLOR.into(),
            ..Default::default()
        })
        .insert(MainMenu)
        .insert(MainMenuBuildButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Build",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                ..Default::default()
            });
        });
}

fn exit_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
