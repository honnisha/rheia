use godot::{
    engine::{IMarginContainer, MarginContainer, RichTextLabel},
    prelude::*,
};

const TEXT_PATH: &str = "Text";

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct TextScreen {
    base: Base<MarginContainer>,

    text: Option<Gd<RichTextLabel>>,
}

impl TextScreen {
    pub fn set_text(&mut self, text: String) {
        let msg = format!("[center]{}[/center]", text);
        self.text
            .as_mut()
            .expect("Text inside TextScreen is not initialized")
            .set_text(msg.into());
    }

    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state);
    }
}

#[godot_api]
impl IMarginContainer for TextScreen {
    fn init(base: Base<MarginContainer>) -> Self {
        Self { base, text: None }
    }

    fn ready(&mut self) {
        match self.base().try_get_node_as::<RichTextLabel>(TEXT_PATH) {
            Some(e) => self.text = Some(e),
            _ => panic!("console_text element not found"),
        };
    }
}
