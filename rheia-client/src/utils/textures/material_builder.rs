use common::blocks::blocks_storage::BlockType;
use godot::{
    engine::{
        base_material_3d::{AlphaAntiAliasing, DepthDrawMode, ShadingMode, TextureFilter, TextureParam},
        Engine, Image, ImageTexture, StandardMaterial3D, Texture2D,
    },
    obj::NewGd,
    prelude::{try_load, Gd, PackedByteArray, StringName, ToGodot},
};
use image::{imageops, ImageBuffer, ImageFormat, RgbaImage};
use log::error;
use log::trace;
use std::io::Cursor;

use super::texture_mapper::TextureMapper;

fn load_image(texture_mapper: &mut TextureMapper, img: &mut RgbaImage, texture_option: Option<&'static str>) {
    let texture = match texture_option {
        Some(t) => t,
        None => {
            return;
        }
    };

    let path = format!("res://assets/block/{}", texture);
    let image = match try_load::<Texture2D>(&path) {
        //let image = match Image::load_from_file(GString::from(&path)) {
        Ok(t) => t.get_image().unwrap(),
        Err(e) => {
            error!("Can't load texture \"{}\"; {:?}", path, e);
            return;
        }
    };
    let b = image.save_png_to_buffer();

    let image_png = match image::load_from_memory(&b.to_vec()) {
        Ok(t) => t,
        Err(e) => {
            error!("Can't load texture \"{}\"; error: {:?}", path, e);
            return;
        }
    };
    let index = match texture_mapper.add_texture(texture.to_string()) {
        Some(i) => i,
        None => {
            return;
        }
    };

    let offset_x = 16 * (index % 32_i64);
    let offset_y = 16 * (index as f64 / 32_f64).floor() as i64;

    imageops::overlay(img, &image_png, offset_x, offset_y);
}

fn generate_texture(texture_mapper: &mut TextureMapper) -> Vec<u8> {
    let size = 16 * 32;
    let mut img: RgbaImage = ImageBuffer::new(size, size);

    for block_type in BlockType::to_iter() {
        let block_type_data = match block_type.get_block_type_info() {
            Some(d) => d,
            None => continue,
        };

        load_image(texture_mapper, &mut img, block_type_data.top_texture);
        load_image(texture_mapper, &mut img, block_type_data.side_texture);
        load_image(texture_mapper, &mut img, block_type_data.bottom_texture);
    }

    let mut b: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut b), ImageFormat::Png).unwrap();

    b.to_vec()
}

pub fn build_blocks_material(texture_mapper: &mut TextureMapper) -> Gd<StandardMaterial3D> {
    trace!("build_blocks_material started");
    let mut material = StandardMaterial3D::new_gd();
    if Engine::singleton().is_editor_hint() {
        return material;
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
    pba.extend(generate_texture(texture_mapper));

    let mut image = Image::new_gd();
    image.load_png_from_buffer(pba);
    let mut texture = ImageTexture::new_gd();
    texture.set_image(image);
    material.set_texture(TextureParam::ALBEDO, texture.upcast());

    trace!("build_blocks_material completed");
    material
}
