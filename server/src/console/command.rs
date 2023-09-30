use std::str::FromStr;

use ahash::HashMap;

use super::commands_executer::CommandError;

#[derive(Clone)]
pub struct Command {
    name: String,
    subcommand_required: bool,
    args: Vec<Arg>,
    commands: Vec<Command>,
}

type DurrenctType<'a> = Option<(&'a Command, Option<&'a Arg>)>;

impl Command {
    pub fn new(name: String) -> Self {
        Self {
            name,
            subcommand_required: false,
            args: Default::default(),
            commands: Default::default(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn subcommand_required(mut self, required: bool) -> Self {
        self.subcommand_required = required;
        self
    }

    /// command_sequence example:
    /// "world"  "create"    "test"      "123"
    /// ^command ^subcommand ^arg        ^arg optional
    ///  name     name        world name  seed
    pub fn eval(&self, command_sequence: &[String]) -> Result<CommandMatch, CommandError> {
        let mut args: HashMap<String, String> = Default::default();
        if command_sequence.len() == 0 {
            return Ok(CommandMatch::new(self.name.clone(), None, args));
        }

        let mut subcommand: Option<Box<CommandMatch>> = None;
        for command in self.commands.iter() {
            // if command matches
            // println!("command.name:{} command_sequence[0]:{}", command.name, command_sequence[0]);
            if command.name == command_sequence[0] {
                let command_match = command.eval(&command_sequence[1..])?;
                subcommand = Some(Box::new(command_match));
                break;
            }
        }

        if subcommand.is_none() && self.subcommand_required {
            return Err(CommandError(format!("subcommand for \"{}\" is required", self.name)));
        }

        let mut i = 0;
        for arg in self.args.iter() {
            let mut arg_value: Option<String> = None;
            if command_sequence.len() > i {
                arg_value = Some(command_sequence[i].clone());
            }
            match arg_value {
                Some(v) => {
                    // println!(
                    //     "arg.name:{} v:{} i:{} command_sequence:{:?}",
                    //     arg.name, v, i, command_sequence
                    // );
                    args.insert(arg.name.clone(), v);
                }
                None => {
                    if arg.required {
                        return Err(CommandError(format!("argument \"{}\" is required", arg.name)));
                    }
                }
            }
            i += 1;
        }

        return Ok(CommandMatch::new(self.name.clone(), subcommand, args));
    }

    /// command_sequence example:
    /// "world"  "create"    "test"      "123"
    /// ^command ^subcommand ^arg        ^arg optional
    ///  name     name        world name  seed
    pub fn get_current(&self, command_sequence: &[String]) -> DurrenctType {
        if command_sequence.len() > 0 {
            for c in self.commands.iter() {
                if self.name != command_sequence[0] {
                    continue;
                }
                if let Some(t) = Command::get_current(c, &command_sequence[1..]) {
                    return Some(t);
                }
            }
        }

        if self.subcommand_required || command_sequence.len() == 0 {
            return Some((self, None));
        }

        // If command arg count is less than provided args
        if self.args.len() < command_sequence.len() {
            return None;
        }

        println!("command_sequence:{:?}", command_sequence);
        Some((self, Some(&self.args[command_sequence.len() - 1])))
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }

    pub fn subcommand(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }
}

#[derive(Clone)]
pub struct Arg {
    name: String,
    required: bool,
}

impl Arg {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            required: false,
        }
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

#[derive(Clone)]
pub struct CommandMatch {
    name: String,
    subcommand: Option<Box<CommandMatch>>,
    args: HashMap<String, String>,
}

impl CommandMatch {
    pub fn new(name: String, subcommand: Option<Box<CommandMatch>>, args: HashMap<String, String>) -> Self {
        Self { name, subcommand, args }
    }

    pub fn get_arg<T: FromStr>(&self, arg_name: &String) -> Result<T, CommandError> {
        match self.args.get(arg_name) {
            Some(a) => match a.parse::<T>() {
                Ok(v) => Ok(v),
                Err(_e) => Err(CommandError(format!("parameter {} converting error", self.name))),
            },
            None => Err(CommandError("parameter has not been provided".to_string())),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn subcommand(&self) -> &Option<Box<CommandMatch>> {
        &self.subcommand
    }
}

#[cfg(test)]
mod tests {
    use crate::console::{command::Arg, commands_executer::CommandsHandler};

    use super::Command;

    fn world_command() -> Command {
        Command::new("world".to_string())
            .subcommand_required(true)
            .subcommand(Command::new("list".to_owned()))
            .subcommand(
                Command::new("create".to_owned())
                    .arg(Arg::new("slug".to_owned()).required(true))
                    .arg(Arg::new("seed".to_owned())),
            )
    }

    #[test]
    fn test_command_eval() {
        let command = world_command();

        let cmd = "world create test 123".to_string();
        let command_sequence = CommandsHandler::parse_command(&cmd);
        let result = command.eval(&command_sequence[1..]);

        assert_eq!(result.is_ok(), true, "error: {}", result.err().unwrap());
        assert_eq!(result.as_ref().unwrap().get_name(), &"world".to_string());

        let subcommand = result.as_ref().unwrap().subcommand();
        assert_eq!(subcommand.is_some(), true, "World create subcommand must be presented");
        let s = subcommand.as_ref().unwrap();
        assert_eq!(s.get_name(), "create");

        let slug = s.get_arg::<String>(&"slug".to_owned());
        assert_eq!(slug.is_ok(), true, "error: {}", slug.err().unwrap());
        assert_eq!(slug.as_ref().unwrap(), &"test".to_string());
    }

    #[test]
    fn test_command_eval_error() {
        let command = world_command();

        let cmd = "world test".to_string();
        let command_sequence = CommandsHandler::parse_command(&cmd);
        let result = command.eval(&command_sequence[1..]);

        assert_eq!(result.is_err(), true);
        assert_eq!(
            result.err().unwrap().to_string(),
            "subcommand for \"world\" is required".to_string()
        );
    }

    #[test]
    fn test_command_current_world() {
        let command = world_command();

        let cmd = "world ".to_string();
        let command_sequence = CommandsHandler::parse_command(&cmd);
        let result = command.get_current(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "world".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_none(), true);
    }

    fn tp_command() -> Command {
        Command::new("tp".to_owned())
            .arg(Arg::new("x".to_owned()).required(true))
            .arg(Arg::new("y".to_owned()).required(true))
            .arg(Arg::new("z".to_owned()).required(true))
            .arg(Arg::new("username".to_owned()))
    }

    #[test]
    fn test_command_current_tp() {
        let command = tp_command();

        let cmd = "tp ".to_string();
        let command_sequence = CommandsHandler::parse_command(&cmd);
        let result = command.get_current(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "tp".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_some(), true);
        assert_eq!(result.as_ref().unwrap().1.as_ref().unwrap().name, "x".to_string());
    }

    #[test]
    fn test_parse_command() {
        let command_sequence = CommandsHandler::parse_command(&"tp ".to_string());
        assert_eq!(command_sequence, vec!["tp", ""]);
    }

    #[test]
    fn test_command_current_tp_root() {
        let command = tp_command();

        let cmd = "tp".to_string();
        let command_sequence = CommandsHandler::parse_command(&cmd);
        let result = command.get_current(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "tp".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_none(), true);
    }

    #[test]
    fn test_command_current_tp_last() {
        let command = tp_command();

        let cmd = "tp 0".to_string();
        let command_sequence = CommandsHandler::parse_command(&cmd);
        let result = command.get_current(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "tp".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_some(), true);
        assert_eq!(result.as_ref().unwrap().1.as_ref().unwrap().name, "x".to_string());
    }
}
