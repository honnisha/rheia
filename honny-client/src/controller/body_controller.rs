use godot::{prelude::*, engine::{Skeleton3D, AnimationPlayer}};

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

        let mut animation_player = generic.get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation_player.call_deferred(StringName::from("play"), &["animation_model_walk".to_variant()]);

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
