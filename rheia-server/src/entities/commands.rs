use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::Command,
    world::{Mut, World},
};
use common::chunks::block_position::BlockPositionTrait;
use network::messages::{NetworkMessageType, ServerMessages};

use crate::{
    network::{
        client_network::ClientNetwork,
        sync_entities::{sync_entity_despawn, sync_entity_spawn},
    },
    worlds::{world_manager::WorldManager, worlds_manager::WorldsManager},
};

use super::{
    entity::Position, entity_tag::EntityTagComponent, skin::EntitySkinComponent, traits::IEntityNetworkComponent,
    EntityComponent,
};

pub struct UpdatePlayerComponent {
    client: ClientNetwork,
    updated_component: EntityComponent,
}

impl UpdatePlayerComponent {
    pub fn create(client: ClientNetwork, updated_component: EntityComponent) -> Self {
        Self {
            client,
            updated_component,
        }
    }
}

impl Command for UpdatePlayerComponent {
    fn apply(self, world: &mut World) {
        world.resource_scope(|_world, worlds_manager: Mut<WorldsManager>| {
            let Some(world_entity) = self.client.get_world_entity() else {
                panic!("UpdatePlayerComponent: player not in the world");
            };
            let Some(mut world_manager) = worlds_manager.get_world_manager_mut(world_entity.get_world_slug()) else {
                panic!(
                    "UpdatePlayerComponent: world \"{}\" doesn't exists",
                    world_entity.get_world_slug()
                );
            };

            let ecs = world_manager.get_ecs_mut();
            let mut entity = ecs.entity_mut(world_entity.get_entity());

            let mut is_send_to_player = false;
            match &self.updated_component {
                EntityComponent::Tag(tag) => {
                    match tag {
                        Some(new_tag) => match entity.get_mut::<EntityTagComponent>() {
                            Some(mut old) => *old = new_tag.clone(),
                            None => {
                                entity.insert(new_tag.clone());
                            }
                        },
                        None => {
                            entity.remove::<EntityTagComponent>();
                        }
                    }
                    sync_update_entity_component::<EntityTagComponent>(&*world_manager, world_entity.get_entity())
                }
                EntityComponent::Skin(entity_skin) => {
                    is_send_to_player = true;
                    let old_skin = entity.get_mut::<EntitySkinComponent>();

                    match entity_skin.as_ref() {
                        Some(new_skin) => {
                            match old_skin {
                                Some(mut old_skin) => {
                                    // Replace the old skin
                                    *old_skin = new_skin.clone();
                                    sync_update_entity_component::<EntitySkinComponent>(
                                        &*world_manager,
                                        world_entity.get_entity(),
                                    );
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
                                entity.remove::<EntitySkinComponent>();
                            }
                            sync_entity_despawn(&*world_manager, world_entity.get_entity());
                        }
                    }
                }
            }

            if is_send_to_player {
                let comp_message = ServerMessages::UpdatePlayerComponent {
                    component: self.updated_component.to_network(),
                };
                self.client
                    .send_message(NetworkMessageType::ReliableOrdered, &comp_message);
            }
        });
    }
}

/// Sync entity component for all watchers
pub(crate) fn sync_update_entity_component<T: Component + IEntityNetworkComponent>(
    world_manager: &WorldManager,
    entity: Entity,
) {
    let ecs = world_manager.get_ecs();
    let entity_ref = ecs.get_entity(entity).unwrap();
    let position = entity_ref.get::<Position>().unwrap();

    let component = match entity_ref.get::<T>() {
        Some(c) => c.to_network(),
        None => T::get_empty(),
    };

    let msg = ServerMessages::UpdateEntityComponent {
        world_slug: world_manager.get_slug().clone(),
        id: entity_ref.id().index(),
        component: component,
    };

    if let Some(entities) = world_manager
        .get_chunks_map()
        .get_chunk_watchers(&position.get_chunk_position())
    {
        for watcher_entity in entities {
            if *watcher_entity == entity {
                continue;
            }

            let watcher_entity_ref = ecs.get_entity(*watcher_entity).unwrap();
            let watcher_client = watcher_entity_ref.get::<ClientNetwork>().unwrap();

            watcher_client.send_message(NetworkMessageType::ReliableOrdered, &msg);
        }
    }
}
