use bevy::prelude::{Mut, World};
use bevy_ecs::system::Command;
use common::chunks::block_position::BlockPositionTrait;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::{
    entities::{
        entity::{Position, Rotation},
        skin::EntitySkin,
    },
    network::{
        client_network::ClientNetwork,
        sync_entities::{sync_entity_despawn, sync_entity_spawn, sync_update_entity_skin},
        sync_players::PlayerSpawnEvent,
    },
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

            // Send world creation message
            let spawn_world = ServerMessages::SpawnWorld {
                world_slug: self.world_slug.clone(),
            };
            self.client
                .send_message(NetworkMessageType::ReliableOrdered, &spawn_world);

            // Send teleport instructions
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

pub struct UpdatePlayerSkin {
    client: ClientNetwork,
    skin: Option<EntitySkin>,
}

impl UpdatePlayerSkin {
    pub fn create(client: ClientNetwork, skin: Option<EntitySkin>) -> Self {
        Self { client, skin }
    }
}

impl Command for UpdatePlayerSkin {
    fn apply(self, world: &mut World) {
        world.resource_scope(|_world, worlds_manager: Mut<WorldsManager>| {
            let Some(world_entity) = self.client.get_world_entity() else {
                panic!("UpdatePlayerSkin: player not in the world");
            };
            let Some(mut world_manager) = worlds_manager.get_world_manager_mut(world_entity.get_world_slug()) else {
                panic!(
                    "UpdatePlayerSkin: world \"{}\" doesn't exists",
                    world_entity.get_world_slug()
                );
            };

            let ecs = world_manager.get_ecs_mut();

            let mut entity = ecs.entity_mut(world_entity.get_entity());
            let old_skin = entity.get_mut::<EntitySkin>();

            match self.skin.as_ref() {
                Some(new_skin) => {
                    match old_skin {
                        Some(mut old_skin) => {
                            // Replace the old skin
                            *old_skin = new_skin.clone();
                            sync_update_entity_skin(&*world_manager, world_entity.get_entity());
                        }
                        None => {
                            // Create the new skin
                            entity.insert(new_skin.clone());
                            sync_entity_spawn(&*world_manager, world_entity.get_entity());
                        }
                    }
                }
                None => {
                    // Remove the old skin
                    if old_skin.is_some() {
                        entity.remove::<EntitySkin>();
                    }
                    sync_entity_despawn(&*world_manager, world_entity.get_entity());
                }
            }

            // Send world creation message
            let skin_message = ServerMessages::UpdatePlayerSkin {
                skin: match self.skin.as_ref() {
                    Some(s) => Some(s.to_network().clone()),
                    None => None,
                },
            };
            self.client
                .send_message(NetworkMessageType::ReliableOrdered, &skin_message);
        });
    }
}
