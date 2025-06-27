use common::blocks::block_type::BlockContent;
use image::{ImageBuffer, ImageFormat, RgbaImage};
use std::io::Cursor;

use crate::{client_scripts::resource_manager::ResourceStorage, world::block_storage::BlockStorage};

use super::texture_mapper::TextureMapper;

pub fn generate_texture(
    texture_mapper: &mut TextureMapper,
    block_storage: &BlockStorage,
    resource_storage: &ResourceStorage,
) -> Result<Vec<u8>, String> {
    let size = 16 * 32;
    let mut img: RgbaImage = ImageBuffer::new(size, size);

    for block_type in block_storage.iter_values() {
        match block_type.get_block_content() {
            BlockContent::Texture {
                texture,
                side_texture,
                bottom_texture,
                ..
            } => {
                if let Err(e) = texture_mapper.load_image(&mut img, texture, resource_storage) {
                    return Err(e);
                }

                if let Some(t) = side_texture {
                    if let Err(e) = texture_mapper.load_image(&mut img, t, resource_storage) {
                        return Err(e);
                    }
                }

                if let Some(t) = bottom_texture {
                    if let Err(e) = texture_mapper.load_image(&mut img, t, resource_storage) {
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
