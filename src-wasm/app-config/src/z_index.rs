// somehow Bevy can only handle negative z-indices,
// so the item on the very top would need a z-index of 0.

pub const Z_INDEX_PLAYER: f32 = -0.4;
pub const Z_INDEX_TILE: f32 = -0.2;

pub const Z_INDEX_GOAL_L: f32 = -0.8;
pub const Z_INDEX_GOAL: f32 = -0.5;
pub const Z_INDEX_GOAL_R: f32 = -0.3;
pub const Z_INDEX_GOAL_DRAG: f32 = -0.1;

pub const Z_INDEX_BACKGROUND: f32 = -1.;
pub const Z_INDEX_GRID: f32 = -0.05;
