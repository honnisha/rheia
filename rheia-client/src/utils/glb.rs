use godot::{
    engine::{GltfDocument, GltfState},
    prelude::*,
};

pub fn glb_import(b: Vec<u8>) -> Result<Gd<Node3D>, String> {
    let mut gltf = GltfDocument::new_gd();

    let mut pba = PackedByteArray::new();
    pba.extend(b);

    let gltf_state = GltfState::new_gd();
    gltf.append_from_buffer(pba, GString::from("base_path?"), gltf_state.clone());
    let scene = match gltf.generate_scene(gltf_state) {
        Some(s) => s,
        None => return Err("gltf generate_scene None".to_string()),
    };
    let scene = scene.cast::<Node3D>();
    Ok(scene)
}
