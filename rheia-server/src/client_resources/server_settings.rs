use super::resources_manager::ResourceManager;
use crate::{launch_settings::LaunchSettings, network::runtime_plugin::RuntimePlugin};
use bevy::prelude::{Res, ResMut, Resource};
use common::{
    blocks::{block_info::generate_block_id, block_type::BlockType, default_blocks::DEFAULT_BLOCKS},
    chunks::chunk_data::BlockIndexType,
};
use network::messages::ServerMessages;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::File, path::PathBuf};

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ServerSettingsManifest {
    block_id_map: Option<BTreeMap<BlockIndexType, String>>,
}

#[derive(Resource)]
pub struct ServerSettings {
    blocks: Vec<BlockType>,
    loaded: bool,

    block_id_map: Option<BTreeMap<BlockIndexType, String>>,
}

impl Default for ServerSettings {
    fn default() -> Self {
        let mut server_settings = Self {
            blocks: Default::default(),
            loaded: false,
            block_id_map: Default::default(),
        };

        for block_type in DEFAULT_BLOCKS.iter() {
            server_settings.blocks.push(block_type.clone());
        }
        server_settings
    }
}

impl ServerSettings {
    pub fn get_network_settings(&self) -> ServerMessages {
        assert!(self.loaded, "server settings is not loaded");
        ServerMessages::Settings {
            block_types: self.blocks.clone(),
            block_id_map: self.block_id_map.as_ref().unwrap().clone(),
        }
    }

    fn load(&mut self, path: PathBuf, _resource_manager: &ResourceManager) -> Result<(), String> {
        log::info!(target: "server_settings", "Start loading server settings &e{}", path.display());

        if !path.exists() {
            // Create settings with default blocks
            let default_manifest = ServerSettingsManifest { block_id_map: None };

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

        let mut block_id_map = match manifest_info.block_id_map {
            Some(m) => m,
            None => Default::default(),
        };

        for block_type in self.blocks.iter() {
            let mut existed = false;
            for (_block_id, block_slug) in block_id_map.iter() {
                if block_slug == block_type.get_slug() {
                    existed = true;
                }
            }

            let mut last_id: BlockIndexType = 0;
            for (block_id, _block_slug) in block_id_map.iter() {
                last_id = *block_id.max(&last_id);
            }

            if !existed {
                let block_id = generate_block_id(&block_type, last_id);
                block_id_map.insert(block_id, block_type.get_slug().clone());
            }
        }

        self.block_id_map = Some(block_id_map.clone());

        let manifest = ServerSettingsManifest {
            block_id_map: Some(block_id_map),
        };
        let file = File::create(path.clone()).expect("File must exists");
        serde_yaml::to_writer(file, &manifest).unwrap();

        self.loaded = true;
        log::info!(target: "server_settings", "Server settings loaded successfully; &e{} blocks", self.get_blocks_count());
        Ok(())
    }

    pub fn get_block_id_map(&self) -> &BTreeMap<u16, String> {
        self.block_id_map.as_ref().expect("block_id_map is not set")
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

    if let Err(e) = server_settings.load(path, &*resource_manager) {
        log::error!(target: "server_settings", "Error loading server settings: {e}");
        RuntimePlugin::stop();
    };
}
