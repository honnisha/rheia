use common::blocks::block_type::BlockContent;
use godot::{
    classes::{base_material_3d::TextureParam, Image, ImageTexture, StandardMaterial3D},
    obj::NewGd,
    prelude::{Gd, PackedByteArray},
};
use image::{ImageBuffer, ImageFormat, RgbaImage};
use std::io::Cursor;

use crate::{client_scripts::resource_manager::ResourceManager, world::block_storage::BlockStorage};

use super::texture_mapper::TextureMapper;

pub fn generate_texture(
    texture_mapper: &mut TextureMapper,
    block_storage: &BlockStorage,
    resource_manager: &ResourceManager,
) -> Result<Vec<u8>, String> {
    let size = 16 * 32;
    let mut img: RgbaImage = ImageBuffer::new(size, size);

    for (_index, block_type) in block_storage.iter() {
        match block_type.get_block_content() {
            BlockContent::Texture {
                texture,
                side_texture,
                bottom_texture,
            } => {
                if let Err(e) = texture_mapper.load_image(&mut img, texture, resource_manager) {
                    return Err(e);
                }

                if let Some(t) = side_texture {
                    if let Err(e) = texture_mapper.load_image(&mut img, t, resource_manager) {
                        return Err(e);
                    }
                }

                if let Some(t) = bottom_texture {
                    if let Err(e) = texture_mapper.load_image(&mut img, t, resource_manager) {
                        return Err(e);
                    }
                }
            }
            _ => continue,
        }
    }

    let mut b: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut b), ImageFormat::Png).unwrap();

    return Ok(b.to_vec());
}
