use bevy_ecs::world::World;
use bracket_lib::random::RandomNumberGenerator;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::console::commands_executer::CommandError;
use crate::console::console_sender::ConsoleSenderType;
use crate::entities::entity::{Position, Rotation};
use crate::network::client_network::ClientNetwork;
use crate::network::server::NetworkContainer;

use super::worlds_manager::WorldsManager;

pub(crate) fn command_parser_world() -> Command {
    Command::new("world")
        .subcommand_required(true)
        .subcommand(Command::new("list").short_flag('l').long_flag("list"))
        .subcommand(
            Command::new("create").short_flag('c').long_flag("create").arg(
                Arg::new("slug")
                    .short('s')
                    .long("slug")
                    .help("slug of the new world")
                    .required(true)
                    .action(ArgAction::Set),
            ),
        )
}

pub(crate) fn command_world(world: &mut World, sender: Box<dyn ConsoleSenderType>, args: ArgMatches) -> Result<(), CommandError> {
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
            let slug = create_matches.get_one::<String>("slug").unwrap();
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
    Command::new("tp")
        .arg(
            Arg::new("x")
                .required(true)
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(f32)),
        )
        .arg(
            Arg::new("y")
                .required(true)
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(f32)),
        )
        .arg(
            Arg::new("z")
                .required(true)
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(f32)),
        )
}

pub(crate) fn command_teleport(world: &mut World, sender: Box<dyn ConsoleSenderType>, args: ArgMatches) -> Result<(), CommandError> {
    let worlds_manager = world.resource::<WorldsManager>();
    let network_container = world.resource::<NetworkContainer>();

    let client = match sender.as_any().downcast_ref::<ClientNetwork>() {
        Some(c) => c,
        None => {
            sender.send_console_message("This command is allowed to be used only for players".to_string());
            return Ok(());
        }
    };
    let x = args.get_one::<f32>("x").unwrap().clone();
    let y = args.get_one::<f32>("y").unwrap().clone();
    let z = args.get_one::<f32>("z").unwrap().clone();

    let position = Position::new(x, y, z);
    let rotation = Rotation::new(0.0, 0.0);

    let world_entity = client.get_world_entity().clone();
    match world_entity {
        Some(world_entity) => {
            let world_manager = worlds_manager
                .get_world_manager(&world_entity.get_world_slug())
                .unwrap();
            let (chunk_changed, abandoned_chunks) =
                world_manager.player_move(&world_entity, position.clone(), rotation.clone());

            if chunk_changed {
                let world_slug = world_entity.get_world_slug().clone();
                client.send_unload_chunks(&network_container, &world_slug, abandoned_chunks);
            }
        }
        None => todo!(),
    }

    client.network_send_teleport(&network_container, &position, &rotation);
    return Ok(());
}
