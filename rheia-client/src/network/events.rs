use common::chunks::chunk_position::ChunkPosition;
use godot::classes::{Engine, RenderingServer};
use godot::obj::Gd;
use network::client::{IClientNetwork, NetworkInfo};
use network::messages::NetworkMessageType;
use network::messages::{ClientMessages, ServerMessages};

use crate::console::console_handler::Console;
use crate::scenes::main_menu::VERSION;
use crate::scenes::main_scene::MainScene;
use crate::utils::bridge::IntoGodotVector;
use crate::world::world_manager::WorldManager;
use crate::world::worlds_manager::WorldsManager;

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
    let lock = main.get_network_lock().expect("network is not set");
    let network = lock.read();

    // Recieve errors from network thread
    for error in network.iter_errors() {
        log::error!(target: "network", "Network error: {}", error);
        return Err(error);
    }

    for command in Console::iter_console_input() {
        let message = ClientMessages::ConsoleInput { command };
        network.send_message(NetworkMessageType::ReliableOrdered, &message);
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
                network.send_message(NetworkMessageType::ReliableOrdered, &connection_info);
            }
            ServerMessages::ConsoleOutput { message } => {
                log::info!(target: "network", "{}", message);
            }

            ServerMessages::ResourcesScheme { list, archive_hash } => {
                let resource_manager = main.get_resource_manager_mut();
                resource_manager.set_resource_scheme(list, archive_hash);
                log::info!(target: "network", "Resources scheme loaded from network");
            }
            ServerMessages::ResourcesPart {
                index,
                total,
                mut data,
                last,
            } => {
                let resource_manager = main.get_resource_manager_mut();
                resource_manager.load_archive_chunk(&mut data);

                if last {
                    if let Err(e) = resource_manager.load_archive() {
                        return Err(format!("Network resources download error: {}", e));
                    }
                    log::info!(target: "network", "Resources archive installed from network; index:{}", index);
                }

                let msg = ClientMessages::ResourcesLoaded { last_index: index };
                network.send_message(NetworkMessageType::ReliableOrdered, &msg);

                main.get_text_screen_mut()
                    .update_text(format!("Media downloaded {}/{}", index + 1, total));
            }
            ServerMessages::Settings { block_types } => {
                log::info!(target: "network", "Recieved settings from the network");

                {
                    let mut worlds_manager = main.get_worlds_manager_mut();
                    let resource_manager = main.get_resource_manager();

                    {
                        let mut block_storage = worlds_manager.get_block_storage_mut();
                        if let Err(e) = block_storage.load_blocks_types(block_types, &*resource_manager) {
                            return Err(e);
                        }
                    }

                    if let Err(e) = worlds_manager.build_textures(&*resource_manager) {
                        return Err(e);
                    }

                    network.send_message(NetworkMessageType::ReliableOrdered, &ClientMessages::SettingsLoaded);
                }

                main.on_server_connected();
            }

            ServerMessages::Teleport {
                world_slug,
                position,
                rotation,
            } => {
                main.get_worlds_manager_mut().teleport_player(
                    world_slug,
                    position.to_godot(),
                    rotation,
                    main.get_network_lock().unwrap(),
                );

                main.get_text_screen_mut().toggle(false);
            }
            ServerMessages::ChunkSectionInfo {
                world_slug,
                chunk_position,
                sections,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                world.bind_mut().recieve_chunk(chunk_position, sections);
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
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                let mut w = world.bind_mut();
                let mut entities_manager = w.get_entities_manager_mut();
                entities_manager.create_entity(id, position.to_godot(), rotation);
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
                let mut worlds_manager = main.get_worlds_manager_mut();
                let Some(world) = get_world_mut(&mut worlds_manager, world_slug) else {
                    continue;
                };
                world.bind_mut().edit_block(position, new_block_info);
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
