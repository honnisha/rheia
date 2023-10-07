use godot::prelude::*;
mod client_scripts;
mod console;
mod controller;
mod entities;
mod events;
mod logger;
mod main_scene;
mod network;
mod utils;
mod world;
mod debug;

struct HonnyCraft;

#[gdextension]
unsafe impl ExtensionLibrary for HonnyCraft {}
