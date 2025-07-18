use common::world_generator::default::Noise;
use image::{ImageBuffer, Rgb};

pub fn generate_noise_image(width: u32, height: u32, noise_settings: Noise, seed: u64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut imgbuf = ImageBuffer::new(width, height);
    imgbuf.put_pixel(0, 0, Rgb([0_u8, 0_u8, 0_u8]));

    let noise = noise_settings.generate(seed);

    for x in 0..width {
        for y in 0..height {
            let value = noise.get_noise(x as f32, y as f32);
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            let v = (value * 255.0) as u8;
            *pixel = Rgb([v, v, v]);
        }
    }

    imgbuf
}

pub fn get_noise(width: u32, height: u32, noise_settings: Noise, seed: u64, x: &i32, y: &i32) -> f32 {
    let mut imgbuf = ImageBuffer::new(width, height);
    imgbuf.put_pixel(0, 0, Rgb([0_u8, 0_u8, 0_u8]));

    let noise = noise_settings.generate(seed);

    noise.get_noise(*x as f32, *y as f32)
}
