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
    blocks::block_type::BlockContent,
    chunks::{
        block_position::{BlockPosition, BlockPositionTrait, ChunkBlockPosition},
        chunk_position::ChunkPosition,
    },
};
use common::{
    blocks::{block_info::BlockFace, block_type::ColliderType},
    chunks::{chunk_data::ChunkSectionData, position::Vector3 as NetworkVector3},
};
use godot::prelude::*;
use physics::{
    PhysicsCollider, PhysicsColliderBuilder,
    physics::{IPhysicsCollider, IPhysicsColliderBuilder},
};

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct CustomObject {
    base: Base<Node3D>,
    collider: Option<PhysicsCollider>,
}

impl CustomObject {
    pub fn create(base: Base<Node3D>) -> Self {
        Self { base, collider: None }
    }

    pub fn attach_glb(&mut self, glb: &Gd<Node3D>, block_face: Option<&BlockFace>) {
        let mut obj = glb.duplicate().unwrap().cast::<Node3D>();

        // magic: locate it on center of block
        obj.set_position(Vector3::new(-0.5, -0.5, -0.5));

        let mut obj_holder = Node3D::new_alloc();
        obj_holder.add_child(&obj);

        let block_face = match block_face {
            Some(f) => f.clone(),
            None => BlockFace::default(),
        };
        let rotation = block_face.get_rotation();
        let mut r = obj.get_rotation_degrees();
        r.x = rotation.yaw % 360.0;
        r.y = rotation.pitch % 360.0;
        obj_holder.set_rotation_degrees(r);

        self.base_mut().add_child(&obj_holder);
    }

    pub fn create_collider(&mut self, position: &BlockPosition, physics: &PhysicsProxy, is_sensor: bool) {
        let physics_type = PhysicsType::ChunkMeshCollider(position.get_chunk_position());
        let collider_builder = PhysicsColliderBuilder::cuboid(1.0, 1.0, 1.0);
        let mut collider = physics.create_collider(collider_builder, Some(physics_type));
        collider.set_position(position.get_position() + NetworkVector3::new(0.5, 0.5, 0.5));
        if is_sensor {
            collider.set_sensor(is_sensor);
        }

        self.collider = Some(collider);
    }

    pub fn remove(&mut self) {
        if let Some(collider) = self.collider.as_mut() {
            collider.remove();
        }
    }

    pub fn get_content(&self) -> Gd<Node3D> {
        let obj = self.base().get_children().iter_shared().next().unwrap();
        obj.cast::<Node3D>()
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
    // Specifically for icons
    pub fn get_first(&self) -> &Gd<CustomObject> {
        self.blocks.values().next().unwrap()
    }

    pub fn setup(
        &mut self,
        y: u32,
        chunk_position: &ChunkPosition,
        chunk_data: &ChunkSectionData,
        block_storage: &BlockStorage,
        physics: &PhysicsProxy,
        resource_storage: &ResourceStorage,
    ) -> Result<(), String> {
        for (block_index, block_info) in chunk_data.iter() {
            let chunk_block_position = ChunkBlockPosition::delinearize(*block_index);
            let Some(block_type) = block_storage.get(&block_info.get_id()) else {
                continue;
            };

            let position = BlockPosition::from_chunk_position(chunk_position, &y, &chunk_block_position);
            match block_type.get_block_content() {
                BlockContent::ModelCube {
                    model,
                    icon_size: _,
                    collider_type,
                } => {
                    self.create_block_model(
                        &position,
                        model,
                        collider_type,
                        Some(physics),
                        resource_storage,
                        block_info.get_face(),
                    )?;
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

    pub fn destory(&mut self) {
        for (_position, mut object) in self.blocks.drain() {
            object.bind_mut().remove();
            object.queue_free();
        }
    }

    pub fn create_block_model(
        &mut self,
        position: &BlockPosition,
        model: &String,
        collider_type: &ColliderType,
        physics: Option<&PhysicsProxy>,
        resource_storage: &ResourceStorage,
        block_face: Option<&BlockFace>,
    ) -> Result<(), String> {
        let Some(media) = resource_storage.get_media(model) else {
            return Err(format!("model:{} is not found", model));
        };
        let Some(glb) = media.get_glb() else {
            return Err(format!("model:{} is not glb", model));
        };
        let mut object = Gd::<CustomObject>::from_init_fn(|base| CustomObject::create(base));

        if let Some(physics) = physics {
            object
                .bind_mut()
                .create_collider(&position, physics, collider_type.is_sensor());
        }

        object.bind_mut().attach_glb(glb, block_face);
        let (_section, block_position) = position.get_block_position();

        // +0.5 to place it on center of the block
        object.set_position(block_position.to_godot() + Vector3::new(0.5, 0.5, 0.5));

        self.base_mut().add_child(&object);
        self.blocks.insert(block_position, object);
        Ok(())
    }
}
