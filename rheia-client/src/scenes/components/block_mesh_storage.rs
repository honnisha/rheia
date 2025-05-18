use ahash::HashMap;
use common::{
    blocks::{
        block_info::{BlockIndexType, BlockInfo},
        block_type::{BlockContent, BlockType},
    },
    chunks::block_position::BlockPosition,
};
use godot::{
    builtin::Vector3,
    classes::{Material, MeshInstance3D, PackedScene},
    obj::{Gd, NewAlloc},
};

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

use super::block_icon::BlockIcon;

enum BlockMesh {
    Texture(Gd<MeshInstance3D>),
    ModelCube(Gd<ObjectsContainer>),
}

pub struct BlockMeshStorage {
    block_icon_scene: Gd<PackedScene>,
    meshes: HashMap<BlockIndexType, (BlockMesh, f32)>,
}

impl BlockMeshStorage {
    fn generate_icon(
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
                mesh.set_position(Vector3::new(-1.5, -1.5, -1.5));

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
                objects_container.set_position(Vector3::new(-1.5, -1.5, -1.5));

                let icon_size = match icon_size {
                    Some(s) => s,
                    None => &1.0,
                };
                self.meshes
                    .insert(block_id, (BlockMesh::ModelCube(objects_container), 3.0 / icon_size));
            }
        }
    }

    pub fn init(
        block_icon_scene: &Gd<PackedScene>,
        block_storage: &BlockStorage,
        material: &Gd<Material>,
        resource_manager: &ResourceManager,
        texture_mapper: &TextureMapper,
    ) -> Self {
        let mut storage = Self {
            block_icon_scene: block_icon_scene.clone(),
            meshes: Default::default(),
        };

        for (block_id, block_type) in block_storage.iter() {
            storage.generate_icon(
                *block_id,
                block_type,
                material,
                texture_mapper,
                block_storage,
                &*resource_manager.get_resources_storage(),
            );
        }
        storage
    }

    pub fn get_icon(&self, block_id: &BlockIndexType) -> Option<Gd<BlockIcon>> {
        let Some((mesh, camera_size)) = self.meshes.get(block_id) else {
            return None;
        };
        let mut icon = self.block_icon_scene.instantiate_as::<BlockIcon>();
        icon.bind_mut().set_block_id(block_id.clone());

        icon.bind_mut().set_camera_size(camera_size.clone());
        match mesh {
            BlockMesh::Texture(gd) => icon.bind_mut().add_component(gd),
            BlockMesh::ModelCube(gd) => icon.bind_mut().add_component(gd),
        }

        Some(icon)
    }
}
