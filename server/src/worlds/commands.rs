use bevy_ecs::world::World;
use bracket_lib::random::RandomNumberGenerator;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::entities::entity::{Position, Rotation};
use crate::network::client_network::ClientNetwork;
use crate::network::server::NetworkContainer;
use crate::{console::console_sender::ConsoleSenderType, network::clients_container::ClientsContainer};

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

pub(crate) fn command_world(world: &mut World, sender: Box<dyn ConsoleSenderType>, args: ArgMatches) {
    let mut worlds_manager = world.resource_mut::<WorldsManager>();
    match args.subcommand() {
        Some(("list", _)) => {
            if worlds_manager.count() == 0 {
                sender.send_console_message("Worlds list is empty".to_string());
                return;
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

pub(crate) fn command_teleport(world: &mut World, sender: Box<dyn ConsoleSenderType>, args: ArgMatches) {
    let mut worlds_manager = world.resource_mut::<WorldsManager>();
    let clients = world.resource::<ClientsContainer>();
    let network_container = world.resource::<NetworkContainer>();

    let client = match sender.as_any().downcast_ref::<ClientNetwork>() {
        Some(c) => c,
        None => {
            sender.send_console_message("Only player call allowed".to_string());
            return;
        },
    };
    let x: f32 = args.get_one::<String>("x").unwrap().parse().unwrap();
    let y: f32 = args.get_one::<String>("y").unwrap().parse().unwrap();
    let z: f32 = args.get_one::<String>("z").unwrap().parse().unwrap();

    let position = Position::new(x, y, z);
    let rotation = Rotation::new(0.0, 0.0);

    let world_entity_lock = client.get_world_entity();
    match world_entity_lock.as_ref() {
        Some(world_entity) => {
            con ti nue
        },
        None => todo!(),
    }

    client.network_send_teleport(&network_container, &position, &rotation);
}
