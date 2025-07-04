use crate::client_resources::server_settings::ServerSettings;
use crate::console::console_sender::ConsoleSenderType;
use crate::entities::entity::{Position, Rotation};
use crate::launch_settings::LaunchSettings;
use crate::network::client_network::ClientNetwork;
use crate::network::events::on_player_move::move_player;
use bevy_ecs::world::World;
use bracket_lib::random::RandomNumberGenerator;
use common::commands::command::{Arg, Command, CommandMatch};
use common::world_generator::default::WorldGeneratorSettings;

use super::worlds_manager::WorldsManager;

pub(crate) fn command_parser_world() -> Command {
    Command::new("world".to_string())
        .subcommand_required(true)
        .subcommand(Command::new("list".to_owned()))
        .subcommand(
            Command::new("create".to_owned())
                .arg(Arg::new("slug".to_owned()).required(true))
                .arg(Arg::new("seed".to_owned())),
        )
}

pub(crate) fn command_world(
    world: &mut World,
    sender: Box<dyn ConsoleSenderType>,
    args: CommandMatch,
) -> Result<(), String> {
    let launch_settings = world.get_resource::<LaunchSettings>().unwrap();
    let world_storage_settings = launch_settings.get_world_storage_settings();

    let server_settings = world.get_resource::<ServerSettings>().unwrap();
    let block_id_map = server_settings.get_block_id_map().clone();

    let mut worlds_manager = world.resource_mut::<WorldsManager>();

    if let Some(world_subcommand) = args.subcommand() {
        match world_subcommand.get_name().as_str() {
            "list" => {
                if worlds_manager.count() == 0 {
                    sender.send_console_message("Worlds list is empty".to_string());
                    return Ok(());
                }
                let worlds = worlds_manager.get_worlds();
                sender.send_console_message("Worlds list:".to_string());
                for (_slug, world) in worlds.iter() {
                    let world = world.read();
                    sender.send_console_message(format!(
                        " - {} (loaded chunks: {})",
                        world.get_slug(),
                        world.get_chunks_count()
                    ));
                }
            }
            "create" => {
                let slug = world_subcommand.get_arg::<String, _>("slug")?;
                if slug.len() == 0 {
                    sender.send_console_message(format!("Name of the world cannot be empty"));
                    return Ok(());
                }

                let seed = match world_subcommand.get_arg::<u64, _>("slug") {
                    Ok(s) => s,
                    Err(_) => {
                        let mut rng = RandomNumberGenerator::new();
                        rng.next_u64()
                    }
                };
                let world = worlds_manager.create_world(
                    slug.clone(),
                    seed,
                    WorldGeneratorSettings::default(),
                    &world_storage_settings,
                    &block_id_map,
                );
                match world {
                    Ok(_) => {
                        sender.send_console_message(format!("World \"{}\" was successfully created", slug));
                    }
                    Err(e) => {
                        sender.send_console_message(format!("World \"{}\" creation error: {}", slug, e));
                    }
                }
            }
            _ => {
                sender.send_console_message("Error".to_string());
            }
        }
    }
    return Ok(());
}

pub(crate) fn command_parser_teleport() -> Command {
    Command::new("tp".to_owned())
        .arg(Arg::new("x".to_owned()).required(true))
        .arg(Arg::new("y".to_owned()).required(true))
        .arg(Arg::new("z".to_owned()).required(true))
}

pub(crate) fn command_teleport(
    world: &mut World,
    sender: Box<dyn ConsoleSenderType>,
    args: CommandMatch,
) -> Result<(), String> {
    let x = args.get_arg::<f32, _>("x")?.clone();
    let y = args.get_arg::<f32, _>("y")?.clone();
    let z = args.get_arg::<f32, _>("z")?.clone();

    let worlds_manager = world.resource::<WorldsManager>();

    let client = match sender.as_any().downcast_ref::<ClientNetwork>() {
        Some(c) => c,
        None => {
            sender.send_console_message("This command is allowed to be used only for players".to_string());
            return Ok(());
        }
    };

    let position = Position::new(x, y, z);
    let rotation = Rotation::new(0.0, 0.0);

    let Some(world_entity) = client.get_world_entity() else {
        sender.send_console_message(format!(
            "Player \"{}\" is not in the world",
            client.get_client_info().unwrap().get_login()
        ));
        return Ok(());
    };

    let mut world_manager = worlds_manager
        .get_world_manager_mut(&world_entity.get_world_slug())
        .unwrap();

    move_player(&mut *world_manager, &world_entity, position, rotation);
    return Ok(());
}
