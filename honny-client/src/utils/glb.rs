use godot::{
    engine::{GltfDocument, GltfState},
    prelude::*,
};

pub fn glb_import(b: Vec<u8>) -> Gd<Node3D> {
    let mut gltf = GltfDocument::new();

    let mut pba = PackedByteArray::new();
    pba.extend(b);

    let gltf_state = GltfState::new();
    gltf.append_from_buffer(pba, GodotString::from("base_path?"), gltf_state.clone());
    let scene = gltf.generate_scene(gltf_state).unwrap();
    let scene = scene.cast::<Node3D>();
    scene
}
