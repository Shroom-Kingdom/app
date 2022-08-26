use bevy::prelude::*;

#[derive(Component)]
pub struct Draggable;

#[derive(Default)]
pub struct Dragging(Option<Entity>);
