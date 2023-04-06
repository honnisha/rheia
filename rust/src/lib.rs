/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{sync::Mutex, thread, time::Duration};

use godot::{prelude::*, private::class_macros::auto_register_classes};
use rayon::ThreadPool;
//use tokio::runtime::Runtime;
//use tokio::time::{sleep, Duration};
mod client_scripts;
mod console_handler;
mod controller;
mod main_scene;
mod utils;
mod world;
use lazy_static::lazy_static;

struct HonnyCraft;

#[gdextension]
unsafe impl ExtensionLibrary for HonnyCraft {
    fn load_library(handle: &mut InitHandle) -> bool {
        handle.register_layer(InitLevel::Scene, DefaultLayer);
        true
    }
}

//lazy_static! {
//    pub static ref RUNTIME: Mutex<Runtime> = Mutex::new(tokio::runtime::Builder::new_multi_thread()
//        .enable_all()
//        .build()
//        .unwrap());
//}

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
