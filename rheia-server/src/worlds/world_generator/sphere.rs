use common::{
    blocks::{block_info::BlockInfo, blocks_storage::BlockType},
    chunks::{block_position::ChunkBlockPosition, chunk_position::ChunkPosition},
    CHUNK_SIZE,
};
use network::messages::ChunkDataType;

#[derive(Default)]
pub struct SphereWorldGenerator {}

impl SphereWorldGenerator {
    pub fn new(_seed: u64) -> Self {
        Self {}
    }

    pub fn generate_chunk_data(
        &self,
        chunk_data: &mut ChunkDataType,
        _chunk_position: &ChunkPosition,
        _vertical_index: usize,
    ) -> bool {
        let center = CHUNK_SIZE as f32 / 2.0;

        for x in 0_u8..(CHUNK_SIZE as u8) {
            for z in 0_u8..(CHUNK_SIZE as u8) {
                for y in 0_u8..(CHUNK_SIZE as u8) {
                    let dx = x as f32 - center;
                    let dy = y as f32 - center;
                    let dz = z as f32 - center;
                    let d = (dx * dx + dy * dy + dz * dz).sqrt();

                    if d < 10.0 {
                        let pos = ChunkBlockPosition::new(x, y, z);
                        chunk_data.insert(pos, BlockInfo::new(BlockType::GrassBlock));
                    };
                }
            }
        }
        return true;
    }
}
