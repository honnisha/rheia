use godot::{classes::Control, meta::AsObjectArg, prelude::*};

const WINDOW_SCENE: &str = "res://scenes/ui/window.tscn";

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct WindowUIComponent {
    base: Base<Control>,

    #[export]
    content_holder: Option<Gd<Control>>,
}

#[godot_api]
impl WindowUIComponent {
    pub fn create() -> Gd<Self> {
        load::<PackedScene>(WINDOW_SCENE).instantiate_as::<Self>()
    }

    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state);
    }

    pub fn is_visible(&self) -> bool {
        self.base().is_visible()
    }

    pub fn add_component(&mut self, node: impl AsObjectArg<Node>) {
        self.content_holder.as_mut().unwrap().add_child(node);
    }
}
