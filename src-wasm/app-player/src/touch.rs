use crate::Player;
use bevy::prelude::*;
use bevy_rapier::prelude::*;

#[derive(Debug)]
pub struct TouchEvent(Entity);

pub fn touch(
    mut query: Query<Entity, With<Player>>,
    mut touch_events: EventWriter<TouchEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    if let Ok(entity) = query.get_single_mut() {
        for contact_event in contact_events.iter() {
            if let ContactEvent::Started(collider1, collider2) = contact_event {
                if collider1.entity() == entity || collider2.entity() == entity {
                    touch_events.send(TouchEvent(entity));
                }
            }
        }
    }
}
