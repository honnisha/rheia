use bevy_ecs::world::World;
use bracket_lib::random::RandomNumberGenerator;

use crate::console::command::{Arg, ArgMatches, Command};
use crate::console::commands_executer::CommandError;
use crate::console::console_sender::ConsoleSenderType;
use crate::entities::entity::{Position, Rotation};
use crate::network::client_network::ClientNetwork;

use super::worlds_manager::WorldsManager;

pub(crate) fn command_parser_world() -> Command {
    Command::new("world".to_string()).subcommand_required(true).subcommand(
        Command::new("list".to_owned())
            .subcommand(Command::new("create".to_owned()).arg(Arg::new("slug".to_owned()).required(true))),
    )
}

pub(crate) fn command_world(
    world: &mut World,
    sender: Box<dyn ConsoleSenderType>,
    args: ArgMatches,
) -> Result<(), CommandError> {
    let mut worlds_manager = world.resource_mut::<WorldsManager>();
    match args.subcommand() {
        Some(("list", _)) => {
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
        Some(("create", create_matches)) => {
            let slug = create_matches.get_arg::<String>("slug".to_owned()).unwrap();
            let mut rng = RandomNumberGenerator::new();
            let seed = rng.next_u64();
            match worlds_manager.create_world(slug.clone(), seed) {
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
    args: ArgMatches,
) -> Result<(), CommandError> {
    let worlds_manager = world.resource_mut::<WorldsManager>();

    let client = match sender.as_any().downcast_ref::<ClientNetwork>() {
        Some(c) => c,
        None => {
            sender.send_console_message("This command is allowed to be used only for players".to_string());
            return Ok(());
        }
    };
    let x = args.get_arg::<f32>("x".to_owned()).unwrap().clone();
    let y = args.get_arg::<f32>("y".to_owned()).unwrap().clone();
    let z = args.get_arg::<f32>("z".to_owned()).unwrap().clone();

    let position = Position::new(x, y, z);
    let rotation = Rotation::new(0.0, 0.0);

    let world_entity = client.get_world_entity();
    match world_entity {
        Some(world_entity) => {
            let mut world_manager = worlds_manager
                .get_world_manager_mut(&world_entity.get_world_slug())
                .unwrap();
            let (chunk_changed, abandoned_chunks) =
                world_manager.player_move(&world_entity, position.clone(), rotation.clone());

            if chunk_changed {
                let world_slug = world_entity.get_world_slug().clone();
                client.send_unload_chunks(&world_slug, abandoned_chunks);
            }
        }
        None => todo!(),
    }

    client.network_send_teleport(&position, &rotation);
    return Ok(());
}
