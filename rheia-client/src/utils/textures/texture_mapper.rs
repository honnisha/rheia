use common::blocks::block_type::{BlockContent, BlockType};
use godot::{engine::Material, obj::Gd};

use crate::world::worlds_manager::BlockStorageRef;

use super::material_builder::build_blocks_material;

#[derive(Debug)]
pub struct TextureMapper {
    textures_map: Vec<String>,
}

impl TextureMapper {
    pub fn new() -> TextureMapper {
        TextureMapper {
            textures_map: Vec::new(),
        }
    }

    pub fn build(&mut self, block_storage: &BlockStorageRef) -> Gd<Material> {
        let texture = build_blocks_material(self, block_storage);
        texture.duplicate().unwrap().cast::<Material>()
    }

    pub fn add_texture(&mut self, texture_name: String) -> Option<i64> {
        if self.textures_map.contains(&texture_name) {
            return None;
        };
        self.textures_map.push(texture_name);
        Some(self.textures_map.len() as i64 - 1_i64)
    }

    #[allow(unused_variables)]
    pub fn get_uv_offset(&self, block_type: &BlockType, side_index: i8) -> Option<usize> {
        let texture = match block_type.get_block_content() {
            BlockContent::Texture {
                texture,
                side_texture,
                bottom_texture,
            } => {
                match side_index {
                    // Topside
                    4 => texture,
                    // Bottom
                    1 => match bottom_texture {
                        Some(t) => t,
                        None => texture,
                    },
                    // Sides
                    _ => match side_texture {
                        Some(t) => t,
                        None => texture,
                    },
                }
            }
            BlockContent::ModelCube { model } => return None,
        };

        self.textures_map.iter().position(|t| t == texture)
    }
}
