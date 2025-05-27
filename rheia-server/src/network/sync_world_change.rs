use common::chunks::{
    block_position::{BlockPosition, BlockPositionTrait},
    chunk_data::BlockDataInfo,
};
use network::messages::{NetworkMessageType, ServerMessages};

use crate::worlds::world_manager::WorldManager;

use super::client_network::ClientNetwork;

pub fn sync_world_block_change(
    world_manager: &WorldManager,
    position: BlockPosition,
    new_block_info: Option<BlockDataInfo>,
) {
    let ecs = world_manager.get_ecs();

    if let Some(entities) = world_manager
        .get_chunks_map()
        .get_chunk_watchers(&position.get_chunk_position())
    {
        for entity in entities {
            let entity_ref = ecs.get_entity(*entity).unwrap();
            let network = entity_ref.get::<ClientNetwork>().unwrap();

            let msg = ServerMessages::EditBlock {
                world_slug: world_manager.get_slug().clone(),
                position: position.clone(),
                new_block_info: new_block_info.clone(),
            };
            network.send_message(NetworkMessageType::WorldInfo, &msg);
        }
    }
}
