use crate::Player;
use bevy::prelude::*;
use bevy_rapier::prelude::*;

#[derive(Debug)]
pub struct TouchEvent(Entity);

pub fn touch(
    mut query: Query<Entity, With<Player>>,
    mut touch_events: EventWriter<TouchEvent>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    if let Ok(entity) = query.get_single_mut() {
        for contact_event in contact_events.iter() {
            if let CollisionEvent::Started(collider_entity1, collider_entity2, _) = contact_event {
                if collider_entity1 == &entity || collider_entity2 == &entity {
                    touch_events.send(TouchEvent(entity));
                }
            }
        }
    }
}
