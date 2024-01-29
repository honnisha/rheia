use super::physics::{PhysicsController, PhysicsContainer};
use physx::prelude::*;

type PxMaterial = physx::material::PxMaterial<()>;
type PxShape = physx::shape::PxShape<(), PxMaterial>;

pub struct PhysxPhysicsController {
    physics: String,
}
impl PhysicsController for PhysxPhysicsController {
    fn create() -> Self {
        Self {
            physics: PhysicsFoundation::<_, PxShape>::default(),
        }
    }

    fn step(&mut self, delta: f32, physics_container: &dyn PhysicsContainer) {}
}
