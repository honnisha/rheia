use godot::{
    classes::{BoxContainer, Button},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=BoxContainer)]
pub struct MainMenuButton {
    base: Base<BoxContainer>,

    #[export]
    pub text: Option<Gd<Button>>,
}
