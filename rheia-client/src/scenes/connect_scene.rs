use std::net::SocketAddr;

use godot::{
    engine::{Button, Control, IControl, LineEdit, RichTextLabel},
    prelude::*,
};

const ERROR_TEXT_PATH: &str = "VBoxContainer/VBoxContainer/Error";
const INPUT_PATH: &str = "VBoxContainer/VBoxContainer/Input";
const BACK_BUTTON_PATH: &str = "VBoxContainer/VBoxContainer/HBoxContainer/Back";
const CONNECT_BUTTON_PATH: &str = "VBoxContainer/VBoxContainer/HBoxContainer/Connect";

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ConnectScreen {
    base: Base<Control>,

    error_text: OnReady<Gd<RichTextLabel>>,
    input: OnReady<Gd<LineEdit>>,
    back_button: OnReady<Gd<Button>>,
    connect_button: OnReady<Gd<Button>>,
}

#[godot_api]
impl ConnectScreen {
    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state);
    }

    fn set_error_text(&mut self, error: String) {
        let msg = format!("[center][color=#B72828]{}[/color][/center]", error);
        self.error_text.set_text(msg.into());
    }

    #[func]
    fn back_pressed(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn input_text_submitted(&mut self, _input: GString) {
        self.connected_pressed();
    }

    #[func]
    fn connected_pressed(&mut self) {
        let ip_port = self.input.get_text().to_string();
        if let Err(e) = ip_port.parse::<SocketAddr>() {
            self.set_error_text(e.to_string());
            return;
        }
        self.base_mut()
            .emit_signal("direct_ip_connect".into(), &[ip_port.to_variant()]);
        self.base_mut().set_visible(false);
    }

    #[signal]
    fn direct_ip_connect();
}

#[godot_api]
impl IControl for ConnectScreen {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            error_text: OnReady::manual(),
            input: OnReady::manual(),
            back_button: OnReady::manual(),
            connect_button: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        self.error_text
            .init(self.base().get_node_as::<RichTextLabel>(ERROR_TEXT_PATH));
        self.set_error_text("".to_string());

        let mut input = self.base().get_node_as::<LineEdit>(INPUT_PATH);
        input.connect(
            "text_submitted".into(),
            Callable::from_object_method(&self.base().to_godot(), "input_text_submitted"),
        );
        self.input.init(input);

        let mut back = self.base().get_node_as::<Button>(BACK_BUTTON_PATH);
        back.connect(
            "pressed".into(),
            Callable::from_object_method(&self.base().to_godot(), "back_pressed"),
        );
        self.back_button.init(back);

        let mut connect = self.base().get_node_as::<Button>(CONNECT_BUTTON_PATH);
        connect.connect(
            "pressed".into(),
            Callable::from_object_method(&self.base().to_godot(), "connected_pressed"),
        );
        self.connect_button.init(connect);
    }
}
