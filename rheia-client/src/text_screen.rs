use godot::{
    engine::{IMarginContainer, MarginContainer, RichTextLabel},
    prelude::*,
};

const TEXT_PATH: &str = "Text";

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct TextScreen {
    base: Base<MarginContainer>,

    text: OnReady<Gd<RichTextLabel>>,
}

impl TextScreen {
    pub fn set_text(&mut self, text: String) {
        let msg = format!("[center]{}[/center]", text);
        self.text.set_text(msg.into());
    }

    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state);
    }
}

#[godot_api]
impl IMarginContainer for TextScreen {
    fn init(base: Base<MarginContainer>) -> Self {
        Self {
            base,
            text: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        self.text.init(self.base().get_node_as::<RichTextLabel>(TEXT_PATH));
    }
}
