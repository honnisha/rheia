use common::network::client::{ClientNetwork, NetworkInfo};
use common::network::messages::NetworkMessageType;
use common::{
    chunks::chunk_position::ChunkPosition,
    network::messages::{ClientMessages, ServerMessages},
};
use godot::obj::Gd;

use crate::console::console_handler::Console;
use crate::main_scene::Main;
use crate::utils::bridge::IntoGodotVector;
use crate::world::world_manager::WorldManager;

fn get_world_mut(main: &mut Main, world_slug: String) -> Option<&mut Gd<WorldManager>> {
    let worlds_manager = main.get_worlds_manager_mut();
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
    let lock = main.get_network_lock();
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
            ServerMessages::Resource { slug, scripts } => {
                let resource_manager = main.get_resource_manager_mut();
                log::info!(target: "network", "Start loading client resource slug:\"{}\"", slug);
                match resource_manager.try_load(&slug, scripts) {
                    Ok(_) => {
                        log::info!(target: "network", "Client resource slug:\"{}\" loaded", slug);
                    }
                    Err(e) => {
                        log::error!(target: "network", "Client resource slug:\"{}\" error: {}", slug, e);
                    }
                }
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
                if let Some(world) = get_world_mut(main, world_slug) {
                    world.bind_mut().load_chunk(chunk_position, sections);
                    chunks.push(chunk_position);
                }
            }
            ServerMessages::UnloadChunks { chunks, world_slug } => {
                if let Some(world) = get_world_mut(main, world_slug) {
                    for chunk_position in chunks.iter() {
                        world.bind_mut().unload_chunk(*chunk_position);
                    }
                }
            }
            ServerMessages::StartStreamingEntity {
                id,
                world_slug,
                position,
                rotation,
            } => {
                if let Some(world) = get_world_mut(main, world_slug) {
                    let mut w = world.bind_mut();
                    let mut entities_manager = w.get_entities_manager_mut();
                    entities_manager.create_entity(id, position.to_godot(), rotation);
                }
            }
            ServerMessages::EntityMove { world_slug, id, position, rotation } => {
                if let Some(world) = get_world_mut(main, world_slug) {
                    let mut w = world.bind_mut();
                    let mut entities_manager = w.get_entities_manager_mut();
                    entities_manager.move_entity(id, position.to_godot(), rotation);
                }
            }
            ServerMessages::StopStreamingEntities { world_slug, ids } => {
                if let Some(world) = get_world_mut(main, world_slug) {
                    let mut w = world.bind_mut();
                    let mut entities_manager = w.get_entities_manager_mut();
                    entities_manager.despawn(ids);
                }
            }
            ServerMessages::EditBlock { position, new_block_info } => {
                todo!();
            }
            _ => panic!("unsupported message"),
        }
    }

    if chunks.len() > 0 {
        let input = ClientMessages::ChunkRecieved {
            chunk_positions: chunks,
        };
        network.send_message(&input, NetworkMessageType::ReliableOrdered);
    }

    network_info
}
