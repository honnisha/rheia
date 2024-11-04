use godot::{
    engine::{BoxContainer, Button, Control, Engine},
    prelude::*,
};

use super::{connect_scene::ConnectScreen, main_scene::MainScene, text_screen::TextScreen};

const GUI_PATH: &str = "GUI";
const BUTTONS_HOLDER_PATH: &str = "GUI/VBoxContainer/BottomHalf/ButtonsHolder";

const TEXT_CONNECT: &str = "Connect";
const TEXT_EXIT: &str = "Exit";

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainMenu {
    base: Base<Node>,

    gui: OnReady<Gd<Control>>,
    buttons_holder: OnReady<Gd<BoxContainer>>,
    connect_screen: OnReady<Gd<ConnectScreen>>,
    text_screen: Gd<TextScreen>,
    main_scene: Option<Gd<MainScene>>,
}

impl MainMenu {
    fn add_button<S: Into<StringName>>(&mut self, label: &str, handler: S) {
        let menu_button =
            load::<PackedScene>("res://scenes/components/menu_button.tscn").instantiate_as::<BoxContainer>();

        let mut button_text = menu_button.find_child("Text".into()).unwrap().cast::<Button>();
        button_text.set_text(label.into());
        button_text.connect(
            "pressed".into(),
            Callable::from_object_method(&self.base().to_godot(), handler),
        );
        self.buttons_holder.add_child(menu_button.upcast());
    }
}

#[godot_api]
impl MainMenu {
    #[func]
    fn connect_pressed(&mut self) {
        self.connect_screen.bind_mut().toggle(true);
    }

    #[func]
    fn on_disconnect(&mut self, message: GString) {
        self.main_scene.take().unwrap().queue_free();

        let botton_text = if message.len() == 0 {
            None
        } else {
            Some("To main menu".to_string())
        };
        let mut text_screen = self.text_screen.bind_mut();
        text_screen.toggle_close_button(botton_text);
        text_screen.set_text(message.to_string());
        text_screen.toggle(true);
        self.gui.set_visible(true);
    }

    #[func]
    fn on_direct_ip_connect(&mut self, ip_port: GString) {
        self.gui.set_visible(false);

        let mut main_scene = load::<PackedScene>("res://scenes/main_scene.tscn").instantiate_as::<MainScene>();
        main_scene.bind_mut().set_ip(ip_port.to_string());
        main_scene.connect(
            "disconnect".into(),
            Callable::from_object_method(&self.base().to_godot(), "on_disconnect"),
        );
        self.base_mut().add_child(main_scene.clone().upcast());
        self.main_scene = Some(main_scene);
    }

    #[func]
    fn exit_pressed(&mut self) {
        Engine::singleton()
            .get_main_loop()
            .expect("main loop is not found")
            .cast::<SceneTree>()
            .quit();
    }

    #[func]
    fn on_text_screen_closed(&mut self) {
        self.text_screen.bind_mut().toggle(false);
    }
}

#[godot_api]
impl INode for MainMenu {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            gui: OnReady::manual(),
            buttons_holder: OnReady::manual(),
            connect_screen: OnReady::manual(),
            text_screen: load::<PackedScene>("res://scenes/text_screen.tscn").instantiate_as::<TextScreen>(),
            main_scene: None,
        }
    }

    fn ready(&mut self) {
        let mut text_screen = self.text_screen.clone();
        text_screen.connect(
            "close_button_pressed".into(),
            Callable::from_object_method(&self.base().to_godot(), "on_text_screen_closed"),
        );
        self.base_mut().add_child(text_screen.upcast());
        self.text_screen.bind_mut().toggle(false);

        self.gui.init(self.base().get_node_as::<Control>(GUI_PATH));

        self.buttons_holder
            .init(self.base().get_node_as::<BoxContainer>(BUTTONS_HOLDER_PATH));

        for child in self.buttons_holder.get_children().iter_shared() {
            child.free();
        }

        self.add_button(TEXT_CONNECT, "connect_pressed");
        self.add_button(TEXT_EXIT, "exit_pressed");

        let mut connect_screen =
            load::<PackedScene>("res://scenes/connect_screen.tscn").instantiate_as::<ConnectScreen>();
        connect_screen.connect(
            "direct_ip_connect".into(),
            Callable::from_object_method(&self.base().to_godot(), "on_direct_ip_connect"),
        );
        connect_screen.bind_mut().toggle(false);
        self.connect_screen.init(connect_screen.clone());
        self.base_mut().add_child(connect_screen.upcast());
    }
}
