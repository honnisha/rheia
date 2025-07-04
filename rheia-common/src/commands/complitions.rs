use crate::utils::string_remove_range;

use super::command::{ArgType, Command};

/// Requesting options for completing the console command
#[derive(Clone, PartialEq)]
pub struct CompleteRequest {
    line: String,
    pos: usize,
}

impl CompleteRequest {
    pub fn create<S: Into<String>>(line: S, pos: usize) -> Self {
        Self { line: line.into(), pos }
    }
    pub fn get_line(&self) -> &String {
        &self.line
    }

    pub fn get_pos(&self) -> &usize {
        &self.pos
    }
}

#[derive(Clone, PartialEq)]
pub struct Completion {
    display: String,
    completion: String,
}

impl Completion {
    fn generate_completion(input: &String, target: &String) -> Option<Self> {
        let Some(i) = target.find(input) else {
            return None;
        };
        let mut display = target.clone();
        display.insert_str(i, "&a");
        display.insert_str(i + input.len() + 2, "&r&f");

        Some(Self {
            display,
            completion: target.clone(),
        })
    }

    pub fn get_display(&self) -> &String {
        &self.display
    }

    // Full replace string
    pub fn get_completion(&self) -> &String {
        &self.completion
    }
}

/// Responding to a request to retrieve console command options
pub struct CompleteResponse {
    offset: usize,
    request: CompleteRequest,
    completions: Vec<Completion>,
}

impl CompleteResponse {
    pub fn create(request: CompleteRequest) -> Self {
        Self {
            offset: 0,
            request: request,
            completions: Default::default(),
        }
    }

    pub fn get_completions(&self) -> &Vec<Completion> {
        &self.completions
    }

    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }

    /// Size of thee current search string len
    ///
    /// For example "world cre|"
    /// would be 3 "cre(ate)"
    pub fn get_offset(&self) -> &usize {
        &self.offset
    }

    pub fn get_request(&self) -> &CompleteRequest {
        &self.request
    }

    pub fn add_completion(&mut self, completion: Completion) {
        self.completions.push(completion);
    }

    pub fn complete<'a>(request: &CompleteRequest, commands: impl Iterator<Item = &'a Command>) -> CompleteResponse {
        let line = request.get_line().clone();
        let pos = request.get_pos().clone();

        let mut complete_response: CompleteResponse = Self::create(request.clone());

        let command_sequence = Command::parse_command(&line[..pos].to_string());
        // Return all command names
        if command_sequence.len() == 0 {
            return complete_response;
        }
        let lead_command = command_sequence[0].clone();

        // If typing main command name
        if pos <= lead_command.len() {
            let input = line[..pos].to_string();
            complete_response.set_offset(input.len());
            for command in commands {
                if let Some(completion) = Completion::generate_completion(&input, command.get_name()) {
                    complete_response.add_completion(completion);
                }
            }
            return complete_response;
        }

        for command in commands {
            // Find subcommand
            if *command.get_name() != lead_command {
                continue;
            }

            let last_arg = command_sequence[command_sequence.len() - 1].clone();
            complete_response.set_offset(last_arg.len());

            if let Some((command, arg)) = command.get_current_subcommand(&command_sequence[1..]) {
                match arg {
                    Some(arg) => {
                        if let Some(arg_type) = arg.get_arg_type() {
                            match arg_type {
                                ArgType::Choices(choices) => {
                                    for choice in choices {
                                        if let Some(completion) = Completion::generate_completion(&last_arg, &choice) {
                                            complete_response.add_completion(completion);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        for command in command.commands() {
                            if let Some(completion) = Completion::generate_completion(&last_arg, command.get_name()) {
                                complete_response.add_completion(completion);
                            }
                        }
                    }
                }
            }
            break;
        }
        return complete_response;
    }
}

pub fn apply_complete(complitions: &CompleteResponse, complition: &Completion) -> (String, i32) {
    let pos = complitions.get_request().get_pos().clone();

    // Remove entered command
    let mut input = string_remove_range(
        complitions.get_request().get_line(),
        pos - complitions.get_offset(),
        pos,
    );

    // Replace with full one
    input.insert_str(pos - complitions.get_offset(), complition.get_completion());

    let caret_column = (pos - complitions.get_offset() + complition.get_completion().len()) as i32;
    (input, caret_column)
}

#[cfg(test)]
mod tests {
    use super::{CompleteRequest, CompleteResponse, Completion, apply_complete};
    use crate::{
        commands::command::{Arg, Command},
        utils::string_remove_range,
    };

    fn get_commands() -> Vec<Command> {
        let mut commands: Vec<Command> = Default::default();

        let c = Command::new("exit".to_string());
        commands.push(c);

        let setting_choices = vec!["ssao"];
        let c = Command::new("setting".to_string())
            .arg(Arg::new("name".to_owned()).required(true).choices(setting_choices))
            .arg(Arg::new("value".to_owned()).required(true));
        commands.push(c);

        commands
    }

    #[test]
    fn test_complete() {
        let request = CompleteRequest::create("ex", 2);
        let commands = get_commands();

        let complitions = CompleteResponse::complete(&request, commands.iter());
        assert_eq!(*complitions.get_offset(), 2);

        assert_eq!(complitions.get_completions().len(), 1);
        let complition = complitions.get_completions().iter().next().unwrap();
        assert_eq!(complition.get_completion(), "exit");

        let (new_input, caret_column) = apply_complete(&complitions, &complition);

        assert_eq!(new_input, "exit");
        assert_eq!(caret_column, 4);
    }

    #[test]
    fn test_string_remove_range() {
        let input = string_remove_range("world", 0, 2);
        assert_eq!(input, "rld");
    }

    #[test]
    fn test_apply() {
        // "wo| hello"
        let request = CompleteRequest::create("wo hello", 2);
        let mut complitions = CompleteResponse::create(request);
        complitions.set_offset(2);
        let complition = Completion {
            display: "world".into(),
            completion: "world".into(),
        };

        let (new_input, caret_column) = apply_complete(&complitions, &complition);

        // Must become "world| hello"
        assert_eq!(new_input, "world hello");
        assert_eq!(caret_column, 5);
    }

    #[test]
    fn test_apply_2() {
        // "wo|"
        let request = CompleteRequest::create("wo", 2);
        let mut complitions = CompleteResponse::create(request);
        complitions.set_offset(2);
        let complition = Completion {
            display: "world".into(),
            completion: "world".into(),
        };

        let (new_input, caret_column) = apply_complete(&complitions, &complition);

        // Must become "world|"
        assert_eq!(new_input, "world");
        assert_eq!(caret_column, 5);
    }
}
