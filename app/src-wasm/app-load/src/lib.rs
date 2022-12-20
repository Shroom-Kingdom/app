use app_core::{
    AppState, CourseLoading, CourseRes, GroundTileUpdateEvent, ObjectSpriteHandles, Tile,
};
use bevy::prelude::*;
use shrm_core::TileVariant;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadingFinished>()
            .add_system_set(SystemSet::on_update(AppState::Load).with_system(check_load))
            .add_system_set(SystemSet::on_exit(AppState::Load).with_system(update_ground_tiles));
    }
}

struct LoadingFinished;

#[allow(clippy::too_many_arguments)]
fn check_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    object_sprite_handles: Res<ObjectSpriteHandles>,
    mut ground_tile_update_events: EventWriter<GroundTileUpdateEvent>,
    course_loading: ResMut<CourseLoading>,
    mut loading_finished: EventWriter<LoadingFinished>,
    mut state: ResMut<State<AppState>>,
) {
    if let Some(course) = &*course_loading.0.read().unwrap() {
        let course = CourseRes::load(
            &mut commands,
            course,
            &asset_server,
            &mut texture_atlases,
            &object_sprite_handles,
            &mut ground_tile_update_events,
        );
        commands.insert_resource(course);
        loading_finished.send(LoadingFinished);
        state.set(AppState::Game).unwrap();
    }
}

fn update_ground_tiles(
    mut query: Query<&mut TextureAtlasSprite>,
    mut child_query: Query<&Children>,
    course: Res<CourseRes>,
) {
    for Tile {
        entity, variant, ..
    } in course.tiles.values()
    {
        if let TileVariant::Ground(ground_variant) = variant {
            if let Ok(children) = child_query.get_mut(*entity) {
                let child = children[1];
                let mut sprite = query.get_mut(child).unwrap();
                *sprite = TextureAtlasSprite::new(ground_variant.get_sprite_sheet_index());
            }
        }
    }
}
