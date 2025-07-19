use common::world_generator::noise::Noise;
use image::{ImageBuffer, Rgb};

pub fn generate_noise_image(
    width: u32,
    height: u32,
    noise_settings: Noise,
    noise_second_settings: Option<Noise>,
    seed: u64,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut imgbuf = ImageBuffer::new(width, height);
    imgbuf.put_pixel(0, 0, Rgb([0_u8, 0_u8, 0_u8]));

    let noise = noise_settings.generate(seed);

    let second_noise = match noise_second_settings.as_ref() {
        Some(s) => Some(s.generate(seed)),
        None => None,
    };

    for x in 0..width {
        for y in 0..height {
            let mut value = noise.get_noise(x as f32, y as f32);
            if let Some(second_noise) = second_noise.as_ref() {
                value = value + (value * second_noise.get_noise(x as f32, y as f32));
                value = value.max(0.0).min(1.0);
            }
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            let v = (value * 255.0) as u8;
            *pixel = Rgb([v, v, v]);
        }
    }

    imgbuf
}

pub fn get_noise(width: u32, height: u32, noise_settings: Noise, noise_second_settings: Option<Noise>, seed: u64, x: &i32, y: &i32) -> f32 {
    let mut imgbuf = ImageBuffer::new(width, height);
    imgbuf.put_pixel(0, 0, Rgb([0_u8, 0_u8, 0_u8]));

    let noise = noise_settings.generate(seed);

    let second_noise = match noise_second_settings.as_ref() {
        Some(s) => Some(s.generate(seed)),
        None => None,
    };
    let mut value = noise.get_noise(*x as f32, *y as f32);
    if let Some(second_noise) = second_noise.as_ref() {
        value = value + (value * second_noise.get_noise(*x as f32, *y as f32));
        value = value.max(0.0).min(1.0);
    }
    value
}
