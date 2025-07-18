use generate_image::IMAGE_SIZE;
use gui::MyApp;

mod generate_image;
mod gui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([IMAGE_SIZE as f32 + 15.0, IMAGE_SIZE as f32 + 15.0]),
        ..Default::default()
    };
    eframe::run_native("Map Viewver", options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))))
}
