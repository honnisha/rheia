use godot::{
    classes::{Button, IMarginContainer, MarginContainer, RichTextLabel},
    prelude::*,
};

const TEXT_PATH: &str = "VBoxContainer/Text";
const CLOSE_BUTTON_PATH: &str = "VBoxContainer/MarginContainer/Back";

#[derive(GodotClass)]
#[class(init, base=MarginContainer)]
pub struct TextScreen {
    base: Base<MarginContainer>,

    #[export]
    text: Option<Gd<RichTextLabel>>,

    #[export]
    close_button: Option<Gd<Button>>,
}

impl TextScreen {
    pub fn update_text(&mut self, text: String) {
        let msg = format!("[center]{}[/center]", text);
        self.text.as_mut().unwrap().set_text(&msg);
    }

    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state);
    }

    pub fn toggle_close_button(&mut self, text: Option<String>) {
        let state = match text {
            Some(t) => {
                self.close_button.as_mut().unwrap().set_text(&t);
                true
            }
            None => false,
        };
        self.close_button.as_mut().unwrap().set_visible(state);
    }
}

#[godot_api]
impl TextScreen {
    #[func]
    fn close_button_pressed(&mut self) {
        self.base_mut().emit_signal("close_button_pressed", &[]);
    }

    #[signal]
    fn close_button_pressed();
}

#[godot_api]
impl IMarginContainer for TextScreen {
    fn ready(&mut self) {
        let mut close_button = self.close_button.as_mut().unwrap().clone();
        close_button.connect(
            "pressed",
            &Callable::from_object_method(&self.base().to_godot(), "close_button_pressed"),
        );
        self.toggle_close_button(None);
    }
}
