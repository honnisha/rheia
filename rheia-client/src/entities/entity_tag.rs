use godot::{
    classes::{base_material_3d::BillboardMode, Font, Label3D},
    prelude::*,
};
use network::messages::NetworkEntityTag;

pub const DEFAULT_FONT_PATH: &str = "res://assets/gui/fonts/Monocraft/Monocraft.otf";

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct EntityTag {
    pub(crate) base: Base<Node3D>,

    label: Gd<Label3D>,
}

impl EntityTag {
    pub fn create(base: Base<Node3D>, tag: NetworkEntityTag) -> Self {
        let mut label = Label3D::new_alloc();

        let default_font = load::<Font>(DEFAULT_FONT_PATH);
        label.set_font(&default_font);

        label.set_billboard_mode(BillboardMode::ENABLED);
        let mut tag_comp = Self { base, label };
        tag_comp.update(tag);
        tag_comp
    }

    pub fn update(&mut self, tag: NetworkEntityTag) {
        self.label.set_text(tag.get_content());
        self.label.set_font_size(tag.get_font_size().clone());
        self.label.set_outline_size(tag.get_outline_size().clone());
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
