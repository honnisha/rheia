use godot::{
    builtin::{Color, Vector3},
    classes::geometry_instance_3d::ShadowCastingSetting,
    engine::{
        base_material_3d::{ShadingMode, Transparency},
        BoxMesh, MeshInstance3D, OrmMaterial3D,
    },
    obj::{Gd, NewAlloc, NewGd},
};

// https://github.com/Ryan-Mirch/Line-and-Sphere-Drawing/blob/main/Draw3D.gd

pub fn generate_box_mesh() -> Gd<MeshInstance3D> {
    let mut box_mesh = BoxMesh::new_gd();
    box_mesh.set_size(Vector3::new(1.0, 1.0, 1.0));

    let mut material = OrmMaterial3D::new_gd();
    material.set_shading_mode(ShadingMode::UNSHADED);
    material.set_albedo(Color::from_rgba(0.0, 0.0, 0.0, 0.5));
    material.set_transparency(Transparency::ALPHA);

    let mut mesh_instance = MeshInstance3D::new_alloc();
    mesh_instance.set_mesh(box_mesh.upcast());
    mesh_instance.set_cast_shadows_setting(ShadowCastingSetting::OFF);
    mesh_instance
}
