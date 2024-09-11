use physx::foundation::DefaultAllocator;
use physx::math::PxVec3;
use physx::owner::Owner;
use physx::prelude::{Physics, PhysicsFoundation, SceneDescriptor};
use physx::scene::Scene;

use super::container::PhysxPhysicsContainer;
use super::types::{OnAdvance, OnCollision, PxScene, PxShape};

pub struct PhysxPhysicsController {
    pub(crate) physics: PhysicsFoundation<DefaultAllocator, PxShape>,
    pub(crate) scene: Owner<PxScene>,
}

impl PhysxPhysicsController {
    pub(crate) fn create() -> Self {
        let mut physics = PhysicsFoundation::<_, PxShape>::default();
        let scene: Owner<PxScene> = physics
            .create(SceneDescriptor {
                gravity: PxVec3::new(0.0, -9.81, 0.0),
                on_advance: Some(OnAdvance),
                on_collide: Some(OnCollision),
                ..SceneDescriptor::new(())
            })
            .unwrap();
        Self { physics, scene }
    }

    pub(crate) fn step(&mut self, delta: f32, _physics_container: &PhysxPhysicsContainer) {
        self.scene
            .step(delta, None::<&mut physx_sys::PxBaseTask>, None, true)
            .expect("error occured during simulation");
    }
}
