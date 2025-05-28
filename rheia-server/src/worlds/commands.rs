use bevy::prelude::{Mut, World};
use bevy_ecs::system::Command;
use common::chunks::block_position::BlockPositionTrait;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::{
    entities::{
        EntityComponent,
        entity::{Position, Rotation},
    },
    network::{client_network::ClientNetwork, sync_players::PlayerSpawnEvent},
};

use super::worlds_manager::WorldsManager;

pub struct SpawnPlayer {
    world_slug: String,
    client: ClientNetwork,
    position: Position,
    rotation: Rotation,
    components: Vec<EntityComponent>,
}

impl SpawnPlayer {
    pub fn create(
        world_slug: String,
        client: ClientNetwork,
        position: Position,
        rotation: Rotation,
        components: Vec<EntityComponent>,
    ) -> Self {
        Self {
            world_slug,
            client,
            position,
            rotation,
            components,
        }
    }
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, worlds_manager: Mut<WorldsManager>| {
            let Some(mut world_manager) = worlds_manager.get_world_manager_mut(&self.world_slug) else {
                panic!("SpawnPlayer: world \"{}\" doesn't exists", self.world_slug);
            };

            let bundle = (self.position.clone(), self.rotation, self.client.clone());
            let world_entity = world_manager.spawn_player(self.position, bundle, self.components.clone());

            self.client.set_world_entity(Some(world_entity.clone()));

            // Send world creation message
            let spawn_world = ServerMessages::SpawnWorld {
                world_slug: self.world_slug.clone(),
            };
            self.client
                .send_message(NetworkMessageType::ReliableOrdered, &spawn_world);

            self.client
                .network_send_spawn(&self.position, &self.rotation, &self.components);

            if world_manager
                .get_chunks_map()
                .is_chunk_loaded(&self.position.get_chunk_position())
            {
                world.send_event(PlayerSpawnEvent::new(world_entity.clone())).unwrap();
            }
        });
    }
}
