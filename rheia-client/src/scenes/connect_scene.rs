use common::utils::validate_username;
use godot::{
    classes::{Button, Control, IControl, LineEdit, RichTextLabel},
    prelude::*,
};
use network::client::resolve_connect_domain_sync;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct ConnectScreen {
    base: Base<Control>,

    #[export]
    error_text: Option<Gd<RichTextLabel>>,

    #[export]
    input: Option<Gd<LineEdit>>,

    #[export]
    username_input: Option<Gd<LineEdit>>,

    #[export]
    back_button: Option<Gd<Button>>,

    #[export]
    connect_button: Option<Gd<Button>>,
}

#[godot_api]
impl ConnectScreen {
    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state);
    }

    #[func]
    fn back_pressed(&mut self) {
        self.toggle(false);
    }

    #[func]
    fn input_text_submitted(&mut self, _input: GString) {
        self.connected_pressed();
    }

    fn set_error_msg(&mut self, error: String) {
        let msg = format!("[color=#B72828]{}[/color]", error);
        self.error_text.as_mut().unwrap().set_text(&msg);
    }

    #[func]
    fn connected_pressed(&mut self) {
        self.error_text.as_mut().unwrap().set_text(&"".to_string());

        let ip_port = self.input.as_mut().unwrap().get_text().to_string();
        if let Err(e) = resolve_connect_domain_sync(&ip_port, 25565_u16) {
            self.set_error_msg(e.to_string());
            return;
        }

        let username = self.username_input.as_mut().unwrap().get_text().to_string();
        if !validate_username(&username) {
            self.set_error_msg("Bad username".to_string());
            return;
        }

        self.base_mut()
            .emit_signal("direct_ip_connect", &[ip_port.to_variant(), username.to_variant()]);
        self.base_mut().set_visible(false);
    }

    pub fn set_ip(&mut self, ip_port: &String) {
        self.input.as_mut().unwrap().set_text(ip_port);
    }

    pub fn set_username(&mut self, username: &String) {
        self.username_input.as_mut().unwrap().set_text(username);
    }

    #[signal]
    fn direct_ip_connect();
}

#[godot_api]
impl IControl for ConnectScreen {
    fn ready(&mut self) {
        self.toggle(false);

        self.set_error_msg("".to_string());

        let mut input = self.input.as_mut().unwrap().clone();
        input.connect(
            "text_submitted",
            &Callable::from_object_method(&self.base().to_godot(), "input_text_submitted"),
        );

        let mut back_button = self.back_button.as_mut().unwrap().clone();
        back_button.connect(
            "pressed",
            &Callable::from_object_method(&self.base().to_godot(), "back_pressed"),
        );

        let mut connect_button = self.connect_button.as_mut().unwrap().clone();
        connect_button.connect(
            "pressed",
            &Callable::from_object_method(&self.base().to_godot(), "connected_pressed"),
        );
    }
}
