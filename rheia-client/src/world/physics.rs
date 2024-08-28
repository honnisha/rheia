use std::sync::Arc;

use ahash::AHashMap;
use common::{
    chunks::chunk_position::ChunkPosition,
    physics::{
        physics::{IPhysicsCollider, IPhysicsContainer},
        PhysicsCollider, PhysicsColliderBuilder, PhysicsContainer, PhysicsRigidBody, QueryFilter,
    },
};
use godot::prelude::*;
use parking_lot::RwLock;

use crate::utils::bridge::{IntoGodotVector, IntoNetworkVector};

#[derive(Clone)]
pub enum PhysicsType {
    ChunkMeshCollider(ChunkPosition),
    EntityCollider(u32),
}

#[derive(Default, Clone)]
pub struct PhysicsProxy {
    physics_container: PhysicsContainer,
    collider_type_map: Arc<RwLock<AHashMap<usize, PhysicsType>>>,
}

impl PhysicsProxy {
    pub fn step(&self, delta: f32) {
        self.physics_container.step(delta);
    }

    pub fn create_collider(
        &self,
        collider_builder: PhysicsColliderBuilder,
        collider_type: PhysicsType,
    ) -> PhysicsCollider {
        let collider = self.physics_container.spawn_collider(collider_builder);
        self.collider_type_map
            .write()
            .insert(collider.get_index(), collider_type);
        collider
    }

    pub fn create_rigid_body(&self) -> PhysicsRigidBody {
        self.physics_container.create_rigid_body()
    }

    pub fn spawn_collider_with_rigid(
        &self,
        collider_builder: PhysicsColliderBuilder,
        rigid_body: PhysicsRigidBody,
    ) -> PhysicsCollider {
        self.physics_container
            .spawn_collider_with_rigid(collider_builder, rigid_body)
    }

    pub fn get_type_by_collider(&self, collider_id: usize) -> Option<PhysicsType> {
        match self.collider_type_map.read().get(&collider_id) {
            Some(c) => Some(c.clone()),
            None => None,
        }
    }

    pub fn raycast(
        &self,
        dir: Vector3,
        max_toi: f32,
        from: Vector3,
        filter: QueryFilter,
    ) -> Option<(PhysicsType, Vector3)> {
        match self
            .physics_container
            .raycast(dir.to_network(), max_toi, from.to_network(), filter)
        {
            Some((collider_id, pos)) => {
                let map = self.collider_type_map.read();
                let Some(collider_type) = map.get(&collider_id) else {
                    panic!(
                        "collider_id:{collider_id} not found inside physics proxy; collider_type_map is not consistent"
                    )
                };
                Some((collider_type.clone(), pos.to_godot()))
            }
            None => None,
        }
    }
}
