use ahash::HashMap;
use regex::Regex;
use std::slice::Iter;
use std::str::FromStr;

pub const REGEX_COMMAND: &str = r####"([\d\w$&+,:;=?@#|'<>.^*()%!-]*)|"([\d\w$&+,:;=?@#|'<>.^*()%!\- ]*)""####;

#[derive(Clone, Debug)]
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

    pub fn parse_command(command: &String) -> Vec<String> {
        let re = Regex::new(REGEX_COMMAND).unwrap();
        re.find_iter(&command).map(|e| e.as_str().to_string()).collect()
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn subcommand_required(mut self, required: bool) -> Self {
        self.subcommand_required = required;
        self
    }

    pub fn commands(&self) -> Iter<'_, Command> {
        self.commands.iter()
    }

    /// command_sequence example:
    /// "world"  "create"    "test"      "123"
    /// ^command ^subcommand ^arg        ^arg optional
    ///  name     name        world name  seed
    pub fn eval(&self, command_sequence: &[String]) -> Result<CommandMatch, String> {
        let mut args: HashMap<String, String> = Default::default();

        let mut subcommand: Option<Box<CommandMatch>> = None;
        if command_sequence.len() > 0 {
            for command in self.commands.iter() {
                // if command matches
                // println!("command.name:{} command_sequence[0]:{}", command.name, command_sequence[0]);
                if command.name == command_sequence[0] {
                    let command_match = command.eval(&command_sequence[1..])?;
                    subcommand = Some(Box::new(command_match));
                    break;
                }
            }
        }

        // println!("subcommand:{:?} self.subcommand_required:{}", subcommand, self.subcommand_required);
        if subcommand.is_none() && self.subcommand_required {
            return Err(format!("&csubcommand for &4\"{}\" &cis required", self.name));
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
                        return Err(format!("&cargument &4\"{}\" &cis required", arg.name));
                    }
                }
            }
            i += 1;
        }

        return Ok(CommandMatch::new(self.name.clone(), subcommand, args));
    }

    // Get current cubcommand
    pub fn get_current_subcommand(&self, command_sequence: &[String]) -> Option<(&Command, Option<&Arg>)> {
        // println!("GET_CURRENT name:{} command_sequence:{:?}", self.name, command_sequence);

        if command_sequence.len() > 0 {
            for c in self.commands.iter() {
                if c.name != command_sequence[0] {
                    continue;
                }
                return c.get_current_subcommand(&command_sequence[1..]);
            }
        }

        // println!("self.args.len():{} command_sequence.len():{} self.subcommand_required:{} self.name:{}", self.args.len(), command_sequence.len(), self.subcommand_required, self.name);
        if self.subcommand_required || command_sequence.len() == 0 {
            return Some((self, None));
        }

        // If command arg count is less than provided args
        if self.args.len() < command_sequence.len() {
            return None;
        }

        // println!("command_sequence:{:?}", command_sequence);
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

#[derive(Clone, Debug)]
pub enum ArgType {
    Choices(Vec<String>),
}

#[derive(Clone, Debug, Default)]
pub struct Arg {
    name: String,
    required: bool,
    arg_type: Option<ArgType>,
}

impl Arg {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            ..Default::default()
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_arg_type(&self) -> Option<&ArgType> {
        self.arg_type.as_ref()
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn choices<S: Into<String>>(mut self, mut choices: Vec<S>) -> Self {
        let c: Vec<String> = choices.drain(..).map(|m| m.into()).collect();
        self.arg_type = Some(ArgType::Choices(c));
        self
    }
}

#[derive(Clone, Debug)]
pub struct CommandMatch {
    name: String,
    subcommand: Option<Box<CommandMatch>>,
    args: HashMap<String, String>,
}

impl CommandMatch {
    pub fn new(name: String, subcommand: Option<Box<CommandMatch>>, args: HashMap<String, String>) -> Self {
        Self { name, subcommand, args }
    }

    pub fn get_arg<T: FromStr, S: Into<String>>(&self, arg_name: S) -> Result<T, String> {
        let key: String = arg_name.into();
        match self.args.get(&key) {
            Some(a) => match a.parse::<T>() {
                Ok(v) => Ok(v),
                Err(_e) => Err(format!("&cparameter &4{} &cconverting error", self.name)),
            },
            None => Err("&cparameter has not been provided".to_string()),
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
    use super::{Arg, Command};

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
        let command_sequence = Command::parse_command(&cmd);
        let result = command.eval(&command_sequence[1..]);

        assert_eq!(result.is_ok(), true, "error: {}", result.err().unwrap());
        assert_eq!(result.as_ref().unwrap().get_name(), &"world".to_string());

        let subcommand = result.as_ref().unwrap().subcommand();
        assert_eq!(subcommand.is_some(), true, "World create subcommand must be presented");
        let s = subcommand.as_ref().unwrap();
        assert_eq!(s.get_name(), "create");

        let slug = s.get_arg::<String, _>(&"slug".to_owned());
        assert_eq!(slug.is_ok(), true, "error: {}", slug.err().unwrap());
        assert_eq!(slug.as_ref().unwrap(), &"test".to_string());
    }

    #[test]
    fn test_command_eval_error() {
        let command = world_command();

        let cmd = "world test".to_string();
        let command_sequence = Command::parse_command(&cmd);
        let result = command.eval(&command_sequence[1..]);

        assert_eq!(result.is_err(), true);
        assert_eq!(
            result.err().unwrap().to_string(),
            "&csubcommand for &4\"world\" &cis required".to_string()
        );
    }

    #[test]
    fn test_command_eval_world_empty() {
        let command = world_command();

        let cmd = "world".to_string();
        let command_sequence = Command::parse_command(&cmd);
        let result = command.eval(&command_sequence[1..]);

        assert_eq!(result.is_err(), true);
        assert_eq!(
            result.err().unwrap().to_string(),
            "&csubcommand for &4\"world\" &cis required".to_string()
        );
    }

    #[test]
    fn test_command_current_world() {
        let command = world_command();

        let cmd = "world ".to_string();
        let command_sequence = Command::parse_command(&cmd);
        let result = command.get_current_subcommand(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "world".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_none(), true);
    }

    #[test]
    fn test_command_current_world_part() {
        let command = world_command();

        let cmd = "world c".to_string();
        let command_sequence = Command::parse_command(&cmd);
        let result = command.get_current_subcommand(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "world".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_none(), true);
    }

    #[test]
    fn test_command_current_world_slug() {
        let command = world_command();

        let cmd = "world create te".to_string();
        let command_sequence = Command::parse_command(&cmd);
        let result = command.get_current_subcommand(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "create".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_some(), true);
        assert_eq!(result.as_ref().unwrap().1.as_ref().unwrap().name, "slug".to_string());
    }

    #[test]
    fn test_command_current_world_slug_empty() {
        let command = world_command();

        let cmd = "world create ".to_string();
        let command_sequence = Command::parse_command(&cmd);
        let result = command.get_current_subcommand(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "create".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_some(), true);
        assert_eq!(result.as_ref().unwrap().1.as_ref().unwrap().name, "slug".to_string());
    }

    #[test]
    fn test_command_current_world_list() {
        let command = world_command();

        let cmd = "world list ".to_string();
        let command_sequence = Command::parse_command(&cmd);
        let result = command.get_current_subcommand(&command_sequence[1..]);

        assert_eq!(result.is_none(), true);
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
        let command_sequence = Command::parse_command(&cmd);
        let result = command.get_current_subcommand(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "tp".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_some(), true);
        assert_eq!(result.as_ref().unwrap().1.as_ref().unwrap().name, "x".to_string());
    }

    #[test]
    fn test_parse_command() {
        let command_sequence = Command::parse_command(&"tp ".to_string());
        assert_eq!(command_sequence, vec!["tp", ""]);

        let command_sequence = Command::parse_command(&"world create te".to_string());
        assert_eq!(command_sequence, vec!["world", "create", "te"]);
    }

    #[test]
    fn test_command_current_tp_root() {
        let command = tp_command();

        let cmd = "tp".to_string();
        let command_sequence = Command::parse_command(&cmd);
        let result = command.get_current_subcommand(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "tp".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_none(), true);
    }

    #[test]
    fn test_command_current_tp_last() {
        let command = tp_command();

        let cmd = "tp 0".to_string();
        let command_sequence = Command::parse_command(&cmd);
        let result = command.get_current_subcommand(&command_sequence[1..]);

        assert_eq!(result.is_some(), true, "Command must be found");
        assert_eq!(result.as_ref().unwrap().0.name, "tp".to_string());
        assert_eq!(result.as_ref().unwrap().1.is_some(), true);
        assert_eq!(result.as_ref().unwrap().1.as_ref().unwrap().name, "x".to_string());
    }
}
