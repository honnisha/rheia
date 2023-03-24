use godot::{
    engine::{
        base_material_3d::{
            AlphaAntiAliasing, DepthDrawMode, ShadingMode, TextureFilter, TextureParam,
        },
        Image, ImageTexture, StandardMaterial3D,
    },
    prelude::{Gd, PackedByteArray, StringName, ToVariant, try_load},
};
use image::{imageops, ImageBuffer, ImageFormat, RgbaImage};
use std::io::Cursor;
use strum::IntoEnumIterator;

use crate::blocks::block_type::BlockType;

use super::texture_mapper::TextureMapper;

fn generate_texture(texture_mapper: &mut TextureMapper) -> Vec<u8> {
    let size = 16 * 32;
    let mut img: RgbaImage = ImageBuffer::new(size, size);

    for block_type in BlockType::iter() {
        let block_type_data = match block_type.get_block_type_info() {
            Some(d) => d,
            None => continue
        };

        match block_type_data.top_texture {
            Some(t) => {
                let try_load(format!("res://assets/block/{}", t));
                imageops::overlay(&mut img, &grass, 16, 0);
            }
            None => ()
        }
    }

//    let grass =
//        image::open("/home/honnisha/godot/honny-craft/godot/assets/block/grass_block_side.png")
//            .unwrap();
//    imageops::overlay(&mut img, &grass, 16 * 2, 0);

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
