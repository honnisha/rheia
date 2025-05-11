use bevy::prelude::{Entity, EntityRef};
use common::chunks::block_position::BlockPositionTrait;
use network::{
    entities::EntityNetworkComponent,
    messages::{NetworkMessageType, ServerMessages},
};
use strum::IntoEnumIterator;

use crate::{
    entities::{
        entity::{Position, Rotation},
        entity_tag::EntityTagComponent,
        skin::EntitySkinComponent,
        traits::IEntityNetworkComponent,
        EntityComponent,
    },
    worlds::world_manager::{ChunkChanged, WorldManager},
};

use super::client_network::ClientNetwork;

pub(crate) fn send_start_streaming_entity(target_client: &ClientNetwork, entity_ref: EntityRef, world_slug: String) {
    let position = entity_ref.get::<Position>().unwrap();
    let rotation = entity_ref.get::<Rotation>().unwrap();
    let skin = entity_ref
        .get::<EntitySkinComponent>()
        .expect("skin is required for send_start_streaming_entity");

    let mut components: Vec<EntityNetworkComponent> = Default::default();

    for comp in EntityComponent::iter() {
        match comp {
            EntityComponent::Tag(_) => {
                if let Some(tag) = entity_ref.get::<EntityTagComponent>() {
                    components.push(tag.to_network());
                }
            }
            EntityComponent::Skin(_) => {
                components.push(skin.to_network());
            }
        }
    }

    let msg = ServerMessages::StartStreamingEntity {
        id: entity_ref.id().index(),
        world_slug: world_slug,
        position: position.to_network(),
        rotation: rotation.to_network(),
        components: components,
    };
    target_client.send_message(NetworkMessageType::ReliableOrdered, &msg);
}

/// Синхронизация спавна для всех наблюдателей чанка, в котором находится entity
/// Отправляет всем наблюдателям чанка StartStreamingEntity
///
/// Обязательно проверять, чтобы информация о игроке не отправилась ему же самому!
///
/// Only for entities with EntitySkin
pub(crate) fn sync_entity_spawn(world_manager: &WorldManager, entity: Entity) {
    let ecs = world_manager.get_ecs();
    let entity_ref = ecs.get_entity(entity).unwrap();
    let position = entity_ref.get::<Position>().unwrap();

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

            send_start_streaming_entity(&watcher_client, entity_ref, world_manager.get_slug().clone());
        }
    }
}

/// Обязательно проверять, чтобы информация о игроке не отправилась ему же самому!
///
/// Выполняет:
/// - Если объект не сменил чанк - отправляет всем наблюдателям чанка EntityMove
/// - Если переход между чанками:
///   • тем игрокам кто наблюдает и старый чанк и новый - отправлять EntityMove
///   • перешел из видимого чанка в невидимый - отправлять StopStreamingEntity
///   • перешел из невидимого чанка в видимый - отправлять StartStreamingEntity
///
/// Only for entities with EntitySkin
pub(crate) fn sync_entity_move(
    world_manager: &WorldManager,
    target_entity: Entity,
    chunks_changed: &Option<ChunkChanged>,
) {
    let ecs = world_manager.get_ecs();
    let entity_ref = ecs.get_entity(target_entity).unwrap();
    let position = entity_ref.get::<Position>().unwrap();
    let rotation = entity_ref.get::<Rotation>().unwrap();

    let move_msg = ServerMessages::EntityMove {
        world_slug: world_manager.get_slug().clone(),
        id: target_entity.index(),
        position: position.to_network(),
        rotation: rotation.to_network(),
    };
    let stop_msg = ServerMessages::StopStreamingEntities {
        world_slug: world_manager.get_slug().clone(),
        ids: vec![target_entity.index()],
    };

    match chunks_changed {
        None => {
            if let Some(entities) = world_manager
                .get_chunks_map()
                .get_chunk_watchers(&position.get_chunk_position())
            {
                for watcher_entity in entities {
                    if *watcher_entity == target_entity {
                        continue;
                    }

                    let watcher_entity_ref = ecs.get_entity(*watcher_entity).unwrap();
                    let watcher_client = watcher_entity_ref.get::<ClientNetwork>().unwrap();
                    watcher_client.send_message(NetworkMessageType::Unreliable, &move_msg);
                }
            }
        }
        Some(change) => {
            let mut old_watchers: &Vec<Entity> = &Default::default();
            if let Some(w) = world_manager.get_chunks_map().get_chunk_watchers(&change.old_chunk) {
                old_watchers = w;
            }

            let mut new_watchers: &Vec<Entity> = &Default::default();
            if let Some(w) = world_manager.get_chunks_map().get_chunk_watchers(&change.new_chunk) {
                new_watchers = w;
            }

            for old_watcher in old_watchers {
                if *old_watcher == target_entity {
                    continue;
                }

                let watcher_entity_ref = ecs.get_entity(*old_watcher).unwrap();
                let watcher_client = watcher_entity_ref.get::<ClientNetwork>().unwrap();

                // If watcher can see old and new chunk
                if new_watchers.contains(&old_watcher) {
                    watcher_client.send_message(NetworkMessageType::Unreliable, &move_msg);
                }
                // Player no longer can see entity
                else {
                    watcher_client.send_message(NetworkMessageType::ReliableOrdered, &stop_msg);
                }
            }

            for new_watcher in new_watchers {
                if *new_watcher == target_entity {
                    continue;
                }

                // New entity in range
                if !old_watchers.contains(&new_watcher) {
                    let watcher_entity_ref = ecs.get_entity(*new_watcher).unwrap();
                    let watcher_client = watcher_entity_ref.get::<ClientNetwork>().unwrap();

                    send_start_streaming_entity(&*watcher_client, entity_ref, world_manager.get_slug().clone());
                }
            }
        }
    }
}

/// Отправка всем наблюдателям чанка StopStreamingEntity
///
/// Only for entities with EntitySkin
pub fn sync_entity_despawn(world_manager: &WorldManager, entity: Entity) {
    let ecs = world_manager.get_ecs();
    let entity_ref = ecs.get_entity(entity).unwrap();
    let position = entity_ref.get::<Position>().unwrap();

    let stop_msg = ServerMessages::StopStreamingEntities {
        world_slug: world_manager.get_slug().clone(),
        ids: vec![entity.index()],
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

            watcher_client.send_message(NetworkMessageType::ReliableOrdered, &stop_msg);
        }
    }
}
