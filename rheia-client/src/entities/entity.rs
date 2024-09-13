use common::chunks::rotation::Rotation;
use godot::{global::lerp, prelude::*};

use super::{enums::generic_animations::GenericAnimations, generic_skin::GenericSkin};

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct Entity {
    pub base: Base<Node3D>,

    skin: Gd<GenericSkin>,
    target_position: Option<Vector3>,
}

impl Entity {
    pub fn create(base: Base<Node3D>) -> Self {
        Self {
            base,
            skin: Gd::<GenericSkin>::from_init_fn(|base| GenericSkin::create(base)),
            target_position: Default::default(),
        }
    }

    pub fn get_current_animation(&self) -> String {
        self.skin.bind().get_current_animation()
    }

    /// Horizontal degrees of character look
    pub fn get_yaw(&self) -> f32 {
        self.base().get_rotation_degrees().y
    }

    /// Vertical degrees of character look
    pub fn get_pitch(&self) -> f32 {
        self.base().get_rotation_degrees().x
    }

    pub fn change_position(&mut self, position: Vector3) {
        self.target_position = Some(position);
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let mut r = self.base().get_rotation_degrees();
        r.x = rotation.yaw % 360.0;
        r.y = rotation.pitch % 360.0;
        self.base_mut().set_rotation_degrees(r);
    }

    pub fn get_rotation(&self) -> Vector3 {
        self.base().get_rotation()
    }

    pub fn set_rotation(&mut self, euler_radians: Vector3) {
        self.base_mut().set_rotation(euler_radians)
    }

    pub fn get_transform(&self) -> Transform3D {
        self.base().get_transform()
    }

    pub fn trigger_animation(&mut self, animation: GenericAnimations) {
        self.skin.bind_mut().trigger_animation(animation)
    }

    pub fn handle_movement(&mut self, movement: Vector3) {
        // let movement = position - e.get_position();
        self.skin.bind_mut().handle_movement(movement)
    }
}

#[godot_api]
impl INode3D for Entity {
    fn ready(&mut self) {
        let skin = self.skin.clone().upcast();
        self.base_mut().add_child(skin);
    }

    fn process(&mut self, _delta: f64) {
        if let Some(target_position) = self.target_position {
            let current_position = self.base().get_position();

            if current_position.distance_to(target_position) >= 10.0 {
                self.base_mut().set_position(target_position);
                self.target_position = None;
            }

            let l = lerp(
                current_position.to_variant(),
                target_position.to_variant(),
                (0.5).to_variant(),
            );
            let new_position = Vector3::from_variant(&l);
            self.base_mut().set_position(new_position);

            if new_position == target_position {
                self.target_position = None;
            }
        }
    }
}
