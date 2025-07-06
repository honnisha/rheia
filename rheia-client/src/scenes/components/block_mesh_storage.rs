use crate::{
    client_scripts::resource_manager::{ResourceManager, ResourceStorage},
    utils::textures::texture_mapper::TextureMapper,
    world::{
        block_storage::BlockStorage,
        chunks::{
            chunk_data_formatter::generate_single_block, mesh::mesh_generator::generate_chunk_geometry,
            objects_container::ObjectsContainer,
        },
    },
};
use ahash::HashMap;
use common::{
    blocks::block_type::{BlockContent, BlockType},
    chunks::{
        block_position::BlockPosition,
        chunk_data::{BlockDataInfo, BlockIndexType},
    },
};
use godot::{
    classes::{Material, MeshInstance3D},
    prelude::*,
};

use super::block_icon::BlockIcon;

pub enum BlockMesh {
    Texture(Gd<Node3D>),
    ModelCube(Gd<Node3D>),
}

#[derive(Default, GodotClass)]
#[class(no_init)]
pub struct BlockMeshStorage {
    meshes: HashMap<BlockIndexType, (BlockMesh, f32)>,
}

impl BlockMeshStorage {
    fn generate_block_mesh(
        &mut self,
        block_id: BlockIndexType,
        block_type: &BlockType,
        material: &Gd<Material>,
        texture_mapper: &TextureMapper,
        block_storage: &BlockStorage,
        resource_storage: &ResourceStorage,
    ) {
        match block_type.get_block_content() {
            BlockContent::Texture { .. } => {
                let block_info = BlockDataInfo::create(block_id, None);
                let bordered_chunk_data = generate_single_block(&block_type, &block_info);
                let geometry = generate_chunk_geometry(texture_mapper, &bordered_chunk_data, &block_storage);

                let mut mesh = MeshInstance3D::new_alloc();
                mesh.set_name("Block mesh");

                mesh.set_mesh(&geometry.mesh_ist);
                mesh.set_material_overlay(material);

                let mut obj = Node3D::new_alloc();
                obj.set_name(&format!("BlockTexture #{}", block_info.get_id()));
                obj.add_child(&mesh);

                self.meshes.insert(block_id, (BlockMesh::Texture(obj), 2.0));
            }
            BlockContent::ModelCube {
                model,
                icon_size,
                collider_type,
            } => {
                let mut objects_container = ObjectsContainer::new_alloc();
                objects_container.set_name(&format!("ObjectsContainer #{}", block_id));
                let position = BlockPosition::new(0, 0, 0);
                objects_container
                    .bind_mut()
                    .create_block_model(&position, model, collider_type, None, resource_storage, None)
                    .unwrap();

                let icon_size = match icon_size {
                    Some(s) => s,
                    None => &1.0,
                };

                // Get first object of chunk
                let obj = objects_container.bind().get_first().clone();

                // Get content of object
                let obj = obj.bind().get_content().duplicate().unwrap().cast::<Node3D>();

                self.meshes
                    .insert(block_id, (BlockMesh::ModelCube(obj), 3.0 / icon_size));
            }
        }
    }

    pub fn init(
        block_storage: &BlockStorage,
        material: &Gd<Material>,
        resource_manager: &ResourceManager,
        texture_mapper: &TextureMapper,
    ) -> Gd<Self> {
        let mut storage: Self = Default::default();

        for (block_slug, block_type) in block_storage.iter() {
            let block_id = match block_storage.get_block_id(block_slug) {
                Some(i) => i,
                None => panic!("block_slug \"{}\" id not found", block_slug),
            };
            storage.generate_block_mesh(
                block_id,
                block_type,
                material,
                texture_mapper,
                block_storage,
                &*resource_manager.get_resources_storage(),
            );
        }

        Gd::<Self>::from_init_fn(|_base| storage)
    }

    pub fn generate_icon(&self, block_id: &BlockIndexType) -> Option<Gd<BlockIcon>> {
        let Some((_mesh, camera_size)) = self.meshes.get(block_id) else {
            return None;
        };
        let mut icon = BlockIcon::create();
        icon.bind_mut().set_block_id(block_id.clone());

        icon.bind_mut().set_camera_size(camera_size.clone());

        let obj = self.get_mesh(block_id);
        icon.bind_mut().add_component(&obj);

        Some(icon)
    }

    pub fn get_mesh(&self, block_id: &BlockIndexType) -> Gd<Node3D> {
        let Some((mesh, _camera_size)) = self.meshes.get(block_id) else {
            panic!("block_id {} not found", block_id);
        };

        let gd = match mesh {
            BlockMesh::Texture(gd) => {
                let mut gd = gd.duplicate().unwrap().cast::<Node3D>();
                // magic: locate it on center of block
                gd.set_position(Vector3::new(-0.5, -0.5, -0.5));
                gd
            }
            BlockMesh::ModelCube(gd) => {
                let gd = gd.duplicate().unwrap().cast::<Node3D>();
                gd
            }
        };

        let mut obj_holder = Node3D::new_alloc();
        obj_holder.add_child(&gd);
        obj_holder
    }
}
