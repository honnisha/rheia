use godot::prelude::*;
mod client_scripts;
mod console;
mod controller;
mod events;
mod logger;
mod main_scene;
mod network;
mod utils;
mod world;
mod debug;

struct Rheia;

#[gdextension]
unsafe impl ExtensionLibrary for Rheia {}
