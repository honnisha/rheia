use super::physics::{
    PhysicsCharacterController, PhysicsColliderBuilder, PhysicsContainer, PhysicsRigidBodyEntity, PhysicsStaticEntity,
};
use crate::network::messages::Vector3;
use parking_lot::RwLock;
use physx::{foundation::DefaultAllocator, prelude::*, owner::Owner};
use std::{borrow::BorrowMut, sync::Arc};

type PxMaterial = physx::material::PxMaterial<()>;
type PxShape = physx::shape::PxShape<(), PxMaterial>;
type PxArticulationLink = physx::articulation_link::PxArticulationLink<(), PxShape>;
type PxRigidStatic = physx::rigid_static::PxRigidStatic<(), PxShape>;
type PxRigidDynamic = physx::rigid_dynamic::PxRigidDynamic<(), PxShape>;
type PxArticulationReducedCoordinate =
    physx::articulation_reduced_coordinate::PxArticulationReducedCoordinate<(), PxArticulationLink>;
type PxScene = physx::scene::PxScene<
    *const std::ffi::c_void,
    PxArticulationLink,
    PxRigidStatic,
    PxRigidDynamic,
    PxArticulationReducedCoordinate,
    OnCollision,
    OnTrigger,
    OnConstraintBreak,
    OnWakeSleep,
    OnAdvance,
>;

/// Next up, the simulation event callbacks need to be defined, and possibly an
/// allocator callback as well.
struct OnCollision;
impl CollisionCallback for OnCollision {
    fn on_collision(&mut self, _header: &physx_sys::PxContactPairHeader, _pairs: &[physx_sys::PxContactPair]) {}
}
struct OnTrigger;
impl TriggerCallback for OnTrigger {
    fn on_trigger(&mut self, _pairs: &[physx_sys::PxTriggerPair]) {}
}

struct OnConstraintBreak;
impl ConstraintBreakCallback for OnConstraintBreak {
    fn on_constraint_break(&mut self, _constraints: &[physx_sys::PxConstraintInfo]) {}
}
struct OnWakeSleep;
impl WakeSleepCallback<PxArticulationLink, PxRigidStatic, PxRigidDynamic> for OnWakeSleep {
    fn on_wake_sleep(
        &mut self,
        _actors: &[&physx::actor::ActorMap<PxArticulationLink, PxRigidStatic, PxRigidDynamic>],
        _is_waking: bool,
    ) {
    }
}

struct OnAdvance;
impl AdvanceCallback<PxArticulationLink, PxRigidDynamic> for OnAdvance {
    fn on_advance(
        &self,
        _actors: &[&physx::rigid_body::RigidBodyMap<PxArticulationLink, PxRigidDynamic>],
        _transforms: &[PxTransform],
    ) {
    }
}

pub struct PhysxPhysicsRigidBodyEntity {
    handler: Owner<PxRigidDynamic>,
}

impl PhysxPhysicsRigidBodyEntity {
    fn create(handler: Owner<PxRigidDynamic>) -> Self {
        Self {
            handler
        }
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

impl PhysicsStaticEntity for PhysxPhysicsStaticEntity {
    fn remove_collider(&mut self) {
        todo!()
    }
}

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

    fn update_collider(&mut self, static_entity: &mut PhysxPhysicsStaticEntity, position: &Vector3) {
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
    controller: Arc<RwLock<RapierPhysicsController>>,
}

impl PhysicsContainer<PhysxPhysicsRigidBodyEntity, PhysxPhysicsStaticEntity> for PhysxPhysicsContainer {
    fn create() -> Self {
        Self {
            controller: Arc::new(RwLock::new(RapierPhysicsController::create())),
        }
    }

    fn step(&self, delta: f32) {
        self.controller.as_ref().write().step(delta, self);
    }

    fn create_rigid_body(&self, height: f32, radius: f32, mass: f32) -> PhysxPhysicsRigidBodyEntity {
        let mut physics = self.controller.as_ref().write().physics;
        let geometry = PxCapsuleGeometry::new(radius, height / 2.0);
        let mut material = physics.create_material(0.5, 0.5, 0.6, ()).unwrap();

        let body = physics
            .create_rigid_dynamic(
                PxTransform::from_translation(&PxVec3::new(0.0, 40.0, 100.0)),
                &geometry,
                material.as_mut(),
                10.0,
                PxTransform::default(),
                (),
            )
            .unwrap();
        PhysxPhysicsRigidBodyEntity::create(body)
    }

    fn create_static(&self) -> PhysxPhysicsStaticEntity {
        todo!()
    }
}

pub struct RapierPhysicsController {
    physics: PhysicsFoundation<DefaultAllocator, PxShape>,
    scene: Owner<PxScene>,
}

impl RapierPhysicsController {
    fn create() -> Self {
        let mut physics = PhysicsFoundation::<_, PxShape>::default();
        let mut scene: Owner<PxScene> = physics
            .create(SceneDescriptor {
                gravity: PxVec3::new(0.0, -9.81, 0.0),
                on_advance: Some(OnAdvance),
                ..SceneDescriptor::new(std::ptr::null())
            })
            .unwrap();
        Self { physics, scene }
    }

    fn step(&mut self, delta: f32, physics_container: &PhysxPhysicsContainer) {
        self.scene
            .step(delta, None::<&mut physx_sys::PxBaseTask>, None, true)
            .expect("error occured during simulation");
    }
}
