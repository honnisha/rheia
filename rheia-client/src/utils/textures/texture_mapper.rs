use common::blocks::block_type::{BlockContent, BlockType};
use godot::{
    builtin::PackedByteArray,
    classes::{base_material_3d::TextureParam, Image, ImageTexture, StandardMaterial3D},
    obj::{Gd, NewGd},
};
use image::RgbaImage;

use crate::{
    client_scripts::{resource_instance::MediaResource, resource_manager::ResourceStorage},
    utils::textures::material_builder::generate_texture,
    world::block_storage::BlockStorage,
};

#[derive(Debug, Default)]
pub struct TextureMapper {
    textures_map: Vec<String>,
}

impl TextureMapper {
    pub fn build(
        &mut self,
        block_storage: &BlockStorage,
        resource_storage: &ResourceStorage,
        material_3d: &mut Gd<StandardMaterial3D>,
    ) -> Result<(), String> {
        let mut pba = PackedByteArray::new();

        let m = match generate_texture(self, block_storage, resource_storage) {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
        pba.extend(m);

        let mut image = Image::new_gd();
        image.load_png_from_buffer(&pba);
        let mut image_texture = ImageTexture::new_gd();
        image_texture.set_image(&image);
        material_3d.set_texture(TextureParam::ALBEDO, &image_texture);
        Ok(())
    }

    pub fn add_texture(&mut self, texture_name: String) -> i64 {
        assert!(!self.textures_map.contains(&texture_name), "texture already exists");
        self.textures_map.push(texture_name);
        self.textures_map.len() as i64 - 1_i64
    }

    pub fn clear(&mut self) {
        self.textures_map.clear();
    }

    #[allow(unused_variables)]
    pub fn get_uv_offset(&self, block_type: &BlockType, side_index: i8) -> Option<usize> {
        let texture = match block_type.get_block_content() {
            BlockContent::Texture {
                texture,
                side_texture,
                bottom_texture,
                ..
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
            BlockContent::ModelCube { model, .. } => return None,
        };

        self.textures_map.iter().position(|t| t == texture)
    }

    pub fn len(&self) -> usize {
        self.textures_map.len()
    }

    pub fn load_image(
        &mut self,
        img: &mut RgbaImage,
        texture_path: &String,
        resource_storage: &ResourceStorage,
    ) -> Result<(), String> {
        if self.textures_map.contains(&texture_path) {
            return Ok(());
        };

        let Some(media_data) = resource_storage.get_media(texture_path) else {
            return Err(format!("Texture media \"{}\" not found inside resources", texture_path));
        };
        let texture_2d = match media_data {
            MediaResource::Texture(t) => t,
            _ => return Err("Textures only support png files".to_string()),
        };
        let b = texture_2d.get_image().unwrap().save_png_to_buffer();

        let image_png = match image::load_from_memory(&b.to_vec()) {
            Ok(t) => t,
            Err(e) => {
                return Err(format!("Can't load texture \"{}\"; error: {:?}", texture_path, e));
            }
        };

        let index = self.add_texture(texture_path.clone());

        let offset_x = 16 * (index % 32_i64);
        let offset_y = 16 * (index as f64 / 32_f64).floor() as i64;

        image::imageops::overlay(img, &image_png, offset_x, offset_y);
        Ok(())
    }
}
