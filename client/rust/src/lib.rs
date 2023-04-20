use godot::{prelude::*, private::class_macros::auto_register_classes};
mod client_scripts;
mod controller;
mod main_scene;
mod utils;
mod world;
mod events;
mod console;
mod network;

struct HonnyCraft;

#[gdextension]
unsafe impl ExtensionLibrary for HonnyCraft {
    fn load_library(handle: &mut InitHandle) -> bool {
        handle.register_layer(InitLevel::Scene, DefaultLayer);
        true
    }
}

struct DefaultLayer;

impl ExtensionLayer for DefaultLayer {
    fn initialize(&mut self) {
        auto_register_classes();
    }

    fn deinitialize(&mut self) {
    }
}
