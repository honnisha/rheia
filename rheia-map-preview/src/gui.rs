use egui::{ColorImage, TextureHandle};
use image::{ImageBuffer, Rgb};

use crate::generate_image::generate_image;

#[derive(Default)]
pub struct MyApp {
    texture: Option<TextureHandle>,
}

fn image_buffer_to_color_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ColorImage {
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image
        .pixels()
        .map(|p| egui::Color32::from_rgb(p[0], p[1], p[2]))
        .collect();
    ColorImage::new(size, pixels)
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self { texture: None };
        app.generate_image(&cc.egui_ctx);
        app
    }

    pub fn generate_image(&mut self, ctx: &egui::Context) {
        self.texture = None;
        println!("Start generate image");
        let image = generate_image();
        let color_image = image_buffer_to_color_image(&image);

        let texture = ctx.load_texture("my-image", color_image, egui::TextureOptions::default());
        self.texture = Some(texture);
        println!("Generation complete!");
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                self.generate_image(&ctx);
            }

            if let Some(texture) = &self.texture {
                ui.image((texture.id(), texture.size_vec2()));
            }

            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                println!("Closing window");
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
