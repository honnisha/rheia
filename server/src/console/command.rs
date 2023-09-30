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
    ///  name     name        world name seed
    pub fn eval(&self, command_sequence: &[String]) -> Result<CommandMatch, CommandError> {
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

        let mut args: HashMap<String, String> = Default::default();
        let mut i = 0;
        for arg in self.args.iter() {
            let mut arg_value: Option<String> = None;
            if command_sequence.len() > i {
                arg_value = Some(command_sequence[i].clone());
            }
            match arg_value {
                Some(v) => {
                    println!("arg.name:{} v:{} i:{} command_sequence:{:?}", arg.name, v, i, command_sequence);
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

    #[test]
    fn test_command_eval() {
        let command = Command::new("world".to_string())
            .subcommand_required(true)
            .subcommand(Command::new("list".to_owned()))
            .subcommand(
                Command::new("create".to_owned())
                    .arg(Arg::new("slug".to_owned()).required(true))
                    .arg(Arg::new("seed".to_owned())),
            );

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
}
