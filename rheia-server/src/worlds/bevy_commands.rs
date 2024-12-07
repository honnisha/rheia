use bevy::prelude::{Mut, World};
use bevy_ecs::world::Command;
use common::chunks::block_position::BlockPositionTrait;

use crate::{
    entities::entity::{Position, Rotation},
    network::{client_network::ClientNetwork, sync_entities::PlayerSpawnEvent},
};

use super::worlds_manager::WorldsManager;

pub struct SpawnPlayer {
    world_slug: String,
    client: ClientNetwork,
    position: Position,
    rotation: Rotation,
}

impl SpawnPlayer {
    pub fn create(world_slug: String, client: ClientNetwork, position: Position, rotation: Rotation) -> Self {
        Self {
            world_slug,
            client,
            position,
            rotation,
        }
    }
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, worlds_manager: Mut<WorldsManager>| {
            let Some(mut world_manager) = worlds_manager.get_world_manager_mut(&self.world_slug) else {
                panic!("SpawnPlayer: world \"{}\" doesn't exists", self.world_slug);
            };

            let world_entity = world_manager.spawn_player(self.client.clone(), self.position, self.rotation);

            self.client.set_world_entity(Some(world_entity.clone()));
            self.client.network_send_teleport(&self.position, &self.rotation);

            if world_manager
                .get_chunks_map()
                .is_chunk_loaded(&self.position.get_chunk_position())
            {
                world.send_event(PlayerSpawnEvent::new(world_entity.clone())).unwrap();
            }
        });
    }
}
