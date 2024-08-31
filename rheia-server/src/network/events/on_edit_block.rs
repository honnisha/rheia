use bevy::prelude::{Event, ResMut};
use bevy_ecs::prelude::EventReader;
use common::{blocks::block_info::BlockInfo, chunks::block_position::BlockPosition};

use crate::{network::{clients_container::ClientCell, sync_world_change::sync_world_block_change}, worlds::worlds_manager::WorldsManager};

#[derive(Event)]
pub struct EditBlockEvent {
    client: ClientCell,
    position: BlockPosition,
    new_block_info: BlockInfo,
}

impl EditBlockEvent {
    pub fn new(client: ClientCell, position: BlockPosition, new_block_info: BlockInfo) -> Self {
        Self {
            client,
            position,
            new_block_info,
        }
    }
}

pub fn on_edit_block(mut edit_block_events: EventReader<EditBlockEvent>, worlds_manager: ResMut<WorldsManager>) {
    for event in edit_block_events.read() {
        let client = event.client.read();

        let world_entity = client.get_world_entity();
        let world_entity = match world_entity.as_ref() {
            Some(w) => w,
            None => {
                log::error!(
                    target: "network",
                    "Client ip:{} tries to request edit block but he is not in the world!",
                    client
                );
                continue;
            }
        };

        let world_manager = worlds_manager
            .get_world_manager(&world_entity.get_world_slug())
            .unwrap();

        world_manager.edit_block(event.position.clone(), event.new_block_info.clone());
        sync_world_block_change(&*world_manager, event.position, event.new_block_info)
    }
}
