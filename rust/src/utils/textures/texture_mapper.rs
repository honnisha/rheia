use crate::world::blocks::block_type_info::BlockTypeInfo;


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

    pub fn get_uv_offset(&self, block_type_info: &'static BlockTypeInfo, side_index: i8) -> Option<usize> {
        let texture_option = match side_index {
            // Topside
            4 => block_type_info.top_texture,
            // Bottom
            1 => block_type_info.bottom_texture,
            // Sides
            _ => block_type_info.side_texture,
        };
        let texture = match texture_option {
            Some(t) => t,
            None => { return None; }
        };

        self.textures_map.iter().position(|t| t == texture)
    }
}
