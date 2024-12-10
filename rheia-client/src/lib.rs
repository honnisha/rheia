use godot::prelude::*;

mod scenes;
mod client_scripts;
mod console;
mod controller;
mod logger;
mod network;
mod utils;
mod world;
mod debug;
mod entities;
mod ui;

struct Rheia;

#[gdextension]
unsafe impl ExtensionLibrary for Rheia {}
