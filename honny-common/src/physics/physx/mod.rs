use super::physics::{
    PhysicsCharacterController, PhysicsColliderBuilder, PhysicsContainer, PhysicsRigidBodyEntity, PhysicsStaticEntity,
};
use crate::network::messages::Vector3 as NetworkVector3;
use parking_lot::RwLock;
use physx::{
    cooking::{create_triangle_mesh, PxCookingParams, TriangleMeshCookingResult},
    foundation::DefaultAllocator,
    owner::Owner,
    prelude::*,
    traits::Class,
};
use physx_sys::{
    PxMeshGeometryFlags as MeshGeometryFlags, PxMeshScale_new_2, PxRigidActor, PxScene_addActor_mut,
    PxTriangleMeshGeometry_new,
};
use std::ffi::c_void;
use std::ptr::null;
use std::sync::Arc;

// https://github.com/EmbarkStudios/physx-rs

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

fn vec_px_from_network(from: &NetworkVector3) -> PxVec3 {
    PxVec3::new(from.x, from.y, from.z)
}

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
    actor: Owner<PxRigidDynamic>,
}

impl PhysxPhysicsRigidBodyEntity {
    fn create(actor: Owner<PxRigidDynamic>) -> Self {
        Self { actor }
    }
}

impl PhysicsRigidBodyEntity for PhysxPhysicsRigidBodyEntity {
    fn set_enabled(&mut self, active: bool) {
        todo!()
    }

    fn apply_impulse(&mut self, impulse: NetworkVector3) {
        todo!()
    }

    fn get_position(&self) -> NetworkVector3 {
        todo!()
    }

    fn set_position(&mut self, position: NetworkVector3) {
        todo!()
    }

    fn raycast(&self, dir: NetworkVector3, max_toi: f32, origin: NetworkVector3) -> Option<(usize, NetworkVector3)> {
        todo!()
    }
}

pub struct PhysxPhysicsCharacterController {}
impl PhysicsCharacterController<PhysxPhysicsRigidBodyEntity> for PhysxPhysicsCharacterController {
    fn create() -> Self {
        todo!()
    }

    fn controller_move(&mut self, entity: &mut PhysxPhysicsRigidBodyEntity, delta: f64, impulse: NetworkVector3) {
        todo!()
    }
}

pub struct PhysxPhysicsStaticEntity {
    pub actor: Owner<PxRigidStatic>,
}

impl PhysxPhysicsStaticEntity {
    fn create(actor: Owner<PxRigidStatic>) -> Self {
        Self { actor }
    }
}

impl PhysicsStaticEntity for PhysxPhysicsStaticEntity {
    fn remove_collider(&mut self) {
        todo!()
    }
}

pub struct PhysxPhysicsColliderBuilder {
    controller: Arc<RwLock<RapierPhysicsController>>,
    collider_verts: Vec<PxVec3>,
    collider_indices: Vec<[u32; 3]>,
    geometry: Option<PxTriangleMeshGeometry>,
}

impl PhysxPhysicsColliderBuilder {
    fn generate_mesh(
        &self,
        physics: &mut PhysicsFoundation<DefaultAllocator, PxShape>,
    ) -> Result<PxTriangleMeshGeometry, String> {
        let mut desc = physx::cooking::PxTriangleMeshDesc::new();
        desc.obj.points.count = self.collider_verts.len() as u32;
        desc.obj.points.stride = std::mem::size_of::<PxVec3>() as u32;
        desc.obj.points.data = self.collider_verts.as_ptr() as *const c_void;

        desc.obj.triangles.count = self.collider_indices.len() as u32;
        desc.obj.triangles.stride = std::mem::size_of::<[u32; 3]>() as u32;
        desc.obj.triangles.data = self.collider_indices.as_ptr() as *const c_void;

        let scale = PxVec3::new(1.0, 1.0, 1.0);

        let params = PxCookingParams::new(physics.physics()).unwrap();
        match create_triangle_mesh(physics.physics_mut(), &params, &desc) {
            TriangleMeshCookingResult::Success(mut mesh) => {
                let sys_vec: physx_sys::PxVec3 = scale.into();
                let mesh_scale = unsafe { PxMeshScale_new_2(&sys_vec as *const physx_sys::PxVec3) };
                let result_mesh = unsafe {
                    PxTriangleMeshGeometry_new(
                        mesh.as_mut().as_mut_ptr(),
                        mesh_scale.as_ptr(),
                        MeshGeometryFlags::empty(),
                    )
                };
                Ok(result_mesh)
            }
            TriangleMeshCookingResult::LargeTriangle => Err("Error with generate collider: LargeTriangle".to_string()),
            TriangleMeshCookingResult::Failure => Err("Error with generate collider: Failure".to_string()),
            TriangleMeshCookingResult::InvalidDescriptor => {
                Err("Error with generate collider: InvalidDescriptor".to_string())
            }
        }
    }
}

impl PhysicsColliderBuilder<PhysxPhysicsStaticEntity> for PhysxPhysicsColliderBuilder {
    fn create() -> Self {
        Self {
            controller: Arc::new(RwLock::new(RapierPhysicsController::create())),
            collider_verts: Default::default(),
            collider_indices: Default::default(),
            geometry: Default::default(),
        }
    }

    fn push_indexes(&mut self, index: [u32; 3]) {
        self.collider_indices.push(index);
    }

    fn push_verts(&mut self, x: f32, y: f32, z: f32) {
        self.collider_verts.push(PxVec3::new(x, y, z));
    }

    fn update_collider(&mut self, static_entity: &mut PhysxPhysicsStaticEntity, position: &NetworkVector3) {
        static_entity.actor.attach_shape(shape);
        // https://github.com/rlidwka/bevy_mod_physx/blob/ef9e56023fb7500c7e5d1f2b66057a16a3caf8d7/src/core/systems/create_actors.rs#L33
    }

    fn len(&self) -> usize {
        self.collider_indices.len()
    }

    fn compile(&mut self) {
        let mut controller = self.controller.as_ref().write();

        let geometry = self.generate_mesh(&mut controller.physics).unwrap();
        self.geometry = Some(geometry);
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
        let mut controller = self.controller.as_ref().write();
        let geometry = PxCapsuleGeometry::new(radius, height / 2.0);
        let mut material = controller.physics.create_material(0.5, 0.5, 0.6, ()).unwrap();

        let mut actor = controller
            .physics
            .create_rigid_dynamic(
                PxTransform::from_translation(&PxVec3::new(0.0, 40.0, 100.0)),
                &geometry,
                material.as_mut(),
                10.0,
                PxTransform::default(),
                (),
            )
            .unwrap();

        unsafe {
            PxScene_addActor_mut(controller.scene.as_mut_ptr(), actor.as_mut_ptr(), null());
        }
        PhysxPhysicsRigidBodyEntity::create(actor)
    }

    fn create_static(&self) -> PhysxPhysicsStaticEntity {
        let mut controller = self.controller.as_ref().write();

        let mut actor: Owner<PxRigidStatic> = controller
            .physics
            .create_static(PxTransform::from_translation(&PxVec3::new(0.0, 0.0, 0.0)), ())
            .unwrap();
        unsafe {
            PxScene_addActor_mut(controller.scene.as_mut_ptr(), actor.as_mut_ptr(), null());
        }
        PhysxPhysicsStaticEntity::create(actor)
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
