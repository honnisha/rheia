use godot::{
    classes::{PanelContainer, RichTextLabel},
    prelude::*,
};

const ITEM_DESCRIPTION_SCENE: &str = "res://scenes/ui/item_decription.tscn";

#[derive(GodotClass)]
#[class(init, base=PanelContainer)]
pub struct ItemDescription {
    base: Base<PanelContainer>,

    #[export]
    text: Option<Gd<RichTextLabel>>,
}

#[godot_api]
impl ItemDescription {
    pub fn create() -> Gd<Self> {
        let mut item = load::<PackedScene>(ITEM_DESCRIPTION_SCENE).instantiate_as::<Self>();
        item.bind_mut().set_description(&"".to_string());
        item
    }

    pub fn set_description(&mut self, new_text: &String) {
        if let Some(text) = self.text.as_mut() {
            text.set_text(new_text);
        }
    }
}
