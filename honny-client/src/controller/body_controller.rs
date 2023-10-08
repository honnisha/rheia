use std::{fs::File, os::unix::prelude::FileExt, io::Read};

use godot::{prelude::*, engine::{AnimationPlayer, animation::LoopMode, MeshInstance3D, GltfDocument, GltfState}};

const GENERIC_MODEL: &str = "res://assets/models/generic/generic.glb";

enum ArmorParts {
    Chest,
    Hands,
    Pants,
    Boots,
}

const PARTS_CHEST: &'static [&str] = &[
    "Node2/root/torso_lower2/torso_middle2/torso_middle",
    "Node2/root/torso_lower2/torso_middle2/torso_upper2/torso_upper",
    "Node2/root/torso_lower2/torso_middle2/torso_upper2/hand_left_shoulder2/hand_left_shoulder",
    "Node2/root/torso_lower2/torso_middle2/torso_upper2/hand_left_shoulder2/hand_left_wrist2/hand_left_wrist",
    "Node2/root/torso_lower2/torso_middle2/torso_upper2/hand_right_shoulder2/hand_right_shoulder",
    "Node2/root/torso_lower2/torso_middle2/torso_upper2/hand_right_shoulder2/hand_right_wrist2/hand_right_wrist",
];
const PARTS_HANDS: &'static [&str] = &[
    "Node2/root/torso_lower2/torso_middle2/torso_upper2/hand_right_shoulder2/hand_right_wrist2/hand_right_fist2/hand_right_fist",
    "Node2/root/torso_lower2/torso_middle2/torso_upper2/hand_left_shoulder2/hand_left_wrist2/hand_left_fist2/hand_left_fist",
];
const PARTS_PANTS: &'static [&str] = &[
    "Node2/root/torso_lower2/torso_lower",
    "Node2/root/torso_lower2/leg_right_hip2/leg_right_hip",
    "Node2/root/torso_lower2/leg_right_hip2/leg_right_shin2/leg_right_shin",
    "Node2/root/torso_lower2/leg_left_hip2/leg_left_hip",
    "Node2/root/torso_lower2/leg_left_hip2/leg_left_shin2/leg_left_shin",
];
const PARTS_BOOTS: &'static [&str] = &[
    "Node2/root/torso_lower2/leg_right_hip2/leg_right_shin2/leg_right_foot2/leg_right_foot",
    "Node2/root/torso_lower2/leg_left_hip2/leg_left_shin2/leg_left_foot2/leg_left_foot",
];

fn get_parts(part: ArmorParts) -> &'static [&'static str] {
    match part {
        ArmorParts::Chest => PARTS_CHEST,
        ArmorParts::Hands => PARTS_HANDS,
        ArmorParts::Pants => PARTS_PANTS,
        ArmorParts::Boots => PARTS_BOOTS,
    }
}
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
        let generic = load::<PackedScene>(GENERIC_MODEL).instantiate_as::<Node3D>();

        let mut animation_player = generic.get_node_as::<AnimationPlayer>("AnimationPlayer");

        let mut animation = animation_player.get_animation(StringName::from("animation_model_walk")).unwrap();
        animation.set_loop_mode(LoopMode::LOOP_LINEAR);

        animation_player.call_deferred(StringName::from("play"), &["animation_model_walk".to_variant()]);

        Self {
            base,
            generic,
        }
    }

    fn replace(&mut self, source: &Node3D, part: ArmorParts) {
        let parts = get_parts(part);
        for path in parts.iter() {
            let mut mesh = self.generic.get_node_as::<MeshInstance3D>(path);
            let part = source.get_node_as::<MeshInstance3D>(path);
            mesh.set_mesh(part.get_mesh().unwrap());
        }
    }
}


#[godot_api]
impl NodeVirtual for BodyController {
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base)
    }

    fn ready(&mut self) {
        self.base.add_child(self.generic.clone().upcast());

        let mut gltf = GltfDocument::new();

        let mut b: Vec<u8> = Vec::new();
        let path = "/home/honnisha/godot/honny-craft/honny-godot/assets/models/generic/replace.glb";
        let mut file = File::open(path).unwrap();
        let _bytes_read = file.read_to_end(&mut b);

        let mut pba = PackedByteArray::new();
        pba.extend(b);

        let gltf_state = GltfState::new();
        gltf.append_from_buffer(pba, GodotString::from("base_path?"), gltf_state.clone());
        let scene = gltf.generate_scene(gltf_state).unwrap();
        let scene = scene.cast::<Node3D>();

        // let scene = load::<PackedScene>("res://assets/models/generic/replace.glb").instantiate_as::<Node3D>();

        self.replace(&scene, ArmorParts::Chest);
    }

    fn process(&mut self, _delta: f64) {}
}
