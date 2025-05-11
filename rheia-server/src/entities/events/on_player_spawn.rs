use bevy::prelude::{EventReader, Res};

use crate::{
    entities::skin::EntitySkinComponent,
    network::{
        sync_entities::sync_entity_spawn,
        sync_players::{send_entities_for_player, PlayerSpawnEvent},
    },
    worlds::worlds_manager::WorldsManager,
};

/// Спавн игрока в мире
///
/// Вызывается при успешном подключении игрока если чанк прогружен
/// или при прогрузке чанка, если entity попал в загружающийся чанк.
pub(crate) fn on_player_spawn(
    worlds_manager: Res<WorldsManager>,
    mut connection_events: EventReader<PlayerSpawnEvent>,
) {
    #[cfg(feature = "trace")]
    let _span = bevy_utils::tracing::info_span!("sync_player_spawn").entered();

    for event in connection_events.read() {
        let target_entity = event.world_entity.get_entity();
        let world_manager = worlds_manager
            .get_world_manager(event.world_entity.get_world_slug())
            .unwrap();

        send_entities_for_player(&world_manager, target_entity);

        let ecs = world_manager.get_ecs();
        let entity_ref = ecs.get_entity(target_entity).unwrap();

        // Sync his entity if exists
        if entity_ref.get::<EntitySkinComponent>().is_some() {
            sync_entity_spawn(&*world_manager, target_entity);
        }
    }
}
