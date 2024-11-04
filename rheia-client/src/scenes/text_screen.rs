use godot::{
    engine::{Button, IMarginContainer, MarginContainer, RichTextLabel},
    prelude::*,
};

const TEXT_PATH: &str = "VBoxContainer/Text";
const CLOSE_BUTTON_PATH: &str = "VBoxContainer/MarginContainer/Back";

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct TextScreen {
    base: Base<MarginContainer>,

    text: OnReady<Gd<RichTextLabel>>,
    close_button: OnReady<Gd<Button>>,
}

impl TextScreen {
    pub fn set_text(&mut self, text: String) {
        let msg = format!("[center]{}[/center]", text);
        self.text.set_text(msg.into());
    }

    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state);
    }

    pub fn toggle_close_button(&mut self, text: Option<String>) {
        let state = match text {
            Some(t) => {
                self.close_button.set_text(t.into());
                true
            },
            None => false,
        };
        self.close_button.set_visible(state);
    }
}

#[godot_api]
impl TextScreen {
    #[func]
    fn close_button_pressed(&mut self) {
        self.base_mut()
            .emit_signal("close_button_pressed".into(), &[]);
    }

    #[signal]
    fn close_button_pressed();
}

#[godot_api]
impl IMarginContainer for TextScreen {
    fn init(base: Base<MarginContainer>) -> Self {
        Self {
            base,
            text: OnReady::manual(),
            close_button: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        self.text.init(self.base().get_node_as::<RichTextLabel>(TEXT_PATH));

        let mut close_button = self.base().get_node_as::<Button>(CLOSE_BUTTON_PATH);
        close_button.connect(
            "pressed".into(),
            Callable::from_object_method(&self.base().to_godot(), "close_button_pressed"),
        );
        self.close_button.init(close_button);
        self.toggle_close_button(None);
    }
}
