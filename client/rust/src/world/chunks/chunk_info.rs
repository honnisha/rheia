use godot::prelude::Vector3;
use ndshape::ConstShape;

use super::block_info::BlockInfo;
use crate::utils::mesh::mesh_generator::ChunkShape;

pub struct ChunkInfo {
    chunk_data: [BlockInfo; 4096],
}

impl ChunkInfo {
    pub fn new(chunk_data: [BlockInfo; 4096]) -> Self {
        ChunkInfo {
            chunk_data: chunk_data,
        }
    }

    pub fn get_chunk_data(&self) -> &[BlockInfo; 4096] {
        &self.chunk_data
    }

    pub fn set_block_by_local_pos(&mut self, pos: u32, block_info: BlockInfo) {
        self.chunk_data[pos as usize] = block_info;
    }

    pub fn set_block(&mut self, global_pos: &[i32; 3], block_info: BlockInfo) {
        let local_pos = ChunkInfo::get_chunk_local_pos_from_global(global_pos);
        let i = ChunkShape::linearize(local_pos) as usize;
        self.chunk_data[i] = block_info;
    }

    // Get global position from chunk coordinate
    pub fn get_chunk_pos_from_coordinate(position: &[i32; 3]) -> Vector3 {
        // -1 because of chunk boundaries
        Vector3::new(
            position[0] as f32 * 16.0 - 1_f32,
            position[1] as f32 * 16.0 - 1_f32,
            position[2] as f32 * 16.0 - 1_f32,
        )
    }

    fn fix_chunk_loc_pos(p: i32) -> i32 {
        if p < 0 {
            return (p + 1_i32) / 16_i32 + -1_i32;
        }
        return p / 16_i32;
    }
    /// Return chunk position from global coordinate
    pub fn get_chunk_pos_by_global(p: &[i32; 3]) -> [i32; 3] {
        [
            ChunkInfo::fix_chunk_loc_pos(p[0]),
            ChunkInfo::fix_chunk_loc_pos(p[1]),
            ChunkInfo::fix_chunk_loc_pos(p[2]),
        ]
    }

    fn fix_loc_pos(p: i32) -> u32 {
        if p < 0 {
            return (15_i32 + ((p + 1_i32) % 16_i32)) as u32;
        }
        return (p % 16_i32) as u32;
    }
    /// Return chunk local position
    /// by global coordinate
    pub fn get_chunk_local_pos_from_global(p: &[i32; 3]) -> [u32; 3] {
        [
            ChunkInfo::fix_loc_pos(p[0]),
            ChunkInfo::fix_loc_pos(p[1]),
            ChunkInfo::fix_loc_pos(p[2]),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::world::chunks::chunk_info::ChunkInfo;

    #[test]
    fn test_get_chunk_pos_by_global() {
        assert_eq!(
            ChunkInfo::get_chunk_pos_by_global(&[0_i32, 1_i32, 20_i32]),
            [0_i32, 0_i32, 1_i32]
        );
        assert_eq!(
            ChunkInfo::get_chunk_pos_by_global(&[-15_i32, -16_i32, -17_i32]),
            [-1_i32, -1_i32, -2_i32]
        );
        assert_eq!(
            ChunkInfo::get_chunk_pos_by_global(&[33_i32, -1_i32, -20_i32]),
            [2_i32, -1_i32, -2_i32]
        );
    }

    #[test]
    fn test_get_chunk_local_pos_from_global() {
        assert_eq!(
            ChunkInfo::get_chunk_local_pos_from_global(&[0_i32, 1_i32, 20_i32]),
            [0_u32, 1_u32, 4_u32]
        );
        assert_eq!(
            ChunkInfo::get_chunk_local_pos_from_global(&[0_i32, -1_i32, -2_i32]),
            [0_u32, 15_u32, 14_u32]
        );
        assert_eq!(
            ChunkInfo::get_chunk_local_pos_from_global(&[-15_i32, -16_i32, -17_i32]),
            [1_u32, 0_u32, 15_u32]
        );
    }
}

impl AsRef<ChunkInfo> for ChunkInfo {
    fn as_ref(&self) -> &Self {
        self
    }
}
impl AsMut<ChunkInfo> for ChunkInfo {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
