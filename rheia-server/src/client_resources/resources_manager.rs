use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Resource;
use common::blocks::block_type::BlockContent;
use common::blocks::block_type::BlockType;
use common::default_resources::DEFAULT_RESOURCES;
use common::utils::calculate_hash;
use common::utils::split_resource_path;
use network::messages::ResurceScheme;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use zip::DateTime;

use crate::LaunchSettings;
use crate::network::runtime_plugin::RuntimePlugin;

use super::resource_instance::ResourceInstance;
use super::server_settings::ServerSettings;

pub const ARCHIVE_CHUNK_SIZE: usize = 1024 * 1024;

#[derive(Resource, Default)]
pub struct ResourceManager {
    resources: HashMap<String, ResourceInstance>,

    archive_data: Option<Vec<u8>>,
    archive_hash: Option<u64>,
    resources_scheme: Option<Vec<ResurceScheme>>,
}

impl ResourceManager {
    pub fn generate_resources_scheme(&mut self) {
        let mut schemes: Vec<ResurceScheme> = Default::default();

        for (slug, resource) in self.resources.iter() {
            let mut scheme = ResurceScheme {
                slug: slug.clone(),
                scripts: Default::default(),
                media: Default::default(),
            };
            for (script_slug, script_data) in resource.iter_scripts() {
                let hash = calculate_hash(&script_data);
                scheme.scripts.insert(hash.to_string(), script_slug.clone());
            }
            for (media_slug, media_data) in resource.iter_media() {
                let hash = calculate_hash(&media_data);
                scheme.media.insert(hash.to_string(), media_slug.clone());
            }
            schemes.push(scheme);
        }

        self.resources_scheme = Some(schemes);
    }

    pub fn get_resources_scheme(&self) -> &Vec<ResurceScheme> {
        self.resources_scheme.as_ref().unwrap()
    }

    pub fn get_archive_hash(&self) -> &u64 {
        self.archive_hash.as_ref().unwrap()
    }

    pub fn has_any_resources(&self) -> bool {
        for (_resource_slug, resource) in self.resources.iter() {
            if resource.get_scripts_count() > 0 {
                return true;
            }
            if resource.get_media_count() > 0 {
                return true;
            }
        }
        return false;
    }

    pub fn _get_media_count(&self) -> u32 {
        let mut count: u32 = 0;
        for (_slug, resource) in self.resources.iter() {
            count += resource.get_media_count() as u32;
        }
        return count;
    }

    pub fn has_media(&self, path: &String) -> Result<bool, String> {
        if DEFAULT_RESOURCES.contains(&path.as_str()) {
            return Ok(true);
        }

        let Some((res_slug, res_path)) = split_resource_path(path) else {
            return Err(format!("cannot split path \"{}\"", path));
        };

        let Some(resource) = self.resources.get(&res_slug) else {
            return Err(format!("resource \"{}\" not found", res_slug));
        };

        if !resource.has_media(&res_path) {
            return Err(format!(
                "resource \"{}\" doesn't contain media \"{}\"; total count: {}",
                res_slug,
                res_path,
                resource.media.len()
            ));
        }
        return Ok(true);
    }

    pub fn rescan_resources(&mut self, path: PathBuf, server_settings: &mut ServerSettings) -> Result<(), String> {
        let path_str = path.into_os_string().into_string().unwrap();
        log::info!(target: "resources", "▼ Rescan resources folders inside: &e{}", path_str);

        let resource_paths = match fs::read_dir(path_str.clone()) {
            Ok(p) => p,
            Err(e) => {
                return Err(format!("read directory &e\"{}\"&r error: &c{}", path_str, e));
            }
        };

        for resource_path in resource_paths {
            let resource_path = resource_path.unwrap().path();

            let mut manifest_path = resource_path.clone();
            manifest_path.push("manifest.yml");
            if !manifest_path.exists() {
                continue;
            }

            let resource_instance = match ResourceInstance::from_manifest(resource_path.clone()) {
                Ok(i) => i,
                Err(e) => {
                    return Err(format!("resource {}: {}", resource_path.display().to_string(), e));
                }
            };
            let resource_slug = resource_instance.get_slug().clone();

            if self.resources.contains_key(&resource_slug) {
                return Err(format!(
                    "&cresource &4\"{}\"&c slug &4\"{}\"&c already exists",
                    resource_path.display().to_string(),
                    resource_slug
                ));
            }

            let blocks = resource_instance.get_blocks();
            for block_type in blocks.iter() {
                server_settings.add_block(block_type.clone());
            }

            log::info!(
                target: "resources",
                "□ Resource &2\"{}\"&r loaded;&7 Title:\"{}\" v\"{}\" Author:\"{}\" Scripts:{} Media:{} Blocks:{}",
                resource_instance.get_slug(),
                resource_instance.get_title(),
                resource_instance.get_version(),
                resource_instance.get_autor(),
                resource_instance.get_scripts_count(),
                resource_instance.get_media_count(),
                blocks.len(),
            );
            self.add_resource(resource_instance.get_slug().clone(), resource_instance);

            if let Err(e) = self.validate_blocks(&blocks) {
                return Err(format!("resource &6\"{}\"&r: {}", resource_slug, e));
            }
        }
        log::info!(target: "resources", "All resources have been successfully loaded: {}", self.resources.len());
        Ok(())
    }

    pub fn validate_blocks(&self, blocks: &Vec<BlockType>) -> Result<(), String> {
        for block_type in blocks.iter() {
            match block_type.get_block_content() {
                BlockContent::Texture {
                    texture,
                    side_texture,
                    side_overlay,
                    bottom_texture,
                    ..
                } => {
                    if let Err(e) = self.has_media(texture) {
                        return Err(format!(
                            "&cblock &4\"{}\" &ctexture not found: &4\"{}\" &7({})",
                            block_type.get_slug(),
                            texture,
                            e,
                        ));
                    }
                    if side_texture.is_some() {
                        if let Err(e) = self.has_media(&side_texture.as_ref().unwrap()) {
                            return Err(format!(
                                "&cblock &4\"{}\" &cside_texture not found: &4\"{}\" &7({})",
                                block_type.get_slug(),
                                side_texture.as_ref().unwrap(),
                                e,
                            ));
                        }
                    }
                    if side_overlay.is_some() {
                        if let Err(e) = self.has_media(&side_overlay.as_ref().unwrap()) {
                            return Err(format!(
                                "&cblock &4\"{}\" &cside_overlay not found: &4\"{}\" &7({})",
                                block_type.get_slug(),
                                side_overlay.as_ref().unwrap(),
                                e,
                            ));
                        }
                    }
                    if bottom_texture.is_some() {
                        if let Err(e) = self.has_media(&bottom_texture.as_ref().unwrap()) {
                            return Err(format!(
                                "&cblock &4\"{}\" &cbottom_texture not found: &4\"{}\" &7({})",
                                block_type.get_slug(),
                                bottom_texture.as_ref().unwrap(),
                                e,
                            ));
                        }
                    }
                }
                BlockContent::ModelCube { model, .. } => {
                    if let Err(e) = self.has_media(model) {
                        return Err(format!(
                            "&cblock &4\"{}\" &cmodel not found: &4\"{}\" &7({})",
                            block_type.get_slug(),
                            model,
                            e,
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn add_resource(&mut self, slug: String, resource: ResourceInstance) {
        self.resources.insert(slug, resource);
    }

    pub fn generate_archive(&mut self) {
        let mut archive_data: Vec<u8> = Default::default();

        let buff = std::io::Cursor::new(&mut archive_data);
        let mut writer = zip::ZipWriter::new(buff);

        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .last_modified_time(DateTime::default());

        for (_resource_slug, resource) in self.resources.iter() {
            for (_scripts_slug, scripts_data) in resource.iter_scripts() {
                let hash = calculate_hash(&scripts_data);
                writer.start_file(hash.to_string(), options).unwrap();
                writer.write_all(scripts_data.as_bytes()).unwrap();
            }

            for (_media_slug, media_data) in resource.iter_media() {
                let hash = calculate_hash(&media_data);
                writer.start_file(hash.to_string(), options).unwrap();
                writer.write_all(media_data).unwrap();
            }
        }
        writer.finish().unwrap();

        self.archive_hash = Some(calculate_hash(&archive_data));
        self.archive_data = Some(archive_data);

        let parts_count = self.get_archive_parts_count(ARCHIVE_CHUNK_SIZE);
        log::info!(target: "resources", "Resources archive generated; archive parts count: {}", parts_count);
    }
    pub fn get_archive_len(&self) -> usize {
        self.archive_data.as_ref().unwrap().len()
    }

    pub fn get_archive_parts_count(&self, chunk_size: usize) -> usize {
        self.archive_data.as_ref().unwrap().len().div_ceil(chunk_size)
    }

    pub fn get_archive_part(&self, index: usize, chunk_size: usize) -> Vec<u8> {
        let parts_count = self.get_archive_parts_count(chunk_size);
        assert!(
            index < parts_count,
            "archive chunk index:{} must be less than max:{}",
            index,
            parts_count
        );

        let start = index * chunk_size;

        let mut end = (index + 1) * chunk_size;
        end = self.get_archive_len().min(end);

        let slice = &self.archive_data.as_ref().unwrap()[start..end];
        slice.to_vec()
    }
}

pub(crate) fn rescan_resources(
    mut resource_manager: ResMut<ResourceManager>,
    launch_settings: Res<LaunchSettings>,
    mut server_settings: ResMut<ServerSettings>,
) {
    if let Err(e) = resource_manager.rescan_resources(launch_settings.get_resources_path(), &mut *server_settings) {
        log::error!(target: "resources", "&cResources loading error:");
        log::error!(target: "resources", "{}", e);
        RuntimePlugin::stop();
        return;
    }
    resource_manager.generate_archive();
    resource_manager.generate_resources_scheme();
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use common::utils::calculate_hash;

    use super::ResourceManager;
    use crate::client_resources::{resource_instance::ResourceInstance, resources_manager::ARCHIVE_CHUNK_SIZE};

    #[test]
    fn test_archive() {
        let mut resource_manager = ResourceManager::default();

        let mut resource_instance = ResourceInstance::empty("test".to_string());

        let file_content = "content".to_string().into_bytes();
        resource_instance.add_media("example.glb".to_string(), file_content.clone());
        resource_manager.add_resource("test".to_string(), resource_instance);

        resource_manager.generate_archive();
        resource_manager.generate_resources_scheme();

        assert_eq!(resource_manager.archive_hash.unwrap(), 431488420107704094);

        let data = [
            80, 75, 3, 4, 10, 0, 0, 0, 0, 0, 0, 0, 33, 0, 169, 48, 197, 254, 7, 0, 0, 0, 7, 0, 0, 0, 19, 0, 0, 0, 52,
            52, 56, 57, 56, 49, 54, 50, 48, 57, 48, 48, 56, 50, 51, 52, 49, 57, 57, 99, 111, 110, 116, 101, 110, 116,
            80, 75, 1, 2, 10, 3, 10, 0, 0, 0, 0, 0, 0, 0, 33, 0, 169, 48, 197, 254, 7, 0, 0, 0, 7, 0, 0, 0, 19, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 164, 129, 0, 0, 0, 0, 52, 52, 56, 57, 56, 49, 54, 50, 48, 57, 48, 48, 56, 50,
            51, 52, 49, 57, 57, 80, 75, 5, 6, 0, 0, 0, 0, 1, 0, 1, 0, 65, 0, 0, 0, 56, 0, 0, 0, 0, 0,
        ];
        assert_eq!(*resource_manager.archive_data.as_ref().unwrap(), data);
        assert_eq!(resource_manager.get_archive_len(), 143);
        assert_eq!(resource_manager.get_archive_parts_count(50), 3);
        assert_eq!(
            resource_manager.get_archive_part(0, 50),
            [
                80, 75, 3, 4, 10, 0, 0, 0, 0, 0, 0, 0, 33, 0, 169, 48, 197, 254, 7, 0, 0, 0, 7, 0, 0, 0, 19, 0, 0, 0,
                52, 52, 56, 57, 56, 49, 54, 50, 48, 57, 48, 48, 56, 50, 51, 52, 49, 57, 57, 99
            ]
        );
        assert_eq!(
            resource_manager.get_archive_part(1, 50),
            [
                111, 110, 116, 101, 110, 116, 80, 75, 1, 2, 10, 3, 10, 0, 0, 0, 0, 0, 0, 0, 33, 0, 169, 48, 197, 254,
                7, 0, 0, 0, 7, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 164, 129, 0, 0
            ]
        );
        assert_eq!(
            resource_manager.get_archive_part(2, 50),
            [
                0, 0, 52, 52, 56, 57, 56, 49, 54, 50, 48, 57, 48, 48, 56, 50, 51, 52, 49, 57, 57, 80, 75, 5, 6, 0, 0,
                0, 0, 1, 0, 1, 0, 65, 0, 0, 0, 56, 0, 0, 0, 0, 0
            ]
        );

        let chunk = resource_manager.get_archive_part(0, ARCHIVE_CHUNK_SIZE);
        assert_eq!(*resource_manager.get_archive_hash(), calculate_hash(&chunk));

        let archive_data = resource_manager.archive_data.as_ref().unwrap();
        let file = std::io::Cursor::new(&archive_data);

        let mut zip = zip::ZipArchive::new(file).unwrap();
        for i in 0..zip.len() {
            let archive_file = zip.by_index(i).unwrap();
            let file_hash = archive_file.name().to_string();

            let mut archive_file_data = Vec::new();
            for i in archive_file.bytes() {
                archive_file_data.push(i.unwrap());
            }

            assert_eq!(archive_file_data, file_content);

            let hash = calculate_hash(&archive_file_data);
            assert_eq!(hash.to_string(), file_hash);
        }
    }
}
