use chrono::Local;
use common::commands::command::{Arg, Command, CommandMatch};
use common::commands::complitions::{CompleteRequest, CompleteResponse, apply_complete};
use common::utils::colors::parse_to_console_godot;
use flume::unbounded;
use flume::{Receiver, Sender};
use godot::classes::{InputEvent, InputEventKey};
use godot::global::Key;
use godot::{
    classes::{IMarginContainer, Input, LineEdit, MarginContainer, RichTextLabel, TextureButton, input::MouseMode},
    prelude::*,
};
use lazy_static::lazy_static;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const CONSOLE_SCENE_PATH: &str = "res://scenes/console.tscn";

fn get_commands() -> Vec<Command> {
    let mut commands: Vec<Command> = Default::default();

    let c = Command::new("exit".to_string());
    commands.push(c);

    let c = Command::new("disconnect".to_string());
    commands.push(c);

    let setting_choices = vec!["ssao", "fps"];
    let c = Command::new("setting".to_string())
        .arg(Arg::new("name".to_owned()).required(true).choices(setting_choices))
        .arg(Arg::new("value".to_owned()).required(true));
    commands.push(c);

    commands
}

#[derive(GodotClass)]
#[class(init, base=MarginContainer)]
pub struct Console {
    base: Base<MarginContainer>,

    #[export]
    console_text: Option<Gd<RichTextLabel>>,

    #[export]
    console_input: Option<Gd<LineEdit>>,

    #[export]
    console_button: Option<Gd<TextureButton>>,

    #[export]
    console_sugestions: Option<Gd<RichTextLabel>>,

    commands_history: Vec<String>,

    commands: Vec<Command>,

    complitions: Option<CompleteResponse>,
    selected_complition: Option<u16>,
}

lazy_static! {
    static ref CONSOLE_OUTPUT_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
    static ref CONSOLE_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

/// Used to transmit motion data
#[derive(GodotClass)]
#[class(no_init)]
pub struct GDCommandMatch {
    pub command_match: CommandMatch,
}

#[godot_api]
impl Console {
    pub fn create() -> Gd<Self> {
        load::<PackedScene>(CONSOLE_SCENE_PATH).instantiate_as::<Self>()
    }

    #[signal]
    pub fn client_command_sended(command: Gd<GDCommandMatch>);

    #[signal]
    pub fn network_command_sended(command: GString);

    fn handle_command(&mut self, command: &String) {
        let command_sequence = Command::parse_command(command);
        if command_sequence.len() == 0 {
            return;
        }
        let lead_command = command_sequence[0].clone();

        let mut gd_m: Option<Gd<GDCommandMatch>> = None;
        for command in self.commands.iter() {
            if *command.get_name() != lead_command {
                continue;
            }

            match command.eval(&command_sequence[1..]) {
                Ok(command_match) => {
                    let m = Gd::<GDCommandMatch>::from_init_fn(|_base| GDCommandMatch { command_match });
                    gd_m = Some(m);
                }
                Err(e) => {
                    log::error!(target: "console", "&cCommand &4\"{}\" &cerror: {}", command.get_name(), e);
                    return;
                }
            };
        }

        match gd_m {
            Some(m) => self.signals().client_command_sended().emit(&m),

            // In case clients commands is not found
            None => self.signals().network_command_sended().emit(command),
        }
    }

    pub fn send_message(message: String) {
        let date = Local::now();
        let m = format!("&f{}: {}", date.format("%H:%M:%S.%3f"), message);

        let godot_msg = parse_to_console_godot(&m);
        godot_print_rich!("{}", godot_msg);
        CONSOLE_OUTPUT_CHANNEL.0.send(godot_msg).unwrap();
    }

    fn append_text(&mut self, message: String) {
        self.console_text
            .as_mut()
            .unwrap()
            .append_text(&format!("\n{}", message));
        self.scroll_to_bottom();
    }

    pub fn is_active() -> bool {
        CONSOLE_ACTIVE.load(Ordering::Relaxed)
    }

    pub fn toggle(&mut self, state: bool) {
        CONSOLE_ACTIVE.store(state, Ordering::Relaxed);
        self.base_mut().set_visible(state);

        self.clear_completions();
        if state {
            let console_input = self.console_input.as_mut().unwrap();
            if !console_input.has_focus() {
                console_input.grab_focus();
            }
            console_input.clear();

            Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        } else {
            Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        }
    }

    fn scroll_to_bottom(&mut self) {
        let console_text = self.console_text.as_mut().unwrap();
        let lines = console_text.get_line_count();
        console_text.scroll_to_line(lines - 1);
    }

    #[func]
    fn on_button_pressed(&mut self) {
        self.submit_text();
    }

    fn submit_text(&mut self) {
        self.clear_completions();

        let command = self.console_input.as_ref().unwrap().get_text().to_string();
        if command.len() == 0 {
            return;
        }

        self.scroll_to_bottom();

        if self.commands_history.contains(&command) {
            let index = self.commands_history.iter().position(|x| *x == command).unwrap();
            self.commands_history.remove(index);
        }
        self.commands_history.push(command.clone());

        self.handle_command(&command);

        self.console_input.as_mut().unwrap().clear();
        self.console_input.as_mut().unwrap().call_deferred("grab_focus", &[]);
    }

    fn generate_completions(&mut self) {
        self.clear_completions();

        let line = self.console_input.as_ref().unwrap().get_text().to_string();
        let pos = self.console_input.as_ref().unwrap().get_caret_column() as usize;

        let request = CompleteRequest::create(line.clone(), pos);
        let complete_response = CompleteResponse::complete(&request, self.commands.iter());
        if complete_response.get_completions().len() > 0 {
            self.selected_complition = Some(0);
            self.complitions = Some(complete_response);
            self.update_completions_display();
        }
    }

    // Update dsiplay of comptition options
    fn update_completions_display(&mut self) {
        let complitions = self.complitions.as_ref().unwrap();
        let mut res: Vec<String> = Default::default();

        let selected = self.selected_complition.get_or_insert_with(|| 0);

        let mut i = 0;
        for complition in complitions.get_completions() {
            let r = if *selected == i {
                format!(
                    "[bgcolor=#393838][b]{}[/b][/bgcolor]",
                    parse_to_console_godot(&complition.get_display())
                )
            } else {
                format!(
                    "[bgcolor=#393838]{}[/bgcolor]",
                    parse_to_console_godot(&complition.get_display())
                )
            };
            res.push(r);
            i += 1;
        }
        self.console_sugestions.as_mut().unwrap().set_text(&res.join("\n"));
    }

    fn select_next(&mut self) {
        let Some(complitions) = self.complitions.as_ref() else {
            return;
        };
        let selected = self.selected_complition.get_or_insert_with(|| 0);
        if *selected >= (complitions.get_completions().len() - 1) as u16 {
            *selected = 0_u16;
        } else {
            *selected += 1;
        }
        self.update_completions_display();
    }

    fn select_prev(&mut self) {
        let Some(complitions) = self.complitions.as_ref() else {
            return;
        };
        let selected = self.selected_complition.get_or_insert_with(|| 0);
        if *selected == 0_u16 {
            *selected = (complitions.get_completions().len() - 1) as u16;
        } else {
            *selected -= 1;
        }
        self.update_completions_display();
    }

    // Display complitions or apply current completion
    fn completion_request(&mut self) {
        if self.selected_complition.is_some() {
            // If complition is displayed
            self.apply_complete();
        } else {
            // Generate completions
            self.generate_completions();
        }
    }

    // Update input by completion sugestion
    fn apply_complete(&mut self) {
        let selected = self.selected_complition.take().unwrap();

        let complitions = self.complitions.as_ref().unwrap();
        let complition = complitions.get_completions().get(selected as usize).unwrap();

        let (new_input, caret_column) = apply_complete(&complitions, &complition);

        // Set text
        self.console_input.as_mut().unwrap().set_text(&new_input);

        // Set cursor position
        self.console_input.as_mut().unwrap().set_caret_column(caret_column);

        self.clear_completions();
    }

    fn clear_completions(&mut self) {
        self.complitions = None;
        self.selected_complition = None;
        self.console_sugestions.as_mut().unwrap().set_text("");
    }

    #[func]
    fn on_window_focus_entered(&mut self) {
        if !Console::is_active() {
            return;
        }
        self.console_input.as_mut().unwrap().call_deferred("grab_focus", &[]);
    }

    #[func]
    fn on_input_changed(&mut self, text: GString) {
        if text.len() == 0 {
            self.clear_completions();
            return;
        }
        self.generate_completions();
    }
}

#[godot_api]
impl IMarginContainer for Console {
    fn ready(&mut self) {
        let gd = self.to_gd().clone();
        if let Some(console_button) = self.console_button.as_mut() {
            console_button
                .signals()
                .pressed()
                .connect_other(&gd, Self::on_button_pressed);
        }

        let console_input = self.console_input.as_mut().unwrap();
        console_input.set_text("");
        console_input
            .signals()
            .text_changed()
            .connect_other(&gd, Self::on_input_changed);

        self.console_text.as_mut().unwrap().set_text("");
        self.console_sugestions.as_mut().unwrap().set_text("");

        self.base()
            .get_window()
            .unwrap()
            .signals()
            .focus_entered()
            .connect_other(&self.to_gd(), Self::on_window_focus_entered);

        self.commands = get_commands();
        self.base_mut().set_visible(false);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if !Console::is_active() {
            return;
        }

        if let Ok(event) = event.clone().try_cast::<InputEventKey>() {
            if event.is_pressed() && (event.get_keycode() == Key::ENTER || event.get_keycode() == Key::KP_ENTER) {
                self.submit_text();
                return;
            }
        }

        let input = Input::singleton();
        if input.is_action_just_pressed("ui_up") {
            self.select_prev();
            self.console_input.as_mut().unwrap().call_deferred("grab_focus", &[]);
        }
        if input.is_action_just_pressed("ui_down") {
            self.select_next();
            self.console_input.as_mut().unwrap().call_deferred("grab_focus", &[]);
        }
        if input.is_action_just_pressed("ui_focus_next") {
            self.completion_request();
            self.console_input.as_mut().unwrap().call_deferred("grab_focus", &[]);
        }
    }

    fn process(&mut self, _delta: f64) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.drain() {
            self.append_text(message);
        }
    }
}
