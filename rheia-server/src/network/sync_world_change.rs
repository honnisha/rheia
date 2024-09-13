use common::{
    blocks::block_info::BlockInfo,
    chunks::block_position::{BlockPosition, BlockPositionTrait},
};
use network::messages::{NetworkMessageType, ServerMessages};

use crate::{entities::entity::NetworkComponent, worlds::world_manager::WorldManager};

pub fn sync_world_block_change(world_manager: &WorldManager, position: BlockPosition, new_block_info: BlockInfo) {
    let ecs = world_manager.get_ecs();

    if let Some(entities) = world_manager
        .get_chunks_map()
        .get_chunk_watchers(&position.get_chunk_position())
    {
        for entity in entities {
            let entity_ref = ecs.get_entity(*entity).unwrap();
            let network = entity_ref.get::<NetworkComponent>().unwrap();
            let client = network.get_client();

            let msg = ServerMessages::EditBlock {
                world_slug: world_manager.get_slug().clone(),
                position: position.clone(),
                new_block_info: new_block_info.clone(),
            };
            client.send_message(NetworkMessageType::WorldInfo, msg);
        }
    }
}
