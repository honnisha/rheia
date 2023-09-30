use std::fmt;

#[derive(Clone)]
pub struct Command {
    name: String,
    subcommand_required: bool,
    args: Vec<Arg>,
    commands: Vec<Command>,
}

#[derive(Debug)]
pub struct CommandEvalError(String);

impl fmt::Display for CommandEvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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

    pub fn eval(&self, command_sequence: &Vec<String>) -> Result<ArgMatches, CommandEvalError> {
        Ok(ArgMatches::new())
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
pub struct ArgMatches {}

#[derive(Debug)]
pub struct ArgError(String);

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ArgMatches {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_arg<T>(&self, arg_name: String) -> Result<T, ArgError> {
        todo!();
    }

    pub fn subcommand(&self) -> Option<(&str, &ArgMatches)> {
        todo!();
        return None;
    }
}
