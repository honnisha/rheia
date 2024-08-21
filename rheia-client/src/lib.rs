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

struct Rheia;

#[gdextension]
unsafe impl ExtensionLibrary for Rheia {}
