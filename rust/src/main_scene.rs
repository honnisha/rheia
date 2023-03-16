use godot::engine::node::InternalMode;
use godot::engine::packed_scene::GenEditState;
use godot::engine::{Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;
use rand::Rng as _;
use std::f64::consts::PI;

// Deriving GodotClass makes the class available to Godot
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl Main {
    #[func]
    pub fn new_game(&mut self) {
        godot_print!("New game");
    }
}

#[godot_api]
impl GodotExt for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("Ready");
    }
}

/// Root here is needs to be the same type (or a parent type) of the node that you put in the child
///   scene as the root. For instance Spatial is used for this example.
fn instantiate_scene<Root>(scene: &PackedScene) -> Gd<Root>
where
    Root: GodotClass + Inherits<Node>,
{
    let s = scene
        .instantiate(GenEditState::GEN_EDIT_STATE_DISABLED)
        .expect("scene instantiated");

    s.cast::<Root>()
}
