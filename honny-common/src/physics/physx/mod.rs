use crate::network::messages::Vector3;

use super::physics::{
    PhysicsCharacterController, PhysicsColliderBuilder, PhysicsContainer, PhysicsController, PhysicsRigidBodyEntity,
    PhysicsStaticEntity,
};
use physx::prelude::*;

type PxMaterial = physx::material::PxMaterial<()>;
type PxShape = physx::shape::PxShape<(), PxMaterial>;

pub struct PhysxPhysicsRigidBodyEntity {}

impl PhysxPhysicsRigidBodyEntity {
    fn create() -> Self {
        todo!()
    }
}

impl PhysicsRigidBodyEntity for PhysxPhysicsRigidBodyEntity {
    fn set_enabled(&mut self, active: bool) {
        todo!()
    }

    fn apply_impulse(&mut self, impulse: Vector3) {
        todo!()
    }

    fn get_position(&self) -> Vector3 {
        todo!()
    }

    fn set_position(&mut self, position: Vector3) {
        todo!()
    }

    fn raycast(&self, dir: Vector3, max_toi: f32, origin: Vector3) -> Option<(usize, Vector3)> {
        todo!()
    }
}

pub struct PhysxPhysicsCharacterController {}
impl PhysicsCharacterController<PhysxPhysicsRigidBodyEntity> for PhysxPhysicsCharacterController {
    fn create() -> Self {
        todo!()
    }

    fn controller_move(&mut self, entity: &mut PhysxPhysicsRigidBodyEntity, delta: f64, impulse: Vector3) {
        todo!()
    }
}

pub struct PhysxPhysicsStaticEntity {}

impl PhysxPhysicsStaticEntity {
    fn create() -> Self {
        todo!()
    }
}

impl PhysicsStaticEntity for PhysxPhysicsStaticEntity {}

pub struct PhysxPhysicsColliderBuilder {}
impl PhysicsColliderBuilder<PhysxPhysicsStaticEntity> for PhysxPhysicsColliderBuilder {
    fn create() -> Self {
        todo!()
    }

    fn push_indexes(&mut self, index: [u32; 3]) {
        todo!()
    }

    fn push_verts(&mut self, x: f32, y: f32, z: f32) {
        todo!()
    }

    fn update_collider(&mut self, static_entity: &PhysxPhysicsStaticEntity, position: &Vector3) {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn compile(&mut self) {
        todo!()
    }
}

#[derive(Clone)]
pub struct PhysxPhysicsContainer {
    physics: String,
}

impl PhysicsContainer<PhysxPhysicsRigidBodyEntity, PhysxPhysicsStaticEntity> for PhysxPhysicsContainer {
    fn create() -> Self {
        todo!()
    }

    fn step(&self, delta: f32) {
        todo!()
    }

    fn create_rigid_body(&self, height: f32, radius: f32, mass: f32) -> PhysxPhysicsRigidBodyEntity {
        todo!()
    }

    fn create_static(&self) -> PhysxPhysicsStaticEntity {
        todo!()
    }
}

pub struct PhysxPhysicsController {
    physics: String,
}

impl PhysicsController<PhysxPhysicsContainer, PhysxPhysicsRigidBodyEntity, PhysxPhysicsStaticEntity> for PhysxPhysicsController {
    fn create() -> Self {
        todo!()
    }

    fn step(&mut self, delta: f32, physics_container: &PhysxPhysicsContainer) {
        todo!()
    }
}
