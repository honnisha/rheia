use godot::{
    engine::{LineEdit, RichTextLabel, TextureButton, MarginContainer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct Console {
    #[base]
    base: Base<MarginContainer>,
    console_text: Option<Gd<RichTextLabel>>,
    console_input: Option<Gd<LineEdit>>,
    console_button: Option<Gd<TextureButton>>,
    console_sugestions: Option<Gd<RichTextLabel>>,
}

#[godot_api]
impl Console {
    fn scroll_to_bottom(&mut self) {
        let c = self.console_text.as_mut().unwrap();
        let lines = c.get_line_count();
        c.scroll_to_line(lines - 1);
    }

    #[func]
    fn button_pressed(&mut self) {
        self.scroll_to_bottom();
        godot_print!("Pressed;");
    }

    #[func]
    fn text_changed(&mut self, new_text: GodotString) {
        godot_print!("new_text: {}", new_text);
    }

    #[func]
    fn text_submitted(&mut self, new_text: GodotString) {
        godot_print!("text_submitted");
    }
}

#[godot_api]
impl NodeVirtual for Console {
    fn init(base: Base<MarginContainer>) -> Self {
        Console {
            console_text: None,
            console_input: None,
            console_button: None,
            console_sugestions: None,
            base: base,
        }
    }
    fn ready(&mut self) {
        match self.base.try_get_node_as::<RichTextLabel>("VBoxContainer/HBoxContainer/ConsoleBackground/MarginContainer/ConsoleText") {
            Some(e) => self.console_text = Some(e),
            _ => {
                godot_error!("console_text element not found");
                return;
            }
        };
        match self.base.try_get_node_as::<LineEdit>("VBoxContainer/HBoxContainer2/TextureRect/ConsoleInput") {
            Some(e) => {
                self.console_input = Some(e);
                self.console_input.as_mut().unwrap().connect(
                    "text_changed".into(),
                    Callable::from_object_method(self.base.share(), "text_changed"),
                    0,
                );
                self.console_input.as_mut().unwrap().connect(
                    "text_submitted".into(),
                    Callable::from_object_method(self.base.share(), "text_submitted"),
                    0,
                );
            }
            _ => {
                godot_error!("console_input element not found");
                return;
            }
        };
        match self.base.try_get_node_as::<TextureButton>("VBoxContainer/HBoxContainer2/ConsoleButton") {
            Some(e) => {
                self.console_button = Some(e);
                self.console_button.as_mut().unwrap().connect(
                    "pressed".into(),
                    Callable::from_object_method(self.base.share(), "button_pressed"),
                    0,
                );
            },
            _ => {
                godot_error!("console_button element not found");
                return;
            }
        };
        match self.base.try_get_node_as::<RichTextLabel>("GUIControl/MarginContainer/ConsoleContainer/VBoxContainer/HBoxContainer3/MarginContainer/ConsoleSugestioins") {
            Some(e) => self.console_sugestions = Some(e),
            _ => {
                godot_error!("console_sugestions element not found");
                return;
            }
        };
        godot_print!("Console successfily loaded;");
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        let input = Input::singleton();
        if input.is_action_just_pressed("ui_up".into(), false) {
            godot_print!("up");
        }
        else if input.is_action_just_pressed("ui_down".into(), false) {
            godot_print!("down");
        }
        else if input.is_action_just_pressed("ui_focus_next".into(), false) {
            godot_print!("tab");
        }
    }
}
