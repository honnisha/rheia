use common::chunks::chunk_position::ChunkPosition;
use godot::obj::Gd;
use network::client::{IClientNetwork, NetworkInfo};
use network::messages::NetworkMessageType;
use network::messages::{ClientMessages, ServerMessages};

use crate::console::console_handler::Console;
use crate::main_scene::Main;
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

pub fn handle_network_events(main: &mut Main) -> NetworkInfo {
    let lock = main.get_network_lock().expect("network is not set");
    let network = lock.read();
    let network_info = network.get_network_info().clone();

    // Recieve errors from network thread
    for error in network.iter_errors() {
        log::error!(target: "network", "Network error: {}", error);
        Main::close();
    }

    for command in Console::iter_console_input() {
        let message = ClientMessages::ConsoleInput { command };
        network.send_message(&message, NetworkMessageType::ReliableOrdered);
    }

    let mut chunks: Vec<ChunkPosition> = Default::default();

    // Recieve decoded server messages from network thread
    for event in network.iter_server_messages() {
        match event {
            ServerMessages::AllowConnection => {
                let connection_info = ClientMessages::ConnectionInfo {
                    login: "Test_cl".to_string(),
                };
                network.send_message(&connection_info, NetworkMessageType::ReliableOrdered);
            }
            ServerMessages::ConsoleOutput { message } => {
                log::info!(target: "network", "{}", message);
            }

            ServerMessages::ResourcesScheme { list } => {
                let resource_manager = main.get_resource_manager_mut();
                resource_manager.set_resource_scheme(list);
                log::info!(target: "network", "Resources scheme loaded from network");
            }
            ServerMessages::ResourcesPart { index, mut data, last } => {
                let resource_manager = main.get_resource_manager_mut();
                resource_manager.load_archive_chunk(&mut data);

                if last {
                    resource_manager.load_archive().unwrap();
                    log::info!(target: "network", "Resources archive installed from network; index:{}", index);
                }

                let msg = ClientMessages::ResourcesLoaded { last_index: index };
                network.send_message(&msg, NetworkMessageType::ReliableOrdered);
            }
            ServerMessages::Settings { block_types } => {
                log::info!(target: "network", "Recieved settings from the network");

                let mut worlds_manager = main.get_worlds_manager_mut();
                let resource_manager = main.get_resource_manager();

                {
                    let mut block_storage = worlds_manager.get_block_storage_mut();
                    block_storage
                        .load_blocks_types(block_types, &*resource_manager)
                        .unwrap();
                }

                worlds_manager.build_textures(&*resource_manager).unwrap();

                network.send_message(&ClientMessages::SettingsLoaded, NetworkMessageType::ReliableOrdered);
            }

            ServerMessages::Teleport {
                world_slug,
                position,
                rotation,
            } => {
                main.get_worlds_manager_mut()
                    .teleport_player(world_slug, position.to_godot(), rotation);
            }
            ServerMessages::ChunkSectionInfo {
                world_slug,
                chunk_position,
                sections,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                if let Some(world) = get_world_mut(&mut worlds_manager, world_slug) {
                    world
                        .bind_mut()
                        .get_chunk_map_mut()
                        .load_chunk(chunk_position, sections);
                    chunks.push(chunk_position);
                }
            }
            ServerMessages::UnloadChunks { chunks, world_slug } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                if let Some(world) = get_world_mut(&mut worlds_manager, world_slug) {
                    for chunk_position in chunks.iter() {
                        world.bind_mut().get_chunk_map_mut().unload_chunk(*chunk_position);
                    }
                }
            }
            ServerMessages::StartStreamingEntity {
                id,
                world_slug,
                position,
                rotation,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                if let Some(world) = get_world_mut(&mut worlds_manager, world_slug) {
                    let mut w = world.bind_mut();
                    let mut entities_manager = w.get_entities_manager_mut();
                    entities_manager.create_entity(id, position.to_godot(), rotation);
                }
            }
            ServerMessages::EntityMove {
                world_slug,
                id,
                position,
                rotation,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                if let Some(world) = get_world_mut(&mut worlds_manager, world_slug) {
                    let mut w = world.bind_mut();
                    let mut entities_manager = w.get_entities_manager_mut();
                    entities_manager.move_entity(id, position.to_godot(), rotation);
                }
            }
            ServerMessages::StopStreamingEntities { world_slug, ids } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                if let Some(world) = get_world_mut(&mut worlds_manager, world_slug) {
                    let mut w = world.bind_mut();
                    let mut entities_manager = w.get_entities_manager_mut();
                    entities_manager.despawn(ids);
                }
            }
            ServerMessages::EditBlock {
                world_slug,
                position,
                new_block_info,
            } => {
                let mut worlds_manager = main.get_worlds_manager_mut();
                if let Some(world) = get_world_mut(&mut worlds_manager, world_slug) {
                    let mut w = world.bind_mut();
                    w.get_chunk_map_mut().edit_block(position, new_block_info);
                }
            }
        }
    }

    if chunks.len() > 0 {
        let input = ClientMessages::ChunkRecieved {
            chunk_positions: chunks,
        };
        network.send_message(&input, NetworkMessageType::WorldInfo);
    }

    network_info
}
