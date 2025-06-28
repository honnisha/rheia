use super::resources_manager::ResourceManager;
use crate::{launch_settings::LaunchSettings, network::runtime_plugin::RuntimePlugin};
use bevy::prelude::{Res, ResMut, Resource};
use common::{
    blocks::{block_info::generate_block_id_map, block_type::BlockType},
    chunks::chunk_data::BlockIndexType,
    default_blocks::generate_default_blocks,
};
use network::messages::ServerMessages;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::File, path::PathBuf};

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct ServerSettingsManifest {
    block_id_map: Option<BTreeMap<BlockIndexType, String>>,
}

#[derive(Resource, Default)]
pub struct ServerSettings {
    blocks: Vec<BlockType>,
    loaded: bool,

    block_id_map: Option<BTreeMap<BlockIndexType, String>>,
}

impl ServerSettings {
    pub(crate) fn setup_blocks(&mut self) -> Result<(), String> {
        assert_eq!(self.blocks.len(), 0, "bloks must be empty");
        self.blocks.clear();

        let default_blocks = match generate_default_blocks() {
            Ok(m) => m,
            Err(e) => return Err(e),
        };
        for block_type in default_blocks.iter() {
            self.blocks.push(block_type.clone());
        }
        Ok(())
    }

    pub fn get_network_settings(&self) -> ServerMessages {
        assert!(self.loaded, "server settings is not loaded");
        ServerMessages::Settings {
            block_types: self.blocks.clone(),
            block_id_map: self.block_id_map.as_ref().unwrap().clone(),
        }
    }

    fn load(&mut self, path: PathBuf, _resource_manager: &ResourceManager) -> Result<(), String> {
        log::info!(target: "settings", "Start loading server settings &e{}", path.display());

        if !path.exists() {
            let default_manifest = ServerSettingsManifest { block_id_map: None };

            let file = File::create(path.clone()).expect("File must exists");
            serde_yaml::to_writer(file, &default_manifest).unwrap();

            log::info!(target: "settings", "Settings file is not exists; Default file was created");
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
                return Err(format!("&cfile &4{}&c yaml parse error: &c{}", path.display(), e));
            }
        };

        let mut block_id_map = match manifest_info.block_id_map {
            Some(m) => m,
            None => Default::default(),
        };

        if let Err(e) = generate_block_id_map(&mut block_id_map, self.blocks.iter()) {
            return Err(format!("&cfile &4{}&c block_id_map error: {}", path.display(), e));
        }

        self.block_id_map = Some(block_id_map.clone());

        let manifest = ServerSettingsManifest {
            block_id_map: Some(block_id_map),
        };
        let file = File::create(path.clone()).expect("File must exists");
        serde_yaml::to_writer(file, &manifest).unwrap();

        self.loaded = true;
        log::info!(target: "settings", "Server settings loaded successfully; &e{} blocks", self.get_blocks_count());
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

pub(crate) fn setup_default_blocks(
    mut server_settings: ResMut<ServerSettings>,
) {
    if RuntimePlugin::is_stopped() {
        return;
    }

    if let Err(e) = server_settings.setup_blocks() {
        log::error!(target: "settings", "&cSetup default blocks error:");
        log::error!(target: "settings", "{}", e);
        RuntimePlugin::stop();
        return;
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

    let mut path = launch_settings.get_server_data_path();
    path.push("settings.yml");

    if let Err(e) = server_settings.load(path, &*resource_manager) {
        log::error!(target: "settings", "&cError loading server settings:");
        log::error!(target: "settings", "{}", e);
        RuntimePlugin::stop();
    };
}
