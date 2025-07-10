use godot::prelude::*;

mod client_scripts;
mod console;
mod controller;
mod debug;
mod entities;
mod logger;
mod network;
mod scenes;
mod ui;
mod utils;
mod world;

struct Rheia;

pub const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;
pub const WARNING_TIME: std::time::Duration = std::time::Duration::from_millis(10);
pub const MAX_THREADS: usize = 12;

#[gdextension]
unsafe impl ExtensionLibrary for Rheia {}

#[cfg(feature = "trace")]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
    tracy_client::ProfiledAllocator::new(std::alloc::System, 100);
