use godot::{
    classes::{Button, IButton, TextureRect},
    prelude::*,
};

const TAB_BUTTON_SCENE: &str = "res://scenes/ui/tab_button_component.tscn";

#[derive(GodotClass)]
#[class(init, base=Button)]
pub struct TabUIButton {
    base: Base<Button>,

    #[export]
    selected_texture: Option<Gd<TextureRect>>,

    tab_key: String,
}

impl TabUIButton {
    pub fn create(title: &String, tab_key: String) -> Gd<Self> {
        let mut result = load::<PackedScene>(TAB_BUTTON_SCENE).instantiate_as::<Self>();
        result.set_text(title);
        result.bind_mut().tab_key = tab_key;

        let gd = result.bind().to_gd();
        result.connect("pressed", &Callable::from_object_method(&gd, "on_pressed"));

        result
    }
}

#[godot_api]
impl TabUIButton {
    pub fn toggle_highlight(&mut self, state: bool) {
        if let Some(selected_texture) = self.selected_texture.as_mut() {
            selected_texture.set_visible(state);
        }
    }

    #[func]
    fn on_pressed(&mut self) {
        let tab_key = self.tab_key.to_variant();
        self.base_mut().emit_signal("tab_pressed", &[tab_key]);
    }

    #[signal]
    fn tab_pressed();
}

#[godot_api]
impl IButton for TabUIButton {
    fn ready(&mut self) {
        if let Some(selected_texture) = self.selected_texture.as_mut() {
            selected_texture.set_visible(false);
        }
    }
}
