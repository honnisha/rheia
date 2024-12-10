use godot::{classes::Button, prelude::*};

#[derive(GodotClass)]
#[class(init, base=Button)]
pub struct CustomButton {
    base: Base<Button>,
}
