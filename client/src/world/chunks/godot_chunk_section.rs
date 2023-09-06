use std::borrow::BorrowMut;

use common::blocks::{blocks_storage::BlockType};
use godot::{prelude::*, engine::{MeshInstance3D, Material, ArrayMesh}};
use ndshape::{ConstShape3u32, ConstShape};

use crate::world::world_manager::get_default_material;

//pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

//pub type ChunkData = [BlockInfo; ChunkShape::SIZE as usize];
pub type ChunkDataBordered = [BlockType; ChunkBordersShape::SIZE as usize];

/// Chunk section, one of the chunk column
/// Contains mesh and data of the chunk section blocks
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ChunkSection {
    #[base]
    pub(crate) base: Base<Node3D>,
    mesh: Option<Gd<MeshInstance3D>>,
    material: Gd<Material>,
    y: u8,
}

impl ChunkSection {
    pub fn create(base: Base<Node3D>, material: Gd<Material>, y: u8) -> Self {
        Self {
            base,
            mesh: None,
            material,
            y,
        }
    }

    pub fn create_mesh(&mut self) {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GodotString::from("ChunkMesh"));

        mesh.set_material_overlay(self.material.share());

        self.base.add_child(mesh.upcast());
        let m = self.base.get_node_as::<MeshInstance3D>("ChunkMesh");
        self.mesh = Some(m);
    }

    pub fn update_mesh(&mut self, new_mesh: Gd<ArrayMesh>) {
        let m = self.mesh.as_mut().unwrap().borrow_mut();
        //let c = new_mesh.get_surface_count();
        m.set_mesh(new_mesh.upcast());
        // println!("update_mesh y:{} surface_count:{}", self.y, c);

        //if c > 0 {
        //    m.create_trimesh_collision();
        //}
        //m.create_convex_collision(false, false);
    }
}

#[godot_api]
impl NodeVirtual for ChunkSection {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base, get_default_material(), 0)
    }

    fn ready(&mut self) {
        self.create_mesh();
    }
}
