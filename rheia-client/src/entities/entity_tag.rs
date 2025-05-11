use godot::{
    classes::{base_material_3d::BillboardMode, Label3D},
    prelude::*,
};
use network::messages::EntityTag as NetoworkEntityTag;

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct EntityTag {
    pub(crate) base: Base<Node3D>,

    label: Gd<Label3D>,
}

impl EntityTag {
    pub fn create(base: Base<Node3D>, tag: NetoworkEntityTag) -> Self {
        let mut label = Label3D::new_alloc();
        label.set_billboard_mode(BillboardMode::ENABLED);
        let mut tag_comp = Self { base, label };
        tag_comp.update(tag);
        tag_comp
    }

    pub fn update(&mut self, tag: NetoworkEntityTag) {
        self.label.set_text(tag.get_content());
        self.label
            .set_position(Vector3::new(0.0, tag.get_offset().clone(), 0.0));
    }
}

#[godot_api]
impl INode3D for EntityTag {
    fn ready(&mut self) {
        let mut base = self.base_mut().clone();
        base.add_child(&self.label);
    }
}
