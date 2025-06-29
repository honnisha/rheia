use common::blocks::block_type::BlockColor;
use godot::builtin::PackedByteArray;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat, Pixel, Rgba, RgbaImage};
use std::io::Cursor;

pub struct TextureImage {
    image: DynamicImage,
}

impl TextureImage {
    pub(crate) fn create(image_buffer: PackedByteArray) -> Result<Self, String> {
        let image = match image::load_from_memory(&image_buffer.to_vec()) {
            Ok(t) => t,
            Err(e) => {
                return Err(format!("generation error: {:?}", e));
            }
        };
        Ok(Self { image })
    }

    pub fn get_source(&self) -> &DynamicImage {
        &self.image
    }

    pub fn change_color_balance(&mut self, color: &BlockColor) -> Self {
        let tint_color = Rgba([color[0], color[1], color[2], 255]);

        let (width, height) = self.image.dimensions();
        let mut output = ImageBuffer::new(width, height);

        for (x, y, pixel) in self.image.pixels() {
            let original = pixel;

            if original.to_rgba()[3] == 0 {
                continue;
            }

            let blended = imageproc::pixelops::interpolate(original, tint_color, 0.5);
            output.put_pixel(x, y, blended);
        }

        Self { image: output.into() }
    }

    pub fn overlay_on_top(&mut self, image: &TextureImage) -> Self {
        let mut background = self.image.clone();

        image::imageops::overlay(&mut background, &image.get_source().to_rgba8(), 0, 0);

        Self { image: background }
    }
}

/// Generates one big texture, which is contains all block textures
pub(crate) struct TexturePack {
    img: RgbaImage,
}

impl TexturePack {
    pub(crate) fn create() -> Self {
        let size = 16 * 32;
        let img: RgbaImage = ImageBuffer::new(size, size);
        Self { img }
    }

    pub(crate) fn add_subimage(&mut self, image: &TextureImage, index: i64) {
        let offset_x = 16 * (index % 32_i64);
        let offset_y = 16 * (index as f64 / 32_f64).floor() as i64;

        image::imageops::overlay(&mut self.img, image.get_source(), offset_x, offset_y);
    }

    pub(crate) fn generate(&self) -> Vec<u8> {
        let mut b: Vec<u8> = Vec::new();
        self.img.write_to(&mut Cursor::new(&mut b), ImageFormat::Png).unwrap();
        b.to_vec()
    }
}
