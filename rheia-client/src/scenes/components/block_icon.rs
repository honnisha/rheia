use common::{
    blocks::{
        block_info::{BlockIndexType, BlockInfo},
        block_type::{BlockContent, BlockType},
    },
    chunks::block_position::BlockPosition,
};
use godot::{
    classes::{Control, IControl, Material, MeshInstance3D},
    prelude::*,
};

use crate::{
    client_scripts::resource_manager::ResourceStorage,
    utils::textures::texture_mapper::TextureMapper,
    world::{
        block_storage::BlockStorage,
        chunks::{
            chunk_data_formatter::generate_single_block, mesh::mesh_generator::generate_chunk_geometry,
            objects_container::ObjectsContainer,
        },
    },
};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct BlockIcon {
    base: Base<Control>,

    #[export]
    pub block_anchor: Option<Gd<Node3D>>,

    #[export]
    camera: Option<Gd<Camera3D>>,
}

impl BlockIcon {
    pub fn setup_icons(
        &mut self,
        block_id: BlockIndexType,
        block_type: &BlockType,
        material: &Gd<Material>,
        texture_mapper: &TextureMapper,
        block_storage: &BlockStorage,
        resource_storage: &ResourceStorage,
    ) {
        let block_anchor = self.block_anchor.as_mut().unwrap();
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

                if let Some(camera) = self.camera.as_mut() {
                    camera.set_size(2.0);
                }
                block_anchor.add_child(&mesh);
            }
            BlockContent::ModelCube { model: _ } => {
                let mut objects_container = ObjectsContainer::new_alloc();
                let position = BlockPosition::new(0, 0, 0);
                objects_container
                    .bind_mut()
                    .create_block_model(&position, block_type, None, resource_storage)
                    .unwrap();
                objects_container.set_position(Vector3::new(-1.5, -1.5, -1.5));

                if let Some(camera) = self.camera.as_mut() {
                    camera.set_size(3.0);
                }
                block_anchor.add_child(&objects_container);
            }
        }
    }
}

#[godot_api]
impl IControl for BlockIcon {
    fn ready(&mut self) {}
}
