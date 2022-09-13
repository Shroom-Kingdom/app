use app_config::*;
use bevy::{prelude::*, render::primitives::Frustum};

pub type MainCameraQuery<'w, 's, 'q> = Query<'w, 's, (&'q Transform, &'q Camera), With<Frustum>>;

#[inline]
pub fn pos_to_world(pos: i32) -> f32 {
    pos as f32 * GRID_SIZE * RAPIER_SCALE
}

#[inline]
pub fn grid_to_world(grid_pos: &[i32; 2]) -> Vec2 {
    [pos_to_world(grid_pos[0]), pos_to_world(grid_pos[1])].into()
}

#[inline]
pub fn grid_to_world_f32(grid_pos: &[f32; 2]) -> Vec2 {
    [
        grid_pos[0] * GRID_SIZE * RAPIER_SCALE,
        grid_pos[1] * GRID_SIZE * RAPIER_SCALE,
    ]
    .into()
}

#[inline]
pub fn world_to_grid_pos(pos: f32) -> i32 {
    (pos / (GRID_SIZE * RAPIER_SCALE)).round() as i32
}

#[inline]
pub fn world_to_grid(world_pos: &[f32; 2]) -> [i32; 2] {
    [
        world_to_grid_pos(world_pos[0]),
        world_to_grid_pos(world_pos[1]),
    ]
}

pub fn cursor_to_world(cursor: Vec2, camera_query: &MainCameraQuery, window: &Window) -> [f32; 2] {
    let cursor = cursor / Vec2::new(window.width(), window.height());

    let (transform, camera) = camera_query.single();

    let camera_position = transform.compute_matrix();
    let projection_matrix = camera.projection_matrix();

    let ndc = cursor * 2.0 - Vec2::ONE;
    let cursor_pos_ndc_far = ndc.extend(1.0);

    let ndc_to_world = camera_position * projection_matrix.inverse();
    let cursor_pos_far = ndc_to_world.project_point3(cursor_pos_ndc_far);

    [cursor_pos_far.truncate().x, cursor_pos_far.truncate().y]
}
