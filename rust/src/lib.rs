use crossbeam_channel::{unbounded, Sender, Receiver};
use godot::{prelude::*, private::class_macros::auto_register_classes};
use lazy_static::lazy_static;
mod client_scripts;
mod console_handler;
mod controller;
mod main_scene;
mod utils;
mod world;

struct HonnyCraft;

#[gdextension]
unsafe impl ExtensionLibrary for HonnyCraft {
    fn load_library(handle: &mut InitHandle) -> bool {
        handle.register_layer(InitLevel::Scene, DefaultLayer);
        true
    }
}

lazy_static! {
    pub static ref CHANNEL: (Sender<i32>, Receiver<i32>) = unbounded();
}

struct DefaultLayer;

impl ExtensionLayer for DefaultLayer {
    fn initialize(&mut self) {
        auto_register_classes();

        // rayon::spawn(move || {
        //     loop {
        //         println!("test loop");
        //         thread::sleep(Duration::from_secs(1));
        //     }
        // });
    }

    fn deinitialize(&mut self) {
        // Nothing -- note that any cleanup task should be performed outside of this method,
        // as the user is free to use a different impl, so cleanup code may not be run.
    }
}
