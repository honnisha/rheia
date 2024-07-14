use common::network::client::{ClientNetwork, NetworkInfo};
use common::network::messages::NetworkMessageType;
use common::{
    chunks::chunk_position::ChunkPosition,
    network::messages::{ClientMessages, ServerMessages},
};

use crate::console::console_handler::Console;
use crate::main_scene::Main;
use crate::utils::bridge::IntoGodotVector;

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
                location,
                yaw,
                pitch,
            } => {
                main.get_world_manager_mut()
                    .teleport_player(world_slug, location.to_godot(), yaw, pitch);
            }
            ServerMessages::ChunkSectionInfo {
                world_slug,
                chunk_position,
                sections,
            } => {
                let world_manager = main.get_world_manager_mut();
                let world = match world_manager.get_world_mut() {
                    Some(w) => w,
                    None => {
                        log::error!(target: "network", "load_chunk tried to run without a world");
                        continue;
                    }
                };
                if world_slug != *world.bind().get_slug() {
                    log::error!(
                        target: "network",
                        "Tried to load chunk {} for non existed world {}",
                        chunk_position, world_slug
                    );
                    continue;
                }
                world.bind_mut().load_chunk(chunk_position, sections);
                chunks.push(chunk_position);
            }
            ServerMessages::UnloadChunks { chunks, world_slug } => {
                let world_manager = main.get_world_manager_mut();
                let world = match world_manager.get_world_mut() {
                    Some(w) => w,
                    None => {
                        log::error!(target: "network", "load_chunk tried to run without a world");
                        continue;
                    }
                };
                if world_slug != *world.bind().get_slug() {
                    log::error!(target: "network", "Tried to unload chunks for non existed world {}", world_slug);
                    continue;
                }
                world.bind_mut().unload_chunk(chunks);
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
