use crate::blocks::block_type_info::BlockTypeInfo;


pub struct TextureMapper {
    textures_map: Vec<String>,
}

impl TextureMapper {
    pub fn new() -> TextureMapper {
        TextureMapper {
            textures_map: Vec::new(),
        }
    }

    pub fn add_texture(&mut self, texture_name: String) -> i64 {
        self.textures_map.push(texture_name);
        self.textures_map.len() as i64 - 1_i64
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
