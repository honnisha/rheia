use godot::{
    classes::{input::MouseMode, Button, Control, IControl, Input, RichTextLabel}, meta::AsArg, prelude::*
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
    pub fn window_closed();

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

        if state {
            Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        } else {
            Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        }
    }

    pub fn is_visible(&self) -> bool {
        self.base().is_visible()
    }

    pub fn add_component(&mut self, node: impl AsArg<Option<Gd<Node>>>) {
        self.content_holder.as_mut().unwrap().add_child(node);
    }

    #[func]
    fn on_close_button_pressed(&mut self) {
        self.signals().window_closed().emit();
        self.toggle(false);
    }
}

#[godot_api]
impl IControl for WindowUIComponent {
    fn ready(&mut self) {
        let gd = self.to_gd().clone();
        if let Some(close_button) = self.close_button.as_mut() {
            close_button
                .signals()
                .pressed()
                .connect_other(&gd, WindowUIComponent::on_close_button_pressed);
            close_button.set_visible(self.show_button_close);
        }
    }
}
