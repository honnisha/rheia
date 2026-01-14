use common::chunks::chunk_position::ChunkPosition;
use godot::classes::{Engine, RenderingServer};
use godot::obj::{Gd, Singleton};
use network::client::{IClientNetwork, NetworkInfo};
use network::entities::EntityNetworkComponent;
use network::messages::{ClientMessages, NetworkMessageType, ServerMessages};

use crate::client_scripts::resource_manager::ResourceManager;
use crate::scenes::main_menu::VERSION;
use crate::scenes::main_scene::MainScene;
use crate::utils::bridge::{IntoChunkPositionVector, IntoGodotVector};
use crate::world::world_manager::WorldManager;
use crate::world::worlds_manager::WorldsManager;

fn get_world(worlds_manager: &WorldsManager, world_slug: String) -> Option<&Gd<WorldManager>> {
    let world = match worlds_manager.get_world() {
        Some(w) => w,
        None => {
            log::error!(target: "network", "Network message for non existed world");
            return None;
        }
    };
    if world_slug != *world.bind().get_slug() {
        log::error!(target: "network", "Network message for non wrong world {} != {}", world_slug, world.bind().get_slug());
        return None;
    }
    Some(world)
}

fn get_world_mut(worlds_manager: &mut WorldsManager, world_slug: String) -> Option<&mut Gd<WorldManager>> {
    let world = match worlds_manager.get_world_mut() {
        Some(w) => w,
        None => {
            log::error!(target: "network", "Network message for non existed world");
            return None;
        }
    };
    if world_slug != *world.bind().get_slug() {
        log::error!(target: "network", "Network message for non wrong world {} != {}", world_slug, world.bind().get_slug());
        return None;
    }
    Some(world)
}

pub fn handle_network_events(main: &mut MainScene) -> Result<NetworkInfo, String> {
    let container = main.get_network().expect("network is not set").clone();
    container.set_network_lock(false);

    let network = container.get_client();

    // Recieve errors from network thread
    for error in network.iter_errors() {
        return Err(error);
    }

    let mut chunks: Vec<ChunkPosition> = Default::default();

    // Recieve decoded server messages from network thread
    for event in network.iter_server_messages() {
        match event {
            ServerMessages::AllowConnection => {
                let device_name = match RenderingServer::singleton().get_rendering_device() {
                    Some(d) => d.get_device_name().to_string(),
                    None => String::from("-"),
                };
                let connection_info = ClientMessages::ConnectionInfo {
                    login: main.get_login().clone(),
                    version: VERSION.to_string(),
                    architecture: Engine::singleton().get_architecture_name().to_string(),
                    rendering_device: device_name,
                };
                log::info!(target: "network", "Server allowed connection");
                network.send_message(NetworkMessageType::ReliableOrdered, &connection_info);
            }
            ServerMessages::Disconnect { message } => {
                let msg = match message {
                    Some(m) => m,
                    None => "-".to_string(),
                };
                main.send_disconnect_event(format!("Disconnected by server: {}", msg));
                break;
            }

            ServerMessages::ConsoleOutput { message } => {
                log::info!(target: "network", "{}", message);
            }

            ServerMessages::ResourcesScheme { list, archive_hash } => {
                let mut resource_manager = main.get_resource_manager_mut();
                resource_manager.set_resource_scheme(list, archive_hash);
                let (scripts_count, media_count) = resource_manager.get_resource_scheme_count();
                log::info!(target: "network", "Network resources scheme loaded &e(scripts:{}, media:{}, archive_hash:{})", scripts_count, media_count, archive_hash);

                let has_saved = if ResourceManager::has_local_saved_resource(&archive_hash).unwrap() {
                    match resource_manager.load_local_archive(&archive_hash) {
                        Ok(count) => {
                            let mut resource_names: Vec<String> = Default::default();
                            for (resource_slug, resource) in resource_manager.get_resources_storage().iter() {
                                if resource.is_network() {
                                    resource_names.push(resource_slug.clone());
                                }
                            }
                            log::info!(target: "network", "Resources cache loaded: &e{}&r; media count:{}", resource_names.join(", "), count);
                        }
                        Err(e) => return Err(format!("Network resources cache load error: {}", e)),
                    }
                    true
                } else {
                    false
                };
                let msg = ClientMessages::ResourcesHasCache { exists: has_saved };
                network.send_message(NetworkMessageType::ReliableOrdered, &msg);
            }
            ServerMessages::ResourcesPart { index, total, mut data } => {
                {
                    let mut resource_manager = main.get_resource_manager_mut();
                    resource_manager.load_archive_chunk(&mut data);

                    let is_last = index + 1 >= total;
                    if is_last {
                        log::info!(target: "network", "Resource pack downloaded!");
                        resource_manager.check_archive_hash().unwrap();
                        let archive_hash = resource_manager.get_archive_hash().unwrap().clone();
                        let path = ResourceManager::get_saved_resource_path(&archive_hash).unwrap();
                        match resource_manager.save_resource_to_local() {
                            Ok(_) => {
                                log::info!(target: "network", "Resources archive saved locally: &6{}", path.display().to_string())
                            }
                            Err(e) => return Err(format!("Network resources local save error: {}", e)),
                        }
                        match resource_manager.load_local_archive(&archive_hash) {
                            Ok(_count) => {
                                let mut resource_names: Vec<String> = Default::default();
                                for (resource_slug, resource) in resource_manager.get_resources_storage().iter() {
                                    if resource.is_network() {
                                        resource_names.push(resource_slug.clone());
                                    }
                                }
                                log::info!(target: "network", "Resources loaded from network: &e{}", resource_names.join(", "));
                            }
                            Err(e) => return Err(format!("Network resources cache load error: {}", e)),
                        }
                    }
                }

                let msg = ClientMessages::ResourcesLoaded { last_index: index };
                network.send_message(NetworkMessageType::ReliableOrdered, &msg);

                main.get_text_screen_mut()
                    .update_text(format!("Media downloading {}/{}", index + 1, total));
            }
            ServerMessages::Settings {
                block_types,
                block_id_map,
            } => {
                log::info!(target: "network", "Recieved settings from the network");

                {
                    let mut worlds_manager = main.get_wm().clone();
                    let resource_manager = main.get_resource_manager();

                    {
                        let wm = worlds_manager.bind();
                        let mut block_storage = wm.get_block_storage_mut();

                        block_storage.set_block_id_map(block_id_map);
                        log::info!(target: "network", "Block id map is set");

                        if let Err(e) =
                            block_storage.load_blocks_types(block_types, &*resource_manager.get_resources_storage())
                        {
                            return Err(e);
                        }
                    }

                    if let Err(e) = worlds_manager
                        .bind_mut()
                        .build_textures(&*&resource_manager.get_resources_storage())
                    {
                        return Err(e);
                    }

                    network.send_message(NetworkMessageType::ReliableOrdered, &ClientMessages::SettingsLoaded);
                }

                main.on_server_connected();
            }

            ServerMessages::SpawnWorld { world_slug } => {
                main.spawn_world(world_slug);
                main.get_text_screen_mut().toggle(false);
            }
            ServerMessages::UpdatePlayerComponent { component } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(player_controller) = worlds_manager.get_player_controller_mut() else {
                    log::error!(target: "network", "network tried to update skin with non existing world");
                    continue;
                };
                match component {
                    EntityNetworkComponent::Tag(_tag) => (),
                    EntityNetworkComponent::Skin(skin) => player_controller.bind_mut().update_skin(skin),
                }
            }
            ServerMessages::PlayerSpawn {
                world_slug,
                position,
                rotation,
                components,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(_world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                let Some(player_controller) = worlds_manager.get_player_controller_mut().as_mut() else {
                    log::error!(target: "network", "network tried to teleport with non existing world");
                    continue;
                };
                player_controller.bind_mut().set_position(position.to_godot());
                player_controller.bind_mut().set_rotation(rotation);

                for component in components {
                    match component {
                        EntityNetworkComponent::Tag(_tag) => (),
                        EntityNetworkComponent::Skin(skin) => player_controller.bind_mut().update_skin(skin),
                    }
                }
            }
            ServerMessages::ChunkSectionInfoEncoded { .. } => {
                panic!("ChunkSectionInfoEncoded must be decoded");
            }
            ServerMessages::ChunkSectionInfo {
                world_slug,
                chunk_position,
                sections,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();

                let center = match worlds_manager.get_player_controller() {
                    Some(c) => c.bind().get_position().to_chunk_position(),
                    None => ChunkPosition::zero(),
                };

                let Some(world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                world.bind_mut().recieve_chunk(center, chunk_position, sections);
                chunks.push(chunk_position);
            }
            ServerMessages::UnloadChunks { chunks, world_slug } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                for chunk_position in chunks.iter() {
                    world.bind_mut().unload_chunk(*chunk_position);
                }
            }
            ServerMessages::StartStreamingEntity {
                id,
                world_slug,
                position,
                rotation,
                components,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                let mut w = world.bind_mut();
                let mut entities_manager = w.get_entities_manager_mut();
                entities_manager.create_entity(id, position.to_godot(), rotation, components);
            }
            ServerMessages::UpdateEntityComponent {
                world_slug,
                id,
                component,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                let mut w = world.bind_mut();
                let mut entities_manager = w.get_entities_manager_mut();
                match component {
                    EntityNetworkComponent::Tag(c) => entities_manager.update_entity_tag(id, c),
                    EntityNetworkComponent::Skin(c) => entities_manager.update_entity_skin(id, c.unwrap()),
                }
            }
            ServerMessages::EntityMove {
                world_slug,
                id,
                position,
                rotation,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                let mut w = world.bind_mut();
                let mut entities_manager = w.get_entities_manager_mut();
                entities_manager.move_entity(id, position.to_godot(), rotation);
            }
            ServerMessages::StopStreamingEntities { world_slug, ids } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                let mut w = world.bind_mut();
                let mut entities_manager = w.get_entities_manager_mut();
                entities_manager.despawn(ids);
            }
            ServerMessages::EditBlock {
                world_slug,
                position,
                new_block_info,
            } => {
                let worlds_manager = main.get_wm().bind();
                let Some(world) = get_world(&worlds_manager, world_slug) else {
                    continue;
                };
                let block_storage = worlds_manager.get_block_storage();
                let resource_manager = main.get_resource_manager();
                let resources_storage = resource_manager.get_resources_storage();
                world
                    .bind()
                    .edit_block(position, &block_storage, new_block_info, &*resources_storage)
                    .unwrap();
            }
        }
    }

    if chunks.len() > 0 {
        let input = ClientMessages::ChunkRecieved {
            chunk_positions: chunks,
        };
        network.send_message(NetworkMessageType::WorldInfo, &input);
    }

    let network_info = network.get_network_info().clone();
    Ok(network_info)
}
