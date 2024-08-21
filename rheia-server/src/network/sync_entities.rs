use bevy::prelude::{Entity, EntityRef, Event, EventReader};
use bevy_ecs::system::Res;
use common::{
    chunks::block_position::BlockPositionTrait,
    network::messages::{NetworkMessageType, ServerMessages},
};

use crate::{
    entities::entity::{NetworkComponent, Position, Rotation},
    worlds::{
        world_manager::{ChunkChanged, WorldManager},
        worlds_manager::WorldsManager,
    },
};

use super::client_network::{ClientNetwork, WorldEntity};

fn send_start_streaming_entity(target_client: &ClientNetwork, entity_ref: EntityRef, world_slug: String) {
    let position = entity_ref.get::<Position>().unwrap();
    let rotation = entity_ref.get::<Rotation>().unwrap();

    let msg = ServerMessages::StartStreamingEntity {
        id: entity_ref.id().index(),
        world_slug: world_slug,
        position: position.to_network(),
        rotation: rotation.to_network(),
    };
    target_client.send_message(NetworkMessageType::ReliableOrdered, msg);
}

/// Отправка всем наблюдателям чанка StartStreamingEntity
///
/// Обязательно проверять, чтобы информация о игроке не отправилась ему же самому!
pub fn sync_entity_spawn(world_manager: &WorldManager, entity: Entity) {
    let ecs = world_manager.get_ecs();
    let entity_ref = ecs.entity(entity);
    let position = entity_ref.get::<Position>().unwrap();

    if let Some(entities) = world_manager
        .get_chunks_map()
        .get_chunk_watchers(&position.get_chunk_position())
    {
        for watcher_entity in entities {
            if *watcher_entity == entity {
                continue;
            }

            let watcher_entity_ref = ecs.entity(*watcher_entity);
            let watcher_network = watcher_entity_ref.get::<NetworkComponent>().unwrap();
            let watcher_client = watcher_network.get_client();

            send_start_streaming_entity(&*watcher_client, entity_ref, world_manager.get_slug().clone());
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
pub fn sync_entity_move(world_manager: &WorldManager, entity: Entity, chunks_changed: &Option<ChunkChanged>) {
    let ecs = world_manager.get_ecs();
    let entity_ref = ecs.entity(entity);
    let position = entity_ref.get::<Position>().unwrap();
    let rotation = entity_ref.get::<Rotation>().unwrap();

    let move_msg = ServerMessages::EntityMove {
        world_slug: world_manager.get_slug().clone(),
        id: entity.index(),
        position: position.to_network(),
        rotation: rotation.to_network(),
    };
    let stop_msg = ServerMessages::StopStreamingEntities {
        world_slug: world_manager.get_slug().clone(),
        ids: vec![entity.index()],
    };

    match chunks_changed {
        None => {
            if let Some(entities) = world_manager
                .get_chunks_map()
                .get_chunk_watchers(&position.get_chunk_position())
            {
                for watcher_entity in entities {
                    if *watcher_entity == entity {
                        continue;
                    }

                    let watcher_entity_ref = ecs.entity(*watcher_entity);
                    let watcher_network = watcher_entity_ref.get::<NetworkComponent>().unwrap();
                    let watcher_client = watcher_network.get_client();
                    watcher_client.send_message(NetworkMessageType::Unreliable, move_msg.clone());
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
                if *old_watcher == entity {
                    continue;
                }

                let watcher_entity_ref = ecs.entity(*old_watcher);
                let watcher_network = watcher_entity_ref.get::<NetworkComponent>().unwrap();
                let watcher_client = watcher_network.get_client();

                // If watcher can see old and new chunk
                if new_watchers.contains(&old_watcher) {
                    watcher_client.send_message(NetworkMessageType::Unreliable, move_msg.clone());
                }
                // Player no longer can see entity
                else {
                    watcher_client.send_message(NetworkMessageType::ReliableOrdered, stop_msg.clone());
                }
            }

            for new_watcher in new_watchers {
                if *new_watcher == entity {
                    continue;
                }

                // New entity in range
                if !old_watchers.contains(&new_watcher) {
                    let watcher_entity_ref = ecs.entity(*new_watcher);
                    let watcher_network = watcher_entity_ref.get::<NetworkComponent>().unwrap();
                    let watcher_client = watcher_network.get_client();

                    send_start_streaming_entity(&*watcher_client, entity_ref, world_manager.get_slug().clone());
                }
            }
        }
    }
}

/// Отправка всем наблюдателям чанка StopStreamingEntity
pub fn sync_entity_despawn(_entity: Entity) {
}

#[derive(Event)]
pub struct PlayerSpawnEvent {
    world_entity: WorldEntity,
}

impl PlayerSpawnEvent {
    pub fn new(world_entity: WorldEntity) -> Self {
        Self { world_entity }
    }
}

/// Спавн игрока в мире
///
/// Вызывается при успешном подключении если чанк прогружен
/// или при прогрузке чанка, если игрок попал в загружающийся чанк.
///
/// Выполняет:
/// - Отправка игроку всех объектов в радиусе видимости из прогруженных чанков
/// - Вызыов синхронихации объекта игрока через `sync_entity_spawn`
pub fn sync_player_spawn(worlds_manager: Res<WorldsManager>, mut connection_events: EventReader<PlayerSpawnEvent>) {
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("sync_player_spawn").entered();

    for event in connection_events.read() {
        let world_manager = worlds_manager
            .get_world_manager(&event.world_entity.get_world_slug())
            .unwrap();

        let ecs = world_manager.get_ecs();
        let entity_ref = ecs.entity(event.world_entity.get_entity());
        let network = entity_ref.get::<NetworkComponent>().unwrap();
        let client = network.get_client();

        // Sends all existing entities from the player's line of sight
        if let Some(player_chunks) = world_manager
            .get_chunks_map()
            .get_watching_chunks(&event.world_entity.get_entity())
        {
            for chunk in player_chunks {
                if !world_manager.get_chunks_map().is_chunk_loaded(chunk) {
                    continue;
                }

                for entity_ref in world_manager.get_ecs().get_chunk_entities(&chunk).unwrap() {
                    // Prevents from sending spawn itself to the player
                    if entity_ref.id() == event.world_entity.get_entity() {
                        continue;
                    }

                    send_start_streaming_entity(&*client, entity_ref, world_manager.get_slug().clone());
                }
            }
        }

        sync_entity_spawn(&*world_manager, event.world_entity.get_entity());
    }
}

/// Передвижение игрока
///
/// Выполняет:
/// - Вызыов синхронихацию объекта игрока через on_entity_spawn
/// - Если игрок сменил чанк:
///   • отправлять StopStreamingEntity из старых чанков
///   • и StartStreamingEntity для новых
pub fn sync_player_move(
    world_manager: &WorldManager,
    world_entity: &WorldEntity,
    chunks_changed: &Option<ChunkChanged>,
) {
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("sync_player_move").entered();

    if let Some(change) = chunks_changed {
        let ecs = world_manager.get_ecs();
        let entity_ref = ecs.entity(world_entity.get_entity());
        let network = entity_ref.get::<NetworkComponent>().unwrap();
        let client = network.get_client();

        // Stop streaming entities from unseen chunks
        let mut ids: Vec<u32> = Default::default();
        for chunk in change.abandoned_chunks.iter() {
            for entity_ref in world_manager.get_ecs().get_chunk_entities(&chunk).unwrap() {
                ids.push(entity_ref.id().index());
            }
        }
        if ids.len() > 0 {
            let msg = ServerMessages::StopStreamingEntities {
                world_slug: world_entity.get_world_slug().clone(),
                ids: Default::default(),
            };
            client.send_message(NetworkMessageType::ReliableOrdered, msg);
        }

        // Start streaming entities from new chunks
        for chunk in change.new_chunks.iter() {
            for entity_ref in world_manager.get_ecs().get_chunk_entities(&chunk).unwrap() {
                if entity_ref.id() == world_entity.get_entity() {
                    continue;
                }
                send_start_streaming_entity(&*client, entity_ref, world_manager.get_slug().clone());
            }
        }
    }

    sync_entity_move(world_manager, world_entity.get_entity(), chunks_changed);
}
