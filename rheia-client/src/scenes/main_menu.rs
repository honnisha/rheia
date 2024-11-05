use super::{connect_scene::ConnectScreen, main_scene::MainScene, text_screen::TextScreen};
use crate::{logger::CONSOLE_LOGGER, utils::settings::GameSettings};
use godot::{
    engine::{BoxContainer, Button, Control, Engine, RichTextLabel},
    prelude::*,
};
use std::{cell::RefCell, rc::Rc};

const GUI_PATH: &str = "GUI";
const BUTTONS_HOLDER_PATH: &str = "GUI/VBoxContainer/BottomHalf/ButtonsHolder";
const BOTTOM_TEXT_PATH: &str = "GUI/VBoxContainer/BottomHalf/MarginContainer/BottomText";

const TEXT_CONNECT: &str = "Connect";
const TEXT_EXIT: &str = "Exit";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainMenu {
    base: Base<Node>,

    gui: OnReady<Gd<Control>>,
    buttons_holder: OnReady<Gd<BoxContainer>>,
    connect_screen: OnReady<Gd<ConnectScreen>>,
    bottom_text: OnReady<Gd<RichTextLabel>>,
    text_screen: Gd<TextScreen>,
    main_scene: Option<Gd<MainScene>>,
    game_settings: Rc<RefCell<GameSettings>>,
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
        if let Some(ip_port) = self.game_settings.borrow().ip_port_direct_connect.as_ref() {
            self.connect_screen.bind_mut().set_ip(ip_port);
        }

        self.connect_screen.bind_mut().toggle(true);
    }

    #[func]
    fn on_direct_ip_connect(&mut self, ip_port: GString) {
        {
            let mut game_settings = self.game_settings.borrow_mut();
            game_settings.ip_port_direct_connect = Some(ip_port.to_string());
            game_settings.save().expect("Settings save error");
        }

        self.gui.set_visible(false);

        let mut main_scene = load::<PackedScene>("res://scenes/main_scene.tscn").instantiate_as::<MainScene>();
        main_scene.bind_mut().set_ip(ip_port.to_string());
        main_scene.connect(
            "disconnect".into(),
            Callable::from_object_method(&self.base().to_godot(), "on_disconnect"),
        );
        self.main_scene = Some(main_scene.clone());
        self.base_mut().add_child(main_scene.upcast());
    }

    #[func]
    fn on_disconnect(&mut self, message: GString) {
        if let Some(mut main_scene) = self.main_scene.take() {
            main_scene.queue_free();
        }

        if message.len() > 0 {
            let mut text_screen = self.text_screen.bind_mut();
            text_screen.set_text(message.to_string());
            text_screen.toggle(true);
        }

        self.gui.set_visible(true);
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

    fn read_settings(&mut self) {
        match GameSettings::read() {
            Ok(s) => {
                let mut game_settings = self.game_settings.borrow_mut();
                *game_settings = s;
            }
            Err(e) => {
                let mut text_screen = self.text_screen.bind_mut();
                text_screen.set_text(format!(
                    "Settings read error: {}\nThe default settings will be used.",
                    e
                ));
                text_screen.toggle(true);
            }
        }
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
            bottom_text: OnReady::manual(),
            text_screen: load::<PackedScene>("res://scenes/text_screen.tscn").instantiate_as::<TextScreen>(),
            main_scene: None,
            game_settings: Rc::new(RefCell::new(GameSettings::default())),
        }
    }

    fn ready(&mut self) {
        if let Err(e) = log::set_logger(&CONSOLE_LOGGER) {
            log::error!(target: "main", "log::set_logger error: {}", e)
        }
        log::set_max_level(log::LevelFilter::Debug);

        log::info!(target: "main", "Loading Rheia version: {}", VERSION);

        let mut text_screen = self.text_screen.clone();
        text_screen.connect(
            "close_button_pressed".into(),
            Callable::from_object_method(&self.base().to_godot(), "on_text_screen_closed"),
        );
        self.base_mut().add_child(text_screen.upcast());
        self.text_screen.bind_mut().toggle(false);
        self.text_screen
            .bind_mut()
            .toggle_close_button(Some("To main menu".to_string()));

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

        self.bottom_text
            .init(self.base().get_node_as::<RichTextLabel>(BOTTOM_TEXT_PATH));
        self.bottom_text.set_text(format!("Version: {}", VERSION).into());

        self.read_settings();
    }
}
