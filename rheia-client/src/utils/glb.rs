use godot::{
    classes::{GltfDocument, GltfState},
    prelude::*,
};

pub fn glb_import(b: Vec<u8>) -> Result<Gd<Node3D>, String> {
    let mut gltf = GltfDocument::new_gd();
    gltf.set_local_to_scene(true);

    let mut pba = PackedByteArray::new();
    pba.extend(b);

    let gltf_state = GltfState::new_gd();
    gltf.append_from_buffer(&pba, &"base_path?".to_string(), &gltf_state);
    let scene = match gltf.generate_scene(&gltf_state) {
        Some(s) => s,
        None => return Err("gltf generate_scene None".to_string()),
    };
    let scene = scene.cast::<Node3D>();
    Ok(scene)
}
