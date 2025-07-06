use super::{
    connect_scene::ConnectScreen, main_menu_button::MainMenuButton, main_scene::MainScene, text_screen::TextScreen,
};
use crate::{LOG_LEVEL, logger::CONSOLE_LOGGER, utils::settings::GameSettings};
use godot::{
    classes::{BoxContainer, Control, Engine, IControl, RichTextLabel},
    meta::AsArg,
    prelude::*,
};
use std::{cell::RefCell, rc::Rc};

const TEXT_CONNECT: &str = "Connect";
const TEXT_EXIT: &str = "Exit";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct MainMenu {
    base: Base<Control>,

    #[export]
    gui: Option<Gd<Control>>,

    #[export]
    buttons_holder: Option<Gd<BoxContainer>>,

    #[export]
    bottom_text: Option<Gd<RichTextLabel>>,

    #[export]
    menu_button: Option<Gd<PackedScene>>,

    #[export]
    text_screen_scene: Option<Gd<PackedScene>>,
    #[init(val = OnReady::manual())]
    text_screen: OnReady<Gd<TextScreen>>,

    #[init(val = Rc::new(RefCell::new(GameSettings::default())))]
    game_settings: Rc<RefCell<GameSettings>>,

    #[export]
    connect_screen_scene: Option<Gd<PackedScene>>,
    #[init(val = OnReady::manual())]
    connect_screen: OnReady<Gd<ConnectScreen>>,

    main_scene: Option<Gd<MainScene>>,
}

impl MainMenu {
    fn add_button<S: AsArg<StringName>>(&mut self, label: &str, handler: S) {
        let mut menu_button = self.menu_button.as_ref().unwrap().instantiate_as::<MainMenuButton>();

        {
            let mut m = menu_button.bind_mut();
            let button_text = m.text.as_mut().unwrap();
            button_text.set_text(label);
            button_text.connect(
                "pressed",
                &Callable::from_object_method(&self.base().to_godot(), handler),
            );
        }

        self.buttons_holder.as_mut().unwrap().add_child(&menu_button);
    }
}

#[godot_api]
impl MainMenu {
    #[func]
    fn connect_pressed(&mut self) {
        if let Some(ip_port) = self.game_settings.borrow().ip_port_direct_connect.as_ref() {
            self.connect_screen.bind_mut().set_ip(ip_port);
        }

        if let Some(username) = self.game_settings.borrow().username.as_ref() {
            self.connect_screen.bind_mut().set_username(username);
        }

        self.connect_screen.bind_mut().toggle(true);
    }

    #[func]
    fn on_direct_ip_connect(&mut self, ip_port: GString, username: GString) {
        {
            let mut game_settings = self.game_settings.borrow_mut();
            game_settings.ip_port_direct_connect = Some(ip_port.to_string());
            game_settings.username = Some(username.to_string());
            game_settings.save().expect("Settings save error");
        }

        self.gui.as_mut().unwrap().set_visible(false);

        let main_scene = MainScene::create(ip_port.to_string(), username.to_string(), self.game_settings.clone());
        main_scene
            .signals()
            .network_disconnect()
            .connect_other(&self.to_gd(), MainMenu::on_network_disconnect);
        self.base_mut().add_child(&main_scene);
        self.main_scene = Some(main_scene);
    }

    #[func]
    fn on_network_disconnect(&mut self, message: GString) {
        if let Some(mut main_scene) = self.main_scene.take() {
            main_scene.queue_free();
        }

        if message.len() > 0 {
            let mut text_screen = self.text_screen.bind_mut();
            text_screen.update_text(message.to_string());
            text_screen.toggle(true);
        }

        self.gui.as_mut().unwrap().set_visible(true);
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
                text_screen.update_text(format!(
                    "Settings read error: {}\nThe default settings will be used.",
                    e
                ));
                text_screen.toggle(true);
            }
        }
    }
}

#[godot_api]
impl IControl for MainMenu {
    fn ready(&mut self) {
        if let Err(e) = log::set_logger(&CONSOLE_LOGGER) {
            log::error!(target: "main", "log::set_logger error: {}", e)
        }
        log::set_max_level(LOG_LEVEL);

        log::info!(target: "main", "Loading Rheia version: {}", VERSION);

        #[cfg(feature = "trace")]
        log::info!(target: "main", "&6Tracy enabled");

        Engine::singleton().set_max_fps(60);

        let mut text_screen = self.text_screen_scene.as_mut().unwrap().instantiate_as::<TextScreen>();
        text_screen.connect(
            "close_button_pressed",
            &Callable::from_object_method(&self.base().to_godot(), "on_text_screen_closed"),
        );
        self.base_mut().add_child(&text_screen);
        self.text_screen.init(text_screen);

        self.text_screen.bind_mut().toggle(false);
        self.text_screen
            .bind_mut()
            .toggle_close_button(Some("To main menu".to_string()));

        for child in self.buttons_holder.as_mut().unwrap().get_children().iter_shared() {
            child.free();
        }

        self.add_button(TEXT_CONNECT, "connect_pressed");
        self.add_button(TEXT_EXIT, "exit_pressed");

        let mut connect_screen = self
            .connect_screen_scene
            .as_ref()
            .unwrap()
            .instantiate_as::<ConnectScreen>();
        connect_screen.connect(
            "direct_ip_connect",
            &Callable::from_object_method(&self.base().to_godot(), "on_direct_ip_connect"),
        );
        connect_screen.bind_mut().toggle(false);
        self.connect_screen.init(connect_screen.clone());
        self.base_mut().add_child(&connect_screen);

        self.bottom_text
            .as_mut()
            .unwrap()
            .set_text(&format!("Version: {}", VERSION));

        self.read_settings();
    }
}
