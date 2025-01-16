use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use chrono::Local;
use common::utils::colors::parse_to_console_godot;
use flume::{bounded, unbounded, Drain};
use flume::{Receiver, Sender};
use godot::{
    classes::{input::MouseMode, IMarginContainer, LineEdit, MarginContainer, RichTextLabel, TextureButton},
    prelude::*,
};
use lazy_static::lazy_static;

const TEXT_PATH: &str = "MarginContainer/VBoxContainer/HBoxContainer/ConsoleBackground/MarginContainer/ConsoleText";
const INPUT_PATH: &str = "MarginContainer/VBoxContainer/HBoxContainer2/TextureRect/ConsoleInput";
const BUTTON_PATH: &str = "MarginContainer/VBoxContainer/HBoxContainer2/ConsoleButton";
const SUGESTIOINS_PATH: &str = "MarginContainer/VBoxContainer/HBoxContainer3/MarginContainer/ConsoleSugestioins";

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct Console {
    base: Base<MarginContainer>,
    console_text: Option<Gd<RichTextLabel>>,
    console_input: Option<Gd<LineEdit>>,
    console_button: Option<Gd<TextureButton>>,
    console_sugestions: Option<Gd<RichTextLabel>>,
    commands_history: Vec<String>,
}

lazy_static! {
    static ref CONSOLE_OUTPUT_CHANNEL: (Sender<String>, Receiver<String>) = unbounded();
    static ref CONSOLE_INPUT_CHANNEL: (Sender<String>, Receiver<String>) = bounded(1);
    static ref CONSOLE_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

#[godot_api]
impl Console {
    pub fn iter_console_input() -> Drain<'static, String> {
        CONSOLE_INPUT_CHANNEL.1.drain()
    }

    pub fn send_message(message: String) {
        let date = Local::now();
        let m = format!("{}: {}", date.format("%H:%M:%S.%3f"), message);

        godot_print!("{}", m);
        CONSOLE_OUTPUT_CHANNEL.0.send(parse_to_console_godot(&m)).unwrap();
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
        let active = Console::is_active();
        self.base_mut().set_visible(active);

        if active {
            let i = self.console_input.as_mut().unwrap();
            if !i.has_focus() {
                i.grab_focus();
            }
            i.clear();

            Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        } else {
            Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        }
    }

    fn scroll_to_bottom(&mut self) {
        let c = self.console_text.as_mut().unwrap();
        let lines = c.get_line_count();
        c.scroll_to_line(lines - 1);
    }

    fn submit_command(&mut self, command: String) {
        if self.commands_history.contains(&command) {
            let index = self.commands_history.iter().position(|x| *x == command).unwrap();
            self.commands_history.remove(index);
        }
        self.commands_history.push(command.clone());
        CONSOLE_INPUT_CHANNEL.0.send(command).unwrap();
    }

    #[func]
    fn button_pressed(&mut self) {
        self.scroll_to_bottom();
        let i = self.console_input.as_mut().unwrap();
        let command = i.get_text().to_string();
        i.clear();
        self.submit_command(command);
    }

    //#[func]
    //fn text_changed(&mut self, new_text: GString) {
    //    godot_print!("text changed: {}", new_text);
    //}

    #[func]
    fn text_submitted(&mut self, new_text: GString) {
        self.scroll_to_bottom();
        self.submit_command(new_text.to_string());
        self.console_input.as_mut().unwrap().clear();
    }
}

#[godot_api]
impl IMarginContainer for Console {
    fn init(base: Base<MarginContainer>) -> Self {
        Console {
            base: base,
            console_text: None,
            console_input: None,
            console_button: None,
            console_sugestions: None,
            commands_history: Default::default(),
        }
    }

    fn ready(&mut self) {
        log::info!(target: "console", "Start loading console;");
        match self.base().try_get_node_as::<RichTextLabel>(TEXT_PATH) {
            Some(e) => self.console_text = Some(e),
            _ => panic!("console_text element not found"),
        };
        match self.base().try_get_node_as::<LineEdit>(INPUT_PATH) {
            Some(mut e) => {
                e.connect(
                    "text_submitted",
                    &Callable::from_object_method(&self.base().to_godot(), "text_submitted"),
                );
                self.console_input = Some(e);
            }
            _ => panic!("console_input element not found"),
        };
        match self.base().try_get_node_as::<TextureButton>(BUTTON_PATH) {
            Some(mut e) => {
                e.connect(
                    "pressed",
                    &Callable::from_object_method(&self.base().to_godot(), "button_pressed"),
                );
                self.console_button = Some(e);
            }
            _ => panic!("console_button element not found"),
        };
        match self.base().try_get_node_as::<RichTextLabel>(SUGESTIOINS_PATH) {
            Some(e) => self.console_sugestions = Some(e),
            _ => panic!("console_sugestions element not found"),
        };
        self.base_mut().set_visible(false);
        log::info!(target: "console", "Console successfully loaded;");
    }

    fn process(&mut self, _delta: f64) {
        #[cfg(feature = "trace")]
        let _span = tracing::span!(tracing::Level::INFO, "console_handler").entered();

        for message in CONSOLE_OUTPUT_CHANNEL.1.drain() {
            self.append_text(message);
        }

        if !Console::is_active() {
            return;
        }

        let input = Input::singleton();
        if input.is_action_just_pressed("ui_up") {
            godot_print!("up");
        } else if input.is_action_just_pressed("ui_down") {
            godot_print!("down");
        } else if input.is_action_just_pressed("ui_focus_next") {
            godot_print!("tab");
        }
    }
}
