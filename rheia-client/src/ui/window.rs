use godot::{
    classes::{Button, Control, IControl, RichTextLabel},
    meta::AsObjectArg,
    prelude::*,
};

const WINDOW_SCENE: &str = "res://scenes/ui/window.tscn";

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct WindowUIComponent {
    base: Base<Control>,

    show_button_close: bool,

    #[export]
    content_holder: Option<Gd<Control>>,

    #[export]
    title_component: Option<Gd<RichTextLabel>>,

    #[export]
    close_button: Option<Gd<Button>>,
}

#[godot_api]
impl WindowUIComponent {
    #[signal]
    fn closed();

    pub fn create(title: String, show_button_close: bool) -> Gd<Self> {
        let mut window = load::<PackedScene>(WINDOW_SCENE).instantiate_as::<Self>();
        window.bind_mut().show_button_close = show_button_close;

        if let Some(title_component) = window.bind_mut().title_component.as_mut() {
            title_component.set_text(&title);
        }
        window
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

    #[func]
    fn on_close_button_pressed(&mut self) {
        self.toggle(false);
        self.signals().closed().emit();
    }
}

#[godot_api]
impl IControl for WindowUIComponent {
    fn ready(&mut self) {
        let gd = self.base().to_godot();
        if let Some(close_button) = self.close_button.as_mut() {
            close_button.connect(
                "pressed",
                &Callable::from_object_method(&gd, "on_close_button_pressed"),
            );
            close_button.set_visible(self.show_button_close);
        }
    }
}
