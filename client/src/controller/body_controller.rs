use godot::prelude::*;

/// Responsible for controlling the full-length generic model
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct BodyController {
    #[base]
    pub(crate) base: Base<Node3D>,

    generic: Gd<Node3D>,
}

impl BodyController {
    pub fn create(base: Base<Node3D>) -> Self {
        let generic = load::<PackedScene>("res://assets/models/generic/generic.tscn").instantiate_as::<Node3D>();
        Self {
            base,
            generic,
        }
    }
}


#[godot_api]
impl NodeVirtual for BodyController {
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base)
    }

    fn ready(&mut self) {
        self.base.add_child(self.generic.share().upcast());
    }

    fn process(&mut self, _delta: f64) {}
}
