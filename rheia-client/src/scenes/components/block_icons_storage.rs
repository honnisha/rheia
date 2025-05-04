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

#[derive(Default)]
pub struct BlockIconsStorage {
    icons: HashMap<BlockIndexType, Gd<BlockIcon>>,
}

impl BlockIconsStorage {
    fn generate_icon(
        icon: &mut Gd<BlockIcon>,
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

                icon.bind_mut().set_camera_size(2.0);
                icon.bind_mut().add_component(&mesh);
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
                icon.bind_mut().set_camera_size(3.0 / icon_size);
                icon.bind_mut().add_component(&objects_container);
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
        let mut storage = Self::default();

        for (block_id, block_type) in block_storage.iter() {
            let mut icon = block_icon_scene.instantiate_as::<BlockIcon>();
            BlockIconsStorage::generate_icon(
                &mut icon,
                *block_id,
                block_type,
                material,
                texture_mapper,
                block_storage,
                &*resource_manager.get_resources_storage(),
            );
            storage.icons.insert(block_id.clone(), icon);
        }
        storage
    }
}
