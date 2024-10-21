use std::{collections::HashMap, fs::File, path::PathBuf};

use bevy::prelude::{Res, ResMut, Resource};
use common::blocks::block_type::{BlockContent, BlockType};
use serde::{Deserialize, Serialize};

use crate::{
    client_resources::default_resources::DEFAULT_BLOCKS, launch_settings::LaunchSettings,
    network::runtime_plugin::RuntimePlugin,
};

use super::resources_manager::ResourceManager;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServerSettingsManifest {
    pub blocks: Option<HashMap<u32, BlockType>>,
}

#[derive(Resource)]
pub struct ServerSettings {
    block_types: HashMap<u32, BlockType>,
    loaded: bool,
}

impl ServerSettings {
    pub fn new() -> Self {
        Self {
            block_types: Default::default(),
            loaded: false,
        }
    }

    pub fn get_block_types(&self) -> &HashMap<u32, BlockType> {
        assert!(self.loaded, "server settings is not loaded");
        &self.block_types
    }

    fn load(&mut self, path: PathBuf, resource_manager: &ResourceManager) -> Result<(), String> {
        log::info!(target: "server_settings", "Start loading server settings &e{}", path.display());

        self.block_types = DEFAULT_BLOCKS.clone();

        if !path.exists() {
            let default_manifest = ServerSettingsManifest {
                blocks: Some(self.block_types.clone()),
            };

            let file = File::create(path.clone()).expect("File should exist");
            serde_yaml::to_writer(file, &default_manifest).unwrap();

            log::info!(target: "server_settings", "Settings file is not exists; Default file was created");
            return Ok(());
        }

        let manifest = match std::fs::read_to_string(path.clone()) {
            Ok(d) => d,
            Err(e) => {
                return Err(format!("Settings file {} file error: &c{}", path.display(), e));
            }
        };

        let manifest_result: Result<ServerSettingsManifest, serde_yaml::Error> = serde_yaml::from_str(&manifest);
        let manifest_info = match manifest_result {
            Ok(m) => m,
            Err(e) => {
                return Err(format!("Settings file &e{}&r yaml parse error: &c{}", path.display(), e));
            }
        };

        if let Some(block_types) = manifest_info.blocks {
            for (i, block_type) in block_types.iter() {
                match block_type.get_block_content() {
                    BlockContent::Texture {
                        texture,
                        side_texture,
                        bottom_texture,
                    } => {
                        if !resource_manager.has_media(texture) {
                            return Err(format!("&ctexture not found: {}", texture));
                        }
                        if side_texture.is_some() && !resource_manager.has_media(&side_texture.as_ref().unwrap()) {
                            return Err(format!("&ctexture not found: {}", side_texture.as_ref().unwrap()));
                        }
                        if bottom_texture.is_some() && !resource_manager.has_media(&bottom_texture.as_ref().unwrap()) {
                            return Err(format!("&ctexture not found: {}", bottom_texture.as_ref().unwrap()));
                        }
                    }
                    BlockContent::ModelCube { model } => {
                        if !resource_manager.has_media(model) {
                            return Err(format!("&cmodel not found: {}", model));
                        }
                    }
                }
                self.block_types.insert(i.clone(), block_type.clone());
            }
        }

        self.loaded = true;
        log::info!(target: "server_settings", "Server settings loaded successfully; {} blocks", self.get_blocks_count());
        return Ok(());
    }

    pub fn get_blocks_count(&self) -> usize {
        self.block_types.len()
    }
}

pub(crate) fn rescan_server_settings(
    mut server_settings: ResMut<ServerSettings>,
    launch_settings: Res<LaunchSettings>,
    resource_manager: Res<ResourceManager>,
) {
    let mut path = launch_settings.get_resources_path();
    path.push("settings.yml");
    let loaded = server_settings.load(path, &*resource_manager);

    if let Some(e) = loaded.err() {
        log::error!(target: "server_settings", "Error loading server settings: {e}");
        RuntimePlugin::stop();
    };
}
