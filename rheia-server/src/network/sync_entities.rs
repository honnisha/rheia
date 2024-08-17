use bevy::prelude::{Entity, Event, EventReader};
use bevy_ecs::system::Res;

use crate::worlds::worlds_manager::WorldsManager;

use super::{client_network::WorldEntity, clients_container::ClientsContainer, server::NetworkContainer};

/// Отправка всем наблюдателям чанка StartStreamingEntity
///
/// Обязательно проверять, чтобы информация о игроке не отправилась ему же самому!
pub fn sync_entity_spawn(_entity: Entity) {}

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
pub fn sync_entity_move(_entity: Entity) {}

#[derive(Event)]
pub struct PlayerSpawnEvent {
    world_entity: WorldEntity
}

impl PlayerSpawnEvent {
    pub fn new(world_entity: WorldEntity) -> Self {
        Self {
            world_entity,
        }
    }
}

/// Спавн игрока в мире
///
/// Вызывается при успешном подключении если чанк прогружен
/// или при прогрузке чанка, если игрок попал в загружающийся чанк.
///
/// Выполняет:
/// - Отправка игроку всех объектов в радиусе видимости из прогруженных чанков
/// - Вызыов синхронихацию объекта игрока через on_entity_spawn
pub fn sync_player_spawn(
    worlds_manager: Res<WorldsManager>,
    network_container: Res<NetworkContainer>,
    clients: Res<ClientsContainer>,
    mut connection_events: EventReader<PlayerSpawnEvent>,
) {
    for event in connection_events.read() {
        log::info!("sync_player_spawn: {}", event.world_entity.get_entity());

        sync_entity_spawn(event.world_entity.get_entity());
    }
}

/// Передвижение игрока
///
/// Выполняет:
/// - Вызыов синхронихацию объекта игрока через on_entity_spawn
/// - Если игрок сменил чанк:
///   • отправлять StopStreamingEntity из старых чанков
///   • и StartStreamingEntity для новых
pub fn sync_player_move(world_entity: WorldEntity) {
    sync_entity_move(world_entity.get_entity());
}
