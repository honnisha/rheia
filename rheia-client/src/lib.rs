use godot::prelude::*;
mod client_scripts;
mod console;
mod controller;
mod logger;
mod main_scene;
mod network;
mod utils;
mod world;
mod debug;
mod entities;
mod text_screen;

struct Rheia;

#[gdextension]
unsafe impl ExtensionLibrary for Rheia {}
