use std::borrow::BorrowMut;

use godot::{
    engine::{node::InternalMode, ArrayMesh, Material, MeshInstance3D},
    prelude::*,
};

/// Godot object Chunk inherited from Node3D
/// contains godot data (mesh, etc)
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Chunk {
    #[base]
    pub base: Base<Node3D>,
    mesh: Option<Gd<MeshInstance3D>>,
    loaded: bool,
}

pub type ChunkPositionType = [i32; 3];

#[godot_api]
impl Chunk {
    pub fn create_mesh(&mut self, material: &Gd<Material>) {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GodotString::from("ChunkMesh"));

        mesh.set_material_overlay(material.share());

        self.base.add_child(mesh.upcast());
        let m = self.base.get_node_as::<MeshInstance3D>("ChunkMesh");
        self.mesh = Some(m);
    }
}

impl Chunk {
    pub fn create(base: Base<Node3D>) -> Self {
        Chunk {
            base: base,
            mesh: None,
            loaded: false,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn update_mesh(&mut self, new_mesh: Gd<ArrayMesh>) {
        let m = self.mesh.as_mut().unwrap().borrow_mut();
        let c = new_mesh.get_surface_count();
        m.set_mesh(new_mesh.upcast());

        if c > 0 {
            m.create_trimesh_collision();
        }
        //m.create_convex_collision(false, false);
        self.loaded = true;
    }
}

#[godot_api]
impl Node3DVirtual for Chunk {
    fn init(base: Base<Node3D>) -> Self {
        Chunk::create(base)
    }

    fn ready(&mut self) {}
}
