use chrono::Local;
use common::utils::colors::parse_to_console_godot;
use flume::{Drain, bounded, unbounded};
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

#[derive(GodotClass)]
#[class(base=MarginContainer)]
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
    fn button_pressed(&mut self) {
        self.submit_text();
    }

    fn submit_text(&mut self) {
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
        CONSOLE_INPUT_CHANNEL.0.send(command).unwrap();

        self.console_input.as_mut().unwrap().clear();
        // self.console_input.as_mut().unwrap().grab_focus();
        self.console_input.as_mut().unwrap().call_deferred("grab_focus", &[]);
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
        let gd = self.to_gd().clone();
        if let Some(console_button) = self.console_button.as_mut() {
            console_button.signals().pressed().connect_other(&gd, Self::button_pressed);
        }
        self.base_mut().set_visible(false);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if !Console::is_active() {
            return;
        }

        if let Ok(event) = event.clone().try_cast::<InputEventKey>() {
            if event.is_pressed() && event.get_keycode() == Key::ENTER {
                self.submit_text();
                return;
            }
        }

        let input = Input::singleton();
        if input.is_action_just_pressed("ui_up") {
            godot_print!("up");
        }
        if input.is_action_just_pressed("ui_down") {
            godot_print!("down");
        }
        if input.is_action_just_pressed("ui_focus_next") {
            godot_print!("tab");
        }
    }

    fn process(&mut self, _delta: f64) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.drain() {
            self.append_text(message);
        }
    }
}
