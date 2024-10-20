use std::{collections::HashMap, path::PathBuf};

use bevy::prelude::{Res, ResMut, Resource};
use serde::{Deserialize, Serialize};

use crate::{launch_settings::LaunchSettings, network::runtime_plugin::RuntimePlugin};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BlockSettingsType {
    pub texture: Option<String>,
    pub side_texture: Option<String>,
    pub bottom_texture: Option<String>,

    pub model: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServerSettingsManifest {
    pub blocks: Option<HashMap<String, BlockSettingsType>>,
}

#[derive(Resource)]
pub struct ServerSettings {
    block_types: HashMap<String, BlockSettingsType>,
    loaded: bool,
}

impl ServerSettings {
    pub fn new() -> Self {
        Self {
            block_types: Default::default(),
            loaded: false,
        }
    }

    fn load(&mut self, path: PathBuf) -> Result<(), String> {
        log::info!(target: "server_settings", "Start loading server settings \"{}\"", path.display());

        if !path.exists() {
            return Err(format!("Settings file {} doesn't exists", path.display()));
        }

        let manifest = match std::fs::read_to_string(path.clone()) {
            Ok(d) => d,
            Err(e) => {
                return Err(format!("Settings file {} file error: {}", path.display(), e));
            }
        };

        let manifest_result: Result<ServerSettingsManifest, serde_yaml::Error> = serde_yaml::from_str(&manifest);
        let manifest_info = match manifest_result {
            Ok(m) => m,
            Err(e) => {
                return Err(format!("Settings file {} yaml parse error: {}", path.display(), e));
            }
        };
        if let Some(block_types) = manifest_info.blocks {
            self.block_types = block_types;
        }

        self.loaded = true;
        log::info!(target: "server_settings", "Server settings loaded successfully");
        return Ok(());
    }
}

pub(crate) fn rescan_server_settings(
    mut server_settings: ResMut<ServerSettings>,
    launch_settings: Res<LaunchSettings>,
) {
    let mut path = launch_settings.get_resources_path();
    path.push("settings.yml");
    let loaded = server_settings.load(path);

    if let Some(e) = loaded.err() {
        log::error!(target: "server_settings", "Error loading server settings: {e}");
        RuntimePlugin::stop();
    };
}
