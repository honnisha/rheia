use godot::{
    engine::{
        base_material_3d::{
            AlphaAntiAliasing, DepthDrawMode, ShadingMode, TextureFilter, TextureParam,
        },
        Image, ImageTexture, StandardMaterial3D,
    },
    prelude::{Gd, GodotString, PackedByteArray, StringName, ToVariant, godot_print, godot_error},
};
use image::{imageops, ImageBuffer, ImageFormat, RgbaImage};
use std::io::Cursor;
use strum::IntoEnumIterator;

use crate::blocks::block_type::BlockType;

use super::texture_mapper::TextureMapper;

fn load_image(
    texture_mapper: &mut TextureMapper,
    img: &mut RgbaImage,
    texture_option: Option<&'static str>,
) {
    let texture = match texture_option {
        Some(t) => t,
        None => {
            return;
        }
    };

    let image = match Image::load_from_file(GodotString::from(format!("res://assets/block/{}", texture))) {
        Some(t) => t,
        None => {
            godot_error!("Can't load texture \"{}\"; not found;", texture);
            return;
        }
    };
    let image_bytes = &image.get_data().to_vec();
    let image_png = match image::load_from_memory_with_format(&image_bytes, ImageFormat::Png) {
        Ok(t) => t,
        Err(e) => {
            godot_error!("Can't load texture \"{}\"; error: {:?}", texture, e);
            return;
        }
    };
    let index = texture_mapper.add_texture(texture.to_string());

    let offset_x = 16 * (index % 32_i64);
    let offset_y = 16 * (index as f64 / 32_f64).floor() as i64;

    imageops::overlay(img, &image_png, offset_x, offset_y);
}

fn generate_texture(texture_mapper: &mut TextureMapper) -> Vec<u8> {
    let size = 16 * 32;
    let mut img: RgbaImage = ImageBuffer::new(size, size);

    for block_type in BlockType::iter() {
        let block_type_data = match block_type.get_block_type_info() {
            Some(d) => d,
            None => continue,
        };

        load_image(texture_mapper, &mut img, block_type_data.top_texture);
        load_image(texture_mapper, &mut img, block_type_data.side_texture);
        load_image(texture_mapper, &mut img, block_type_data.bottom_texture);
    }

    let mut b: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut b), ImageFormat::Png)
        .unwrap();

    b.to_vec()
}

pub fn build_blocks_material(texture_mapper: &mut TextureMapper) -> Gd<StandardMaterial3D> {
    let mut material = StandardMaterial3D::new();
    material.set_alpha_scissor_threshold(0_f64);
    material.set_alpha_antialiasing(AlphaAntiAliasing::ALPHA_ANTIALIASING_OFF);

    material.set_shading_mode(ShadingMode::SHADING_MODE_PER_PIXEL);

    material.set_metallic(0_f64);
    material.set_specular(0_f64);

    material.set_roughness(0_f64);
    material.set_clearcoat(0.23_f64);

    material.set_texture_filter(TextureFilter::TEXTURE_FILTER_NEAREST);
    material.set_ao_light_affect(1.0_f64);
    material.set(StringName::from("ao_enabled"), true.to_variant());
    material.set_depth_draw_mode(DepthDrawMode::DEPTH_DRAW_OPAQUE_ONLY);
    material.set_refraction(0.27_f64);

    let mut pba = PackedByteArray::new();
    pba.extend(generate_texture(texture_mapper));

    let mut image = Image::new();
    image.load_png_from_buffer(pba);
    let mut texture = ImageTexture::new();
    texture.set_image(image);
    material.set_texture(TextureParam::TEXTURE_ALBEDO, texture.upcast());

    material
}