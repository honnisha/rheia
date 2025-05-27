use super::resources_manager::ResourceManager;
use crate::{launch_settings::LaunchSettings, network::runtime_plugin::RuntimePlugin};
use bevy::prelude::{Res, ResMut, Resource};
use common::blocks::{block_type::BlockType, default_blocks::DEFAULT_BLOCKS};
use network::messages::ServerMessages;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ServerSettingsManifest {
    blocks: Option<Vec<BlockType>>,
}

#[derive(Resource, Default)]
pub struct ServerSettings {
    blocks: Vec<BlockType>,
    loaded: bool,
}

impl ServerSettings {
    pub fn get_network_settings(&self) -> ServerMessages {
        assert!(self.loaded, "server settings is not loaded");
        ServerMessages::Settings {
            block_types: self.blocks.clone(),
        }
    }

    fn load(&mut self, path: PathBuf, resource_manager: &ResourceManager) -> Result<(), String> {
        log::info!(target: "server_settings", "Start loading server settings &e{}", path.display());

        for block_type in DEFAULT_BLOCKS.iter() {
            self.blocks.push(block_type.clone());
        }

        if !path.exists() {
            // Create settings with default blocks
            let default_manifest = ServerSettingsManifest {
                blocks: Some(DEFAULT_BLOCKS.clone()),
            };

            let file = File::create(path.clone()).expect("File must exists");
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
                return Err(format!(
                    "Settings file &e{}&r yaml parse error: &c{}",
                    path.display(),
                    e
                ));
            }
        };

        if let Some(blocks) = manifest_info.blocks {
            if let Err(e) = resource_manager.validate_blocks(&blocks) {
                return Err(e);
            }
            for block_type in blocks.iter() {
                self.blocks.push(block_type.clone());
            }
        }

        self.loaded = true;
        log::info!(target: "server_settings", "Server settings loaded successfully; &e{} blocks", self.get_blocks_count());
        Ok(())
    }

    pub fn get_blocks(&self) -> &Vec<BlockType> {
        &self.blocks
    }

    pub fn add_block(&mut self, block_type: BlockType) {
        self.blocks.push(block_type);
    }

    pub fn get_blocks_count(&self) -> usize {
        self.blocks.len()
    }
}

pub(crate) fn rescan_server_settings(
    mut server_settings: ResMut<ServerSettings>,
    launch_settings: Res<LaunchSettings>,
    resource_manager: Res<ResourceManager>,
) {
    if RuntimePlugin::is_stopped() {
        return;
    }

    let mut path = launch_settings.get_resources_path();
    path.push("settings.yml");
    let loaded = server_settings.load(path, &*resource_manager);

    if let Some(e) = loaded.err() {
        log::error!(target: "server_settings", "Error loading server settings: {e}");
        RuntimePlugin::stop();
    };
}
