use common::blocks::block_type::BlockContent;
use godot::{
    engine::{
        base_material_3d::{AlphaAntiAliasing, DepthDrawMode, ShadingMode, TextureFilter, TextureParam},
        Engine, Image, ImageTexture, StandardMaterial3D,
    },
    obj::NewGd,
    prelude::{Gd, PackedByteArray, StringName, ToGodot},
};
use image::{ImageBuffer, ImageFormat, RgbaImage};
use std::io::Cursor;

use crate::{client_scripts::resource_manager::ResourceManager, world::block_storage::BlockStorage};

use super::texture_mapper::TextureMapper;

fn generate_texture(
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

pub fn build_blocks_material(
    texture_mapper: &mut TextureMapper,
    block_storage: &BlockStorage,
    resource_manager: &ResourceManager,
) -> Result<Gd<StandardMaterial3D>, String> {
    log::trace!("build_blocks_material started");
    let mut material = StandardMaterial3D::new_gd();
    if Engine::singleton().is_editor_hint() {
        return Ok(material);
    }

    material.set_alpha_scissor_threshold(0_f32);
    material.set_alpha_antialiasing(AlphaAntiAliasing::OFF);

    material.set_shading_mode(ShadingMode::PER_PIXEL);

    material.set_metallic(0_f32);
    material.set_specular(0_f32);

    material.set_roughness(0_f32);
    material.set_clearcoat(0.23_f32);

    material.set_texture_filter(TextureFilter::NEAREST);
    material.set_ao_light_affect(1.0_f32);
    material.set(StringName::from("ao_enabled"), true.to_variant());
    material.set_depth_draw_mode(DepthDrawMode::OPAQUE_ONLY);
    material.set_refraction(0.27_f32);

    let mut pba = PackedByteArray::new();

    let m = match generate_texture(texture_mapper, block_storage, resource_manager) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };
    pba.extend(m);

    let mut image = Image::new_gd();
    image.load_png_from_buffer(pba);
    let mut texture = ImageTexture::new_gd();
    texture.set_image(image);
    material.set_texture(TextureParam::ALBEDO, texture.upcast());

    log::trace!("build_blocks_material completed");
    Ok(material)
}
