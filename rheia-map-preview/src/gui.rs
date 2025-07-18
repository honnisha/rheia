use bracket_lib::random::RandomNumberGenerator;
use common::world_generator::default::Noise;
use egui::{ColorImage, TextureHandle};
use image::{ImageBuffer, Rgb};

use crate::{generate_image::generate_map_image, noise::{generate_noise_image, get_noise}};

const NOISE_WIDTH: u32 = 600;
const NOISE_HEIGHT: u32 = 600;
const INPUT_LINES: usize = 8;

#[derive(Default)]
pub struct MyApp {
    seed: String,
    seed_value: u64,

    texture: Option<TextureHandle>,
    noise_texture: Option<TextureHandle>,
    noise_setting: String,
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
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        // app.generate_image(&cc.egui_ctx);

        let mut rng = RandomNumberGenerator::new();
        app.seed_value = rng.next_u64();
        app.seed = app.seed_value.to_string();

        app.noise_setting = serde_yaml::to_string(&Noise::default()).unwrap();

        app
    }

    pub fn generate_image(&mut self, ctx: &egui::Context) {
        self.texture = None;
        println!("Start generate map image");
        let image = match generate_map_image(self.seed_value.clone()) {
            Ok(i) => i,
            Err(e) => {
                println!("Generation map error: {}", e);
                return;
            },
        };
        let color_image = image_buffer_to_color_image(&image);

        let texture = ctx.load_texture("map-image", color_image, egui::TextureOptions::default());
        self.texture = Some(texture);
        println!("Generation map complete!");
    }

    pub fn generate_noise_image(&mut self, ctx: &egui::Context) {
        self.noise_texture = None;

        let noise_settings: Noise = match serde_yaml::from_str(&self.noise_setting) {
            Ok(s) => s,
            Err(e) => {
                println!("Noise settings error: {}", e);
                return;
            }
        };

        println!("Start generate noise image");
        let image = generate_noise_image(NOISE_WIDTH, NOISE_HEIGHT, noise_settings, self.seed_value.clone());
        let color_image = image_buffer_to_color_image(&image);

        let texture = ctx.load_texture("noise-image", color_image, egui::TextureOptions::default());
        self.noise_texture = Some(texture);
        println!("Generation noise complete!");
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Seed:");
                if ui.text_edit_singleline(&mut self.seed).changed() {
                    match self.seed.parse::<u64>() {
                        Ok(val) => {
                            self.seed_value = val;
                            println!("Seed updated: {}", self.seed_value);
                        },
                        Err(e) => {
                            println!("Seed paring error: {}", e);
                        },
                    }
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Randomize seed").clicked() {
                    let mut rng = RandomNumberGenerator::new();
                    self.seed_value = rng.next_u64();
                    self.seed = self.seed_value.to_string();
                }
            });
            ui.separator();

            if ui.button("Update map").clicked() {
                self.generate_image(&ctx);
            }

            if let Some(texture) = &self.texture {
                ui.image((texture.id(), texture.size_vec2()));
            }

            ui.separator();

            ui.label("Noise settings:");
            ui.add(egui::TextEdit::multiline(&mut self.noise_setting).desired_rows(INPUT_LINES));

            ui.horizontal(|ui| {
                if ui.button("Generate noise").clicked() {
                    self.generate_noise_image(&ctx);
                }
                if ui.button("Randomize and generate").clicked() {
                    let mut rng = RandomNumberGenerator::new();
                    self.seed_value = rng.next_u64();
                    self.seed = self.seed_value.to_string();
                    self.generate_noise_image(&ctx);
                }
                if ui.button("Save noise").clicked() {
                    let noise_settings: Noise = match serde_yaml::from_str(&self.noise_setting) {
                        Ok(s) => s,
                        Err(e) => {
                            println!("Noise settings error: {}", e);
                            return;
                        }
                    };
                    let image = generate_noise_image(NOISE_WIDTH, NOISE_HEIGHT, noise_settings, self.seed_value.clone());
                    image.save("noise.png").unwrap();
                    println!("noise.png saved");
                }
            });

            if let Some(texture) = &self.noise_texture {
                let img = ui.image((texture.id(), texture.size_vec2()));

                if img.hovered() {
                    if let Some(pointer_pos) = img.hover_pos() {
                        let rect = img.rect;
                        let rel_x = (pointer_pos.x - rect.left()) as i32;
                        let rel_y = (pointer_pos.y - rect.top()) as i32;


                        let noise_settings: Noise = match serde_yaml::from_str(&self.noise_setting) {
                            Ok(s) => s,
                            Err(e) => {
                                println!("Noise settings error: {}", e);
                                return;
                            }
                        };

                        let value = get_noise(NOISE_WIDTH, NOISE_HEIGHT, noise_settings, self.seed_value.clone(), &rel_x, &rel_y);
                        ui.label(format!("Position: x:{} y:{} value:{:.2}", rel_x, rel_y, value));
                    }
                }
            }

            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                println!("Closing window");
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
