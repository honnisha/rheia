use godot::{
    classes::{Control, IControl},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct BlockIcon {
    base: Base<Control>,

    #[export]
    pub block_anchor: Option<Gd<Node3D>>,
}

impl BlockIcon {}

#[godot_api]
impl IControl for BlockIcon {
    fn ready(&mut self) {
        for child in self.block_anchor.as_mut().unwrap().get_children().iter_shared() {
            child.free();
        }
    }
}
