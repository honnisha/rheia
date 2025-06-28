use common::blocks::block_type::{BlockColor, BlockContent, BlockType};
use godot::{
    builtin::PackedByteArray,
    classes::{Image, ImageTexture, StandardMaterial3D, base_material_3d::TextureParam},
    obj::{Gd, NewGd},
};

use crate::{
    client_scripts::{resource_manager::ResourceStorage, texture_image::TexturePack},
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

        let m = match self.generate_texture(block_storage, resource_storage) {
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

    pub fn clear(&mut self) {
        self.textures_map.clear();
    }

    fn generate_texture(
        &mut self,
        block_storage: &BlockStorage,
        resource_storage: &ResourceStorage,
    ) -> Result<Vec<u8>, String> {
        let mut texture_pack = TexturePack::create();

        for block_type in block_storage.iter_values() {
            match block_type.get_block_content() {
                BlockContent::Texture {
                    texture,
                    side_texture,
                    side_overlay,
                    bottom_texture,
                    colors_scheme,
                } => {
                    // Top texture
                    let mut texture_image = resource_storage.generate_image(texture).unwrap();
                    match colors_scheme {
                        Some(colors) => {
                            for color in colors {
                                let index = self.add_texture_index(texture.clone(), Some(&color));
                                texture_image = texture_image.change_color_balance(color);
                                texture_pack.add_subimage(&texture_image, index);
                            }
                        }
                        None => {
                            let index = self.add_texture_index(texture.clone(), None);
                            texture_pack.add_subimage(&texture_image, index);
                        }
                    }

                    // Side texture
                    if let Some(texture) = side_texture.as_ref() {
                        let mut side_texture_image = resource_storage.generate_image(texture).unwrap();
                        match colors_scheme {
                            Some(colors) => {
                                for color in colors {
                                    let index = self.add_texture_index(texture.clone(), Some(&color));

                                    if let Some(side_overlay) = side_overlay {
                                        let mut overlay_texture_image =
                                            resource_storage.generate_image(side_overlay).unwrap();
                                        overlay_texture_image = overlay_texture_image.change_color_balance(color);

                                        let new_side_texture_image =
                                            side_texture_image.overlay_on_top(&overlay_texture_image);
                                        texture_pack.add_subimage(&new_side_texture_image, index);
                                    } else {
                                        texture_pack.add_subimage(&side_texture_image, index);
                                    }
                                }
                            }
                            None => {
                                let index = self.add_texture_index(texture.clone(), None);
                                if let Some(side_overlay) = side_overlay {
                                    let overlay_texture_image =
                                        resource_storage.generate_image(side_overlay).unwrap();

                                    let new_side_texture_image =
                                        side_texture_image.overlay_on_top(&overlay_texture_image);
                                    texture_pack.add_subimage(&new_side_texture_image, index);
                                } else {
                                    texture_pack.add_subimage(&side_texture_image, index);
                                }
                            }
                        }
                    }

                    // Bottom texture
                    if let Some(texture) = bottom_texture.as_ref() {
                        let index = self.add_texture_index(texture.clone(), None);
                        let texture_image = resource_storage.generate_image(texture).unwrap();
                        texture_pack.add_subimage(&texture_image, index);
                    }
                }
                _ => continue,
            }
        }

        return Ok(texture_pack.generate());
    }

    pub fn add_texture_index(&mut self, texture_name: String, _color: Option<&BlockColor>) -> i64 {
        // TODO: color

        assert!(!self.textures_map.contains(&texture_name), "texture already exists");
        self.textures_map.push(texture_name);
        self.textures_map.len() as i64 - 1_i64
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
}
