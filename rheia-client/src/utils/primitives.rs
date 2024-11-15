use godot::{
    builtin::{Color, Variant, Vector3},
    classes::{geometry_instance_3d::ShadowCastingSetting, mesh::PrimitiveType},
    classes::{base_material_3d::ShadingMode, BoxMesh, ImmediateMesh, Material, MeshInstance3D, OrmMaterial3D},
    obj::{Gd, NewAlloc, NewGd},
};

// https://github.com/Ryan-Mirch/Line-and-Sphere-Drawing/blob/main/Draw3D.gd

pub fn get_face_vector() -> Vec<Vector3> {
    vec![
        Vector3::new(-0.5, -0.5, -0.5),
        Vector3::new(0.5, -0.5, -0.5),
        Vector3::new(0.5, -0.5, -0.5),
        Vector3::new(0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, -0.5),
        Vector3::new(-0.5, 0.5, -0.5),
        Vector3::new(-0.5, 0.5, -0.5),
        Vector3::new(-0.5, -0.5, -0.5),
    ]
}

pub fn generate_lines(mut positions: Vec<Vector3>, color: Color) -> Gd<MeshInstance3D> {
    let mut mesh_instance = MeshInstance3D::new_alloc();

    let mut material = OrmMaterial3D::new_gd();
    material.set_shading_mode(ShadingMode::UNSHADED);
    material.set_albedo(color);

    let mut immediate_mesh = ImmediateMesh::new_gd();
    let m = Material::new_gd();
    immediate_mesh.call(
        "surface_begin",
        &[
            Variant::from(PrimitiveType::LINES),
            Variant::from(m.clone()),
        ],
    );

    for pos in positions.drain(..) {
        immediate_mesh.surface_add_vertex(pos);
    }
    immediate_mesh.surface_end();

    mesh_instance.set_mesh(&immediate_mesh);
    mesh_instance.set_cast_shadows_setting(ShadowCastingSetting::OFF);
    mesh_instance
}

pub fn _generate_box_mesh(size: Vector3, color: Color) -> Gd<MeshInstance3D> {
    let mut box_mesh = BoxMesh::new_gd();
    box_mesh.set_size(size);

    let mut material = OrmMaterial3D::new_gd();
    material.set_shading_mode(ShadingMode::UNSHADED);
    material.set_albedo(color);

    let mut mesh_instance = MeshInstance3D::new_alloc();
    mesh_instance.set_mesh(&box_mesh);
    mesh_instance.set_cast_shadows_setting(ShadowCastingSetting::OFF);
    mesh_instance
}
