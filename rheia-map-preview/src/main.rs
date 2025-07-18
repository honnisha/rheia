use gui::MyApp;

mod generate_image;
mod noise;
mod gui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native("Map Viewver", options, Box::new(|cc| Ok(Box::new(MyApp::new(cc)))))
}
