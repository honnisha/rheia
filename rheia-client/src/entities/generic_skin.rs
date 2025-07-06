use ahash::HashMap;
use godot::{classes::AnimationPlayer, prelude::*};
use lazy_static::lazy_static;

use super::enums::{generic_animations::GenericAnimations, generic_body_parts::BodyPart};

const GENERIC_MODEL: &str = "res://assets/models/generic/generic.glb";

type PartsType = HashMap<&'static str, &'static str>;
lazy_static! {
    static ref PARTS_CHEST: PartsType = {
        let mut m = HashMap::default();
        m.insert("Node2/root/b_torso_lower/b_torso_middle", "torso_middle");
        m.insert("Node2/root/b_torso_lower/b_torso_middle/b_torso_upper", "torso_upper");
        m.insert(
            "Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_hand_right_shoulder",
            "hand_right_shoulder",
        );
        m.insert(
            "Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_hand_right_shoulder/b_hand_right_elbow",
            "hand_right_elbow",
        );
        m.insert(
            "Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_hand_left_shoulder",
            "hand_left_shoulder",
        );
        m.insert(
            "Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_hand_left_shoulder/b_hand_left_elbow",
            "hand_left_elbow",
        );
        m
    };
    static ref PARTS_HANDS: PartsType = {
        let mut m = HashMap::default();
        m.insert("Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_hand_right_shoulder/b_hand_right_elbow/b_hand_right_fist", "hand_right_fist");
        m.insert(
            "Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_hand_right_shoulder/b_hand_right_elbow",
            "hand_right_wrist",
        );
        m.insert("Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_hand_left_shoulder/b_hand_left_elbow/b_hand_left_fist", "hand_left_fist");
        m.insert(
            "Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_hand_left_shoulder/b_hand_left_elbow",
            "hand_left_wrist",
        );
        m
    };
    static ref PARTS_PANTS: PartsType = {
        let mut m = HashMap::default();
        m.insert("Node2/root/b_torso_lower", "torso_lower");
        m.insert("Node2/root/b_torso_lower/b_leg_right_hip", "leg_right_hip");
        m.insert(
            "Node2/root/b_torso_lower/b_leg_right_hip/b_leg_right_shin",
            "leg_right_shin",
        );
        m.insert("Node2/root/b_torso_lower/b_leg_left_hip", "leg_left_hip");
        m.insert(
            "Node2/root/b_torso_lower/b_leg_left_hip/b_leg_left_shin",
            "leg_left_shin",
        );
        m
    };
    static ref PARTS_BOOTS: PartsType = {
        let mut m = HashMap::default();
        m.insert(
            "Node2/root/b_torso_lower/b_leg_right_hip/b_leg_right_shin/b_leg_right_foot",
            "leg_right_foot",
        );
        m.insert(
            "Node2/root/b_torso_lower/b_leg_right_hip/b_leg_right_shin",
            "leg_right_licorice",
        );
        m.insert(
            "Node2/root/b_torso_lower/b_leg_left_hip/b_leg_left_shin/b_leg_left_foot",
            "leg_left_foot",
        );
        m.insert(
            "Node2/root/b_torso_lower/b_leg_left_hip/b_leg_left_shin",
            "leg_left_licorice",
        );
        m
    };
    static ref PARTS_HEAD: PartsType = {
        let mut m = HashMap::default();
        m.insert("Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_neck", "neck");
        m.insert(
            "Node2/root/b_torso_lower/b_torso_middle/b_torso_upper/b_neck/b_head",
            "head",
        );
        m
    };
}

fn get_parts(part: &BodyPart) -> &'static PartsType {
    match part {
        BodyPart::Chest => &PARTS_CHEST,
        BodyPart::Hands => &PARTS_HANDS,
        BodyPart::Pants => &PARTS_PANTS,
        BodyPart::Boots => &PARTS_BOOTS,
        BodyPart::Head => &PARTS_HEAD,
    }
}
/// Responsible for controlling the full-length generic model
#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct GenericSkin {
    pub(crate) base: Base<Node3D>,

    generic: Gd<Node3D>,
    animation_player: Gd<AnimationPlayer>,
}

impl GenericSkin {
    pub fn create(base: Base<Node3D>) -> Self {
        let generic = load::<PackedScene>(GENERIC_MODEL).instantiate_as::<Node3D>();

        let animation_player = generic.get_node_as::<AnimationPlayer>("AnimationPlayer");
        // let mut animation = animation_player
        //     .get_animation(StringName::from(GenericAnimations::Idle.to_string()))
        //     .unwrap();
        // animation.set_loop_mode(LoopMode::LINEAR);
        Self {
            base,
            generic,
            animation_player,
        }
    }

    fn play_animation(&mut self, animation: GenericAnimations) {
        self.animation_player
            .call_deferred(&StringName::from("play"), &[animation.to_string().to_variant()]);
    }

    pub fn get_current_animation(&self) -> String {
        self.animation_player.get_current_animation().to_string()
    }

    pub fn handle_movement(&mut self, movement: Vector3) {
        let falling = movement.y < -0.01;

        if falling {
            self.play_animation(GenericAnimations::Fall);
        }
        // Moving horizontally
        else if movement.x.abs() > 0.01 || movement.z.abs() > 0.01 {
            self.play_animation(GenericAnimations::Walk);
        } else {
            self.play_animation(GenericAnimations::Idle);
        }
    }

    pub fn trigger_animation(&mut self, animation: GenericAnimations) {
        self.play_animation(animation);
    }

    fn replace(&mut self, source_model: &Node3D, part: BodyPart) -> Result<(), String> {
        let parts = get_parts(&part);
        for (bone_path, mesh_prefix) in parts.iter() {
            // Get original bone
            let mut bone = match self.generic.try_get_node_as::<Node3D>(*bone_path) {
                Some(node) => node,
                None => return Err(format!("Generic part:{} node bone \"{}\" not found", part, bone_path)),
            };

            // Get target bone
            let mut target_bone = match source_model.try_get_node_as::<Node3D>(*bone_path) {
                Some(node) => node,
                None => {
                    return Err(format!(
                        "Replace target part:{} node bone \"{}\" not found",
                        part, bone_path
                    ))
                }
            };

            // Remove all original meshes
            for mut orig_child in bone.get_children().iter_shared() {
                if orig_child.get_name().to_string().starts_with(mesh_prefix) {
                    orig_child.queue_free();
                }
            }

            // Append meshes from target bone
            for target_child in target_bone.get_children().iter_shared() {
                if target_child.get_name().to_string().starts_with(mesh_prefix) {
                    target_bone.remove_child(&target_child);
                    bone.add_child(&target_child);
                    // target_child.reparent(bone.clone().upcast());
                }
            }
        }
        Ok(())
    }
}

#[godot_api]
impl INode3D for GenericSkin {
    fn ready(&mut self) {
        let generic = self.generic.clone();
        self.base_mut().add_child(&generic);
    }
}
