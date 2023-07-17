use bevy_ecs::world::World;
use bracket_lib::random::RandomNumberGenerator;
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::console::console_sender::ConsoleSenderType;

use super::worlds_manager::WorldsManager;

pub(crate) fn get_command_parser() -> Command {
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

pub(crate) fn world_command(world: &mut World, sender: &dyn ConsoleSenderType, args: ArgMatches) {
    let mut worlds_manager = world.resource_mut::<WorldsManager>();
    match args.subcommand() {
        Some(("list", _)) => {
            if worlds_manager.count() == 0 {
                sender.send_console_message("Worlds list is empty".to_string());
                return;
            }
            let worlds = worlds_manager.get_worlds();
            sender.send_console_message("Worlds list:".to_string());
            for world in worlds.iter() {
                sender.send_console_message(format!(" - {}", world.get_slug()));
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
