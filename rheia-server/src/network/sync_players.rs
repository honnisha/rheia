use bevy::prelude::{Entity, Event};
use network::messages::{NetworkMessageType, ServerMessages};

use crate::{
    entities::skin::EntitySkinComponent,
    worlds::world_manager::{ChunkChanged, WorldManager},
};

use super::{
    client_network::{ClientNetwork, WorldEntity},
    sync_entities::{send_start_streaming_entity, sync_entity_move},
};

#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub world_entity: WorldEntity,
}

impl PlayerSpawnEvent {
    pub fn new(world_entity: WorldEntity) -> Self {
        Self { world_entity }
    }
}

/// Выполняет:
/// - Отправка игроку всех объектов в радиусе видимости из прогруженных чанков
pub fn send_entities_for_player(world_manager: &WorldManager, target_entity: Entity) {
    let ecs = world_manager.get_ecs();
    let entity_ref = ecs.get_entity(target_entity).unwrap();
    if let Some(client) = entity_ref.get::<ClientNetwork>() {
        // Sends all existing entities from the player's line of sight
        if let Some(player_chunks) = world_manager.get_chunks_map().get_watching_chunks(&target_entity) {
            for chunk in player_chunks {
                if !world_manager.get_chunks_map().is_chunk_loaded(chunk) {
                    continue;
                }

                for target_ref in world_manager.get_ecs().get_chunk_entities(&chunk).unwrap() {
                    // Prevents from sending spawn itself to the player
                    if target_ref.id() == target_entity {
                        continue;
                    }

                    if target_ref.get::<EntitySkinComponent>().is_some() {
                        send_start_streaming_entity(&*client, target_ref, world_manager.get_slug().clone());
                    }
                }
            }
        }
    }
}

/// Передвижение игрока
///
/// Выполняет:
///   • Вызыов синхронихации объекта игрока через sync_entity_move
///
/// - вызывает для ClientNetwork:
///   • отправлять ему StopStreamingEntity из старых чанков
///   • и StartStreamingEntity для новых
pub fn sync_player_move(world_manager: &WorldManager, target_entity: Entity, chunks_changed: &Option<ChunkChanged>) {
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("sync_player_move").entered();

    let ecs = world_manager.get_ecs();
    let entity_ref = ecs.get_entity(target_entity).unwrap();

    if let Some(change) = chunks_changed {
        let client = entity_ref.get::<ClientNetwork>().unwrap();

        // Stop streaming entities from unseen chunks
        let mut ids: Vec<u32> = Default::default();
        for chunk in change.abandoned_chunks.iter() {
            for entity_ref in world_manager.get_ecs().get_chunk_entities(&chunk).unwrap() {
                ids.push(entity_ref.id().index());
            }
        }
        if ids.len() > 0 {
            let msg = ServerMessages::StopStreamingEntities {
                world_slug: world_manager.get_slug().clone(),
                ids: Default::default(),
            };
            client.send_message(NetworkMessageType::ReliableOrdered, &msg);
        }

        // Start streaming entities from new chunks
        for chunk in change.new_chunks.iter() {
            for target_ref in world_manager.get_ecs().get_chunk_entities(&chunk).unwrap() {
                if target_ref.id() == target_entity {
                    continue;
                }
                if target_ref.get::<EntitySkinComponent>().is_some() {
                    send_start_streaming_entity(&*client, target_ref, world_manager.get_slug().clone());
                }
            }
        }
    }

    // Sync his entity if exists
    if entity_ref.get::<EntitySkinComponent>().is_some() {
        sync_entity_move(world_manager, target_entity, chunks_changed);
    }
}
