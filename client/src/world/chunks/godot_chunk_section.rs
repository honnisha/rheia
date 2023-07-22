use std::borrow::BorrowMut;

use common::blocks::{block_info::BlockInfo, blocks_storage::BlockType};
use godot::{prelude::*, engine::{MeshInstance3D, Material, ArrayMesh}};
use ndshape::{ConstShape3u32, ConstShape};

pub type ChunkShape = ConstShape3u32<16, 16, 16>;
pub type ChunkBordersShape = ConstShape3u32<18, 18, 18>;

pub type ChunkData = [BlockInfo; ChunkShape::SIZE as usize];
pub type ChunkDataBordered = [BlockType; ChunkBordersShape::SIZE as usize];

/// Chunk section, one of the chunk column
/// Contains mesh and data of the chunk section blocks
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ChunkSection {
    #[base]
    pub base: Base<Node3D>,
    mesh: Option<Gd<MeshInstance3D>>,
}

impl ChunkSection {
    pub fn create(base: Base<Node3D>) -> Self {
        Self {
            base,
            mesh: None,
        }
    }

    pub fn create_mesh(&mut self, material: &Gd<Material>) {
        let mut mesh = MeshInstance3D::new_alloc();
        mesh.set_name(GodotString::from("ChunkMesh"));

        mesh.set_material_overlay(material.share());

        self.base.add_child(mesh.upcast());
        let m = self.base.get_node_as::<MeshInstance3D>("ChunkMesh");
        self.mesh = Some(m);
    }

    pub fn update_mesh(&mut self, new_mesh: Gd<ArrayMesh>) {
        let m = self.mesh.as_mut().unwrap().borrow_mut();
        let c = new_mesh.get_surface_count();
        m.set_mesh(new_mesh.upcast());

        if c > 0 {
            m.create_trimesh_collision();
        }
        //m.create_convex_collision(false, false);
    }
}

#[godot_api]
impl NodeVirtual for ChunkSection {
    /// For default godot init; only Self::create is using
    fn init(base: Base<Node3D>) -> Self {
        Self::create(base)
    }

    fn ready(&mut self) {
    }
}
