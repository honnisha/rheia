use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GameSettings {
    pub ip_port_direct_connect: Option<String>,
    pub username: Option<String>,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            ip_port_direct_connect: None,
            username: None,
        }
    }
}

impl GameSettings {
    pub fn read() -> Result<Self, String> {
        let path = match GameSettings::get_settings_path() {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        if !path.exists() {
            return Ok(Self::default());
        }

        let settings_string = match std::fs::read_to_string(path.clone()) {
            Ok(d) => d,
            Err(e) => {
                return Err(format!("Settings file {} file error: {}", path.display(), e));
            }
        };

        let settings_result: Result<GameSettings, serde_yaml::Error> = serde_yaml::from_str(&settings_string);
        let settings = match settings_result {
            Ok(m) => m,
            Err(e) => {
                return Err(format!("Settings file \"{}\" yaml parse error: {}", path.display(), e));
            }
        };
        Ok(settings)
    }

    pub fn save(&self) -> Result<(), String> {
        let path = match GameSettings::get_settings_path() {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        if !path.exists() {
            let prefix = path.parent().unwrap();
            create_dir_all(prefix).unwrap();
        }
        let file = File::create(path.clone()).expect("File must exists");
        if let Err(e) = serde_yaml::to_writer(file, &self) {
            return Err(format!("Settings file \"{}\" yaml write error: {}", path.display(), e));
        }
        Ok(())
    }

    fn get_settings_path() -> Result<PathBuf, String> {
        let config_dir = match dirs_next::config_dir() {
            Some(c) => c,
            None => return Err("Error getting the path for the settings file".to_string()),
        };
        let mut config_path = PathBuf::from(config_dir);
        config_path.push("RheiaData");
        config_path.push("settings.yml");
        Ok(config_path)
    }
}
