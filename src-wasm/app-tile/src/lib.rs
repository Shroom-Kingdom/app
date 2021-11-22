use app_config::{GROUND_FRICTION, RAPIER_SCALE};
use app_ground::{Ground, Grounds};
use bevy::{
    prelude::*, reflect::TypeUuid, render::camera::Camera, utils::HashSet, winit::WinitWindows,
};
use bevy_rapier::{na::Point2, prelude::*};
use web_sys::{HtmlCanvasElement, HtmlElement};
use winit::platform::web::WindowExtWebSys;

#[derive(Debug, TypeUuid)]
#[uuid = "89872da7-a8c0-4753-a2b7-cf3f84356d9d"]
pub struct Tile;

#[derive(Debug, TypeUuid)]
#[uuid = "7cafe334-3fc2-453a-846b-e8b14212592e"]
pub struct Tiles(HashSet<Entity>);

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, spawn_tile);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_tile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    winit_windows: Res<WinitWindows>,
    mut grounds: ResMut<Grounds>,
    camera_query: Query<(&Transform, &Camera)>,
) {
    let window = windows.get_primary().unwrap();
    let winit_window = winit_windows.get_window(window.id()).unwrap();
    let canvas = winit_window.canvas();
    if mouse_button_input.just_pressed(MouseButton::Left) {
        place_tile(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            window,
            &canvas,
            &mut grounds,
            &camera_query,
        );
    }
    if mouse_button_input.just_pressed(MouseButton::Right) {
        web_sys::console::log_1(&"PRESSED RIGHT".into());
    }
}

fn place_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    window: &Window,
    canvas: &HtmlCanvasElement,
    grounds: &mut ResMut<Grounds>,
    camera_query: &Query<(&Transform, &Camera)>,
) {
    let body = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();
    let cursor_position = if let Some(cursor_pointer) = window.cursor_position() {
        cursor_pointer
    } else {
        return;
    };

    let position = cursor_to_world(cursor_position, camera_query, &body, canvas);

    let texture_handle = asset_server.load("MW_Field_plain_0.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 16, 48);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let tile_size = 2.;

    commands
        .spawn_bundle(RigidBodyBundle {
            position: position.into(),
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .with_children(|parent| {
            let ground = parent
                .spawn_bundle(ColliderBundle {
                    collider_type: ColliderType::Sensor,
                    shape: ColliderShape::polyline(
                        vec![
                            Point2::new(-tile_size + 0.51, tile_size - 0.5),
                            Point2::new(tile_size - 0.51, tile_size - 0.5),
                        ],
                        None,
                    ),
                    flags: ActiveEvents::INTERSECTION_EVENTS.into(),
                    ..Default::default()
                })
                .insert(Ground)
                .insert(ColliderPositionSync::Discrete)
                .id();
            // TODO on entity despawn?
            grounds.insert(ground);
            parent.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    scale: Vec3::new(tile_size, tile_size, 0.),
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(6),
                ..Default::default()
            });
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(tile_size - 0.5, tile_size - 0.5),
            material: ColliderMaterial {
                friction: GROUND_FRICTION,
                friction_combine_rule: CoefficientCombineRule::Multiply,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}

fn cursor_to_world(
    cursor: Vec2,
    camera_query: &Query<(&Transform, &Camera)>,
    body: &HtmlElement,
    canvas: &HtmlCanvasElement,
) -> Vec2 {
    let diff = Vec2::new(
        0.,
        (body.offset_height() - canvas.offset_height() - 5) as f32,
    );
    let cursor =
        (cursor - diff) / Vec2::new(canvas.offset_width() as f32, canvas.offset_height() as f32);

    let (transform, camera) = camera_query.single().expect("main camera not found");

    let camera_position = transform.compute_matrix();
    let projection_matrix = camera.projection_matrix;

    // Normalized device coordinate cursor position from (-1, -1, -1) to (1, 1, 1)
    let cursor_ndc = (cursor) * 2.0 - Vec2::from([1.0, 1.0]);
    let cursor_pos_ndc_far = cursor_ndc.extend(1.0);

    let ndc_to_world = camera_position * projection_matrix.inverse();
    let cursor_pos_far = ndc_to_world.project_point3(cursor_pos_ndc_far);

    [
        cursor_pos_far.truncate().x / RAPIER_SCALE,
        cursor_pos_far.truncate().y / RAPIER_SCALE,
    ]
    .into()
}
