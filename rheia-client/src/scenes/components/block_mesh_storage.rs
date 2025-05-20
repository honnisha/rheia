use crate::{
    client_scripts::resource_manager::{ResourceManager, ResourceStorage},
    utils::textures::texture_mapper::TextureMapper,
    world::{
        block_storage::BlockStorage,
        chunks::{
            chunk_data_formatter::generate_single_block,
            mesh::mesh_generator::generate_chunk_geometry,
            objects_container::{CustomObject, ObjectsContainer},
        },
    },
};
use ahash::HashMap;
use common::{
    blocks::{
        block_info::{BlockIndexType, BlockInfo},
        block_type::{BlockContent, BlockType},
    },
    chunks::block_position::BlockPosition,
};
use godot::{
    classes::{Material, MeshInstance3D},
    prelude::*,
};

use super::block_icon::BlockIcon;

pub enum BlockMesh {
    Texture(Gd<MeshInstance3D>),
    ModelCube(Gd<CustomObject>),
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
            BlockContent::Texture {
                texture: _,
                side_texture: _,
                bottom_texture: _,
            } => {
                let block_info = BlockInfo::create(block_id, None);
                let bordered_chunk_data = generate_single_block(&block_type, &block_info);
                let geometry = generate_chunk_geometry(texture_mapper, &bordered_chunk_data, &block_storage);

                let mut mesh = MeshInstance3D::new_alloc();
                mesh.set_mesh(&geometry.mesh_ist);

                mesh.set_material_overlay(material);

                self.meshes.insert(block_id, (BlockMesh::Texture(mesh), 2.0));
            }
            BlockContent::ModelCube { model: _, icon_size } => {
                let mut objects_container = ObjectsContainer::new_alloc();
                let position = BlockPosition::new(0, 0, 0);
                objects_container
                    .bind_mut()
                    .create_block_model(&position, block_type, None, resource_storage)
                    .unwrap();

                let icon_size = match icon_size {
                    Some(s) => s,
                    None => &1.0,
                };
                let obj = objects_container.bind().get_first().clone();
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

        for (block_id, block_type) in block_storage.iter() {
            storage.generate_block_mesh(
                *block_id,
                block_type,
                material,
                texture_mapper,
                block_storage,
                &*resource_manager.get_resources_storage(),
            );
        }

        Gd::<Self>::from_init_fn(|_base| storage)
    }

    pub fn get_icon(&self, block_id: &BlockIndexType) -> Option<Gd<BlockIcon>> {
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

        match mesh {
            BlockMesh::Texture(gd) => {
                let mut obj = gd.duplicate().unwrap().cast::<Node3D>();
                obj.set_position(Vector3::new(-1.5, -1.5, -1.5));
                obj
            }
            BlockMesh::ModelCube(gd) => {
                let mut obj = gd.bind().get_content().duplicate().unwrap().cast::<Node3D>();
                obj.set_position(Vector3::new(-0.5, -0.5, -0.5));
                obj
            }
        }
    }
}
