use super::command::Command;

/// Requesting options for completing the console command
#[derive(Clone, PartialEq)]
pub struct CompleteRequest {
    line: String,
    pos: usize,
}

impl CompleteRequest {
    pub fn create(line: String, pos: usize) -> Self {
        Self { line, pos }
    }
    pub fn get_line(&self) -> &String {
        &self.line
    }

    pub fn get_pos(&self) -> &usize {
        &self.pos
    }
}

/// Responding to a request to retrieve console command options
pub struct CompleteResponse {
    request: CompleteRequest,
    completions: Vec<String>,
}

impl CompleteResponse {
    pub fn create(request: CompleteRequest) -> Self {
        Self { request: request, completions: Default::default() }
    }

    pub fn get_completions(&self) -> &Vec<String> {
        &self.completions
    }

    pub fn get_request(&self) -> &CompleteRequest {
        &self.request
    }

    pub fn add_completion(&mut self, completion: String) {
        self.completions.push(completion);
    }

    pub fn complete<'a>(request: &CompleteRequest, commands: impl Iterator<Item = &'a Command>) -> Option<CompleteResponse> {
        let line = request.get_line().clone();
        let pos = request.get_pos().clone();

        let mut complete_response: CompleteResponse = Self::create(request.clone());

        let command_sequence = Command::parse_command(&line[..pos].to_string());
        // Return all command names
        if command_sequence.len() == 0 {
            return None;
        }
        let lead_command = command_sequence[0].clone();

        // Complete command name
        if pos <= lead_command.len() {
            for command in commands {
                if command.get_name().starts_with(&line[..pos]) {
                    complete_response.add_completion(command.get_name()[pos..].to_string());
                }
            }
        } else {
            for command in commands {
                if *command.get_name() != lead_command {
                    continue;
                }

                let last_arg = command_sequence[command_sequence.len() - 1].clone();

                if let Some((command, arg)) = command.get_current(&command_sequence[1..]) {
                    match arg {
                        Some(_a) => {}
                        None => {
                            for c in command.commands() {
                                // if command name starts with arg name
                                if c.get_name().starts_with(&last_arg) {
                                    let complete = c.get_name()[last_arg.len()..].to_string();
                                    complete_response.add_completion(complete);
                                }
                            }
                        }
                    }
                }
                break;
            }
        }

        return if complete_response.get_completions().len() > 0 {
            Some(complete_response)
        } else {
            None
        };
    }
}
