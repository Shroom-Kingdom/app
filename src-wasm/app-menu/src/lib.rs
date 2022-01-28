mod game;

use app_core::{is_done_insert_course, AppLabel, AppState};
use bevy::prelude::*;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct MainMenuBuildButton;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(exit_menu))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(is_done_insert_course)
                    .after(AppLabel::InsertCourse)
                    .with_system(game::setup_game_ui),
            )
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(start_game))
            .add_system(on_hover);
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

#[allow(clippy::type_complexity)]
fn on_hover(mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>) {
    for (interaction, mut color) in query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            _ => {}
        }
    }
}

#[allow(clippy::type_complexity)]
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
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(MainMenu)
        .insert(MainMenuBuildButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Build",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
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
