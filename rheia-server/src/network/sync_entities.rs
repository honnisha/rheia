use bevy::prelude::{Entity, EntityRef, Event, EventReader};
use bevy_ecs::system::Res;
use common::{
    chunks::{block_position::BlockPositionTrait, chunk_position::ChunkPosition},
    network::messages::{NetworkMessageType, ServerMessages},
};

use crate::{
    entities::entity::{NetworkComponent, Position, Rotation},
    worlds::{world_manager::WorldManager, worlds_manager::WorldsManager},
};

use super::client_network::WorldEntity;

fn send_start_streaming_entity(entity_ref: EntityRef, world_slug: String) {
    let position = entity_ref.get::<Position>().unwrap();
    let rotation = entity_ref.get::<Rotation>().unwrap();

    let network = entity_ref.get::<NetworkComponent>().unwrap();
    let client = network.get_client();

    let msg = ServerMessages::StartStreamingEntity {
        id: entity_ref.id().index(),
        world_slug: world_slug,
        position: position.to_network(),
        rotation: rotation.to_network(),
    };
    client.send_message(NetworkMessageType::ReliableOrdered, msg);
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
        for _e in entities {}
    }
}

/// Отправка всем наблюдателям чанка StopStreamingEntity
pub fn sync_entity_despawn(_entity: Entity) {}

/// Обязательно проверять, чтобы информация о игроке не отправилась ему же самому!
///
/// Выполняет:
/// - Если объект не сменил чанк - отправляет всем наблюдателям чанка EntityMove
/// - Если переход между чанками:
///   • тем игрокам кто наблюдает и старый чанк и новый - отправлять EntityMove
///   • перешел из видимого чанка в невидимый - отправлять StopStreamingEntity
///   • перешел из невидимого чанка в видимый - отправлять StartStreamingEntity
pub fn sync_entity_move(_entity: Entity, _chunks_changed: &Option<(Vec<ChunkPosition>, Vec<ChunkPosition>)>) {}

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

                    send_start_streaming_entity(entity_ref, world_manager.get_slug().clone());
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
    chunks_changed: &Option<(Vec<ChunkPosition>, Vec<ChunkPosition>)>,
) {
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("sync_player_move").entered();

    if let Some((abandoned_chunks, new_chunks)) = chunks_changed {
        let ecs = world_manager.get_ecs();
        let entity_ref = ecs.entity(world_entity.get_entity());
        let network = entity_ref.get::<NetworkComponent>().unwrap();
        let client = network.get_client();

        let mut ids: Vec<u32> = Default::default();
        for chunk in abandoned_chunks {
            for entity in world_manager.get_ecs().get_chunk_entities(&chunk).unwrap() {
                ids.push(entity.id().index());
            }
        }
        if ids.len() > 0 {
            let msg = ServerMessages::StopStreamingEntities {
                world_slug: world_entity.get_world_slug().clone(),
                ids: Default::default(),
            };
            client.send_message(NetworkMessageType::ReliableOrdered, msg);
        }

        for chunk in new_chunks {
            for entity_ref in world_manager.get_ecs().get_chunk_entities(&chunk).unwrap() {
                if entity_ref.id() == world_entity.get_entity() {
                    continue;
                }
                send_start_streaming_entity(entity_ref, world_manager.get_slug().clone());
            }
        }
    }

    sync_entity_move(world_entity.get_entity(), chunks_changed);
}
