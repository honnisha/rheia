use godot::{
    engine::{
        base_material_3d::{
            AlphaAntiAliasing, DepthDrawMode, ShadingMode, TextureFilter, TextureParam,
        },
        Image, ImageTexture, StandardMaterial3D,
    },
    prelude::{Gd, PackedByteArray, StringName, ToVariant},
};
use image::{imageops, ImageBuffer, RgbaImage};

fn generate_texture() -> Vec<u8> {
    let size = 16 * 32;
    let mut img: RgbaImage = ImageBuffer::new(size, size);

    let grass =
        image::open("/home/honnisha/godot/honny-craft/godot/assets/block/grass_top.png").unwrap();
    imageops::overlay(&mut img, &grass, 32, 0);

    let grass =
        image::open("/home/honnisha/godot/honny-craft/godot/assets/block/grass_block_side.png")
            .unwrap();
    imageops::overlay(&mut img, &grass, 32 * 2, 0);

    let grass =
        image::open("/home/honnisha/godot/honny-craft/godot/assets/block/dirt.png").unwrap();
    imageops::overlay(&mut img, &grass, 32 * 3, 0);


    let fout = &mut File::create(&Path::new("/home/honnisha/godot/honny-craft/godot/assets/test.png")).unwrap();
    im.write_to(fout, ImageFormat::Png).unwrap();

    img.as_raw().to_vec()
}

pub fn get_blocks_material() -> Gd<StandardMaterial3D> {
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
    pba.extend(generate_texture());

    let mut image = Image::new();
    image.load_png_from_buffer(pba);
    let mut texture = ImageTexture::new();
    texture.set_image(image);
    material.set_texture(TextureParam::TEXTURE_ALBEDO, texture.upcast());

    material
}
