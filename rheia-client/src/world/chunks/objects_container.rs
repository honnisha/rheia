use common::chunks::position::Vector3 as NetworkVector3;
use crate::{
    client_scripts::resource_manager::ResourceStorage,
    utils::bridge::IntoGodotVector,
    world::{
        block_storage::BlockStorage,
        physics::{PhysicsProxy, PhysicsType},
    },
};
use ahash::AHashMap;
use common::{
    blocks::block_type::{BlockContent, BlockType},
    chunks::{
        block_position::{BlockPosition, BlockPositionTrait, ChunkBlockPosition},
        chunk_position::ChunkPosition,
    },
};
use godot::{classes::{BoxMesh, Mesh, MeshInstance3D}, prelude::*};
use network::messages::ChunkDataType;
use physics::{
    physics::{IPhysicsCollider, IPhysicsColliderBuilder},
    PhysicsCollider, PhysicsColliderBuilder,
};

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct CustomObject {
    base: Base<Node3D>,
    collider: PhysicsCollider,
}

impl CustomObject {
    pub fn create(base: Base<Node3D>, position: &BlockPosition, physics: &PhysicsProxy) -> Self {
        let physics_type = PhysicsType::ChunkMeshCollider(position.get_chunk_position());
        let collider_builder = PhysicsColliderBuilder::cuboid(1.0, 1.0, 1.0);
        let mut collider = physics.create_collider(collider_builder, Some(physics_type));
        collider.set_position(position.get_position() + NetworkVector3::new(0.5, 0.5, 0.5));

        Self { base, collider }
    }

    pub fn attach_glb(&mut self, glb: &Gd<Node3D>) {
        let mut glb = glb.duplicate().unwrap().cast::<Node3D>();
        glb.set_position(Vector3::new(1.0, 1.0, 1.0));
        self.base_mut().add_child(&glb);
    }

    pub fn remove(&mut self) {
        self.collider.remove();
    }
}

/// Container for custom objects of map per chunk section
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct ObjectsContainer {
    base: Base<Node3D>,
    blocks: AHashMap<ChunkBlockPosition, Gd<CustomObject>>,
}

impl ObjectsContainer {
    pub fn setup(
        &mut self,
        y: u32,
        chunk_position: &ChunkPosition,
        chunk_data: &ChunkDataType,
        block_storage: &BlockStorage,
        physics: &PhysicsProxy,
        resource_storage: &ResourceStorage,
    ) -> Result<(), String> {
        for (chunk_block_position, block_info) in chunk_data.iter() {
            let Some(block_type) = block_storage.get(&block_info.get_id()) else {
                continue;
            };

            let position = BlockPosition::from_chunk_position(chunk_position, &y, chunk_block_position);
            match block_type.get_block_content() {
                BlockContent::ModelCube { model: _ } => {
                    self.create_block_model(&position, &block_type, physics, resource_storage)?;
                }
                _ => continue,
            }
        }
        Ok(())
    }

    pub fn remove(&mut self, chunk_block_position: &ChunkBlockPosition) -> Option<()> {
        if let Some(mut object) = self.blocks.remove(chunk_block_position) {
            object.bind_mut().remove();
            object.queue_free();
            return Some(());
        }
        return None;
    }

    pub fn create_block_model(
        &mut self,
        position: &BlockPosition,
        block_type: &BlockType,
        physics: &PhysicsProxy,
        resource_storage: &ResourceStorage,
    ) -> Result<(), String> {
        let Some(model) = block_type.get_model() else {
            return Err("update_block_model called for non model".to_string());
        };
        let Some(media) = resource_storage.get_media(model) else {
            return Err(format!("model:{} is not found", model));
        };
        let Some(glb) = media.get_glb() else {
            return Err(format!("model:{} is not glb", model));
        };
        let mut object = Gd::<CustomObject>::from_init_fn(|base| CustomObject::create(base, &position, physics));
        object.bind_mut().attach_glb(glb);
        let (_section, block_position) = position.get_block_position();
        object.set_position(block_position.to_godot());

        self.base_mut().add_child(&object);
        self.blocks.insert(block_position, object);
        Ok(())
    }
}
