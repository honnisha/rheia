/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;

mod main_scene;
mod client_scripts;
mod mesh;
mod chunks;
mod world;
pub mod utils;
pub mod world_generator;
pub mod schematics;
pub mod blocks;
pub mod textures;

struct HonnyCraft;

#[gdextension]
unsafe impl ExtensionLibrary for HonnyCraft {}
