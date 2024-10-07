use common::blocks::block_type::{BlockContent, BlockType};

#[derive(Debug)]
pub struct TextureMapper {
    textures_map: Vec<String>,
}

impl Clone for TextureMapper {
    fn clone(&self) -> Self {
        TextureMapper {
            textures_map: self.textures_map.clone(),
        }
    }
}

impl TextureMapper {
    pub fn new() -> TextureMapper {
        TextureMapper {
            textures_map: Vec::new(),
        }
    }

    pub fn add_texture(&mut self, texture_name: String) -> Option<i64> {
        if self.textures_map.contains(&texture_name) {
            return None;
        };
        self.textures_map.push(texture_name);
        Some(self.textures_map.len() as i64 - 1_i64)
    }

    pub fn get_uv_offset(&self, block_type: &BlockType, side_index: i8) -> Option<usize> {
        match block_type.get_block_content() {
            BlockContent::Texture { texture, side_texture, bottom_texture } => {
                let texture_option = match side_index {
                    // Topside
                    4 => texture,
                    // Bottom
                    1 => match bottom_texture { Some(t) => t, None => texture },
                    // Sides
                    _ => match side_texture { Some(t) => t, None => texture },
                };
            },
            BlockContent::ModelCube { voxel_visibility, model } => return None,
        }
        let texture = match texture_option {
            Some(t) => t,
            None => {
                return None;
            }
        };

        self.textures_map.iter().position(|t| t == texture)
    }
}
