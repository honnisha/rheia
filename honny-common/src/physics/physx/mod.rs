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
    PxHitFlags, PxMeshGeometryFlags as MeshGeometryFlags, PxMeshScale_new_2, PxPhysics_createShape_mut,
    PxQueryFilterData, PxQueryFilterData_new, PxSceneQueryExt_raycastSingle, PxScene_addActor_mut,
    PxTriangleMeshGeometry_new,
};
use std::ptr::null;
use std::sync::Arc;
use std::{ffi::c_void, mem::MaybeUninit, ptr::null_mut};

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

fn vec_sys_px_from_network(from: &NetworkVector3) -> physx_sys::PxVec3 {
    physx_sys::PxVec3 {
        x: from.x,
        y: from.y,
        z: from.z,
    }
}

fn vec_network_from_px(from: &PxVec3) -> NetworkVector3 {
    NetworkVector3::new(from.x(), from.y(), from.z())
}

fn vec_network_from_sys_px(from: &physx_sys::PxVec3) -> NetworkVector3 {
    NetworkVector3::new(from.x, from.y, from.z)
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
    controller: Arc<RwLock<PhysxPhysicsController>>,
}

impl PhysxPhysicsRigidBodyEntity {
    fn create(actor: Owner<PxRigidDynamic>, controller: Arc<RwLock<PhysxPhysicsController>>) -> Self {
        Self { actor, controller }
    }
}

impl PhysicsRigidBodyEntity for PhysxPhysicsRigidBodyEntity {
    fn set_enabled(&mut self, active: bool) {
        self.actor.enable_gravity(active)
    }

    fn apply_impulse(&mut self, impulse: NetworkVector3) {
        self.actor
            .as_mut()
            .add_force(&vec_px_from_network(&impulse), ForceMode::Impulse, true);
    }

    fn get_position(&self) -> NetworkVector3 {
        vec_network_from_px(&self.actor.get_global_position())
    }

    fn set_position(&mut self, position: NetworkVector3) {
        let pose = self.actor.get_global_pose();
        self.actor.set_global_pose(&pose, true)
    }

    fn raycast(&self, dir: NetworkVector3, max_toi: f32, origin: NetworkVector3) -> Option<(usize, NetworkVector3)> {
        let controller = self.controller.as_ref().read();

        let mut raycast_hit = MaybeUninit::uninit();

        let filter = unsafe { PxQueryFilterData_new() };
        if !unsafe {
            PxSceneQueryExt_raycastSingle(
                controller.scene.as_ptr(),
                &vec_sys_px_from_network(&origin),
                &vec_sys_px_from_network(&dir),
                max_toi,
                PxHitFlags::Default,
                raycast_hit.as_mut_ptr(),
                &filter as *const _,
                null_mut(),
                null_mut(),
            )
        } {
            return None;
        }

        // SAFETY: raycastSingle returned true, so we assume buffer is initialized
        let raycast_hit = unsafe { raycast_hit.assume_init() };
        Some((0_usize, vec_network_from_sys_px(&raycast_hit.position)))

        //Some(RaycastHit {
        //    actor: unsafe { get_actor_entity_from_ptr(raycast_hit.actor) },
        //    shape: unsafe { get_shape_entity_from_ptr(raycast_hit.shape) },
        //    face_index: raycast_hit.faceIndex,
        //    flags: raycast_hit.flags,
        //    position: raycast_hit.position.to_bevy(),
        //    normal: raycast_hit.normal.to_bevy(),
        //    distance: raycast_hit.distance,
        //})
    }
}

pub struct PhysxPhysicsCharacterController {}
impl PhysicsCharacterController<PhysxPhysicsRigidBodyEntity> for PhysxPhysicsCharacterController {
    fn create() -> Self {
        Self {}
    }

    fn controller_move(&mut self, entity: &mut PhysxPhysicsRigidBodyEntity, delta: f64, impulse: NetworkVector3) {
        todo!()
        // https://github.com/rlidwka/bevy_mod_physx/blob/ef9e56023fb7500c7e5d1f2b66057a16a3caf8d7/examples/kinematic.rs
    }
}

pub struct PhysxPhysicsStaticEntity {
    pub actor: Owner<PxRigidStatic>,
    pub shape: Option<Owner<PxShape>>,
}

impl PhysxPhysicsStaticEntity {
    fn create(actor: Owner<PxRigidStatic>) -> Self {
        Self {
            actor,
            shape: Default::default(),
        }
    }
}

impl PhysicsStaticEntity for PhysxPhysicsStaticEntity {
    fn remove_collider(&mut self) {
        self.actor.detach_shape(&mut self.shape.take().unwrap());
    }
}

pub struct PhysxPhysicsColliderBuilder {
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
        // let controller = self.controller.as_ref().read();
        let mut material = controller.physics.create_material(0.5, 0.5, 0.6, ()).unwrap();

        //let shape = physics.create_shape(geometry, materials, is_exclusive, shape_flags, user_data)
        let mut shape: Owner<PxShape> = unsafe {
            physx::shape::Shape::from_raw(
                PxPhysics_createShape_mut(
                    controller.physics.as_mut_ptr(),
                    self.geometry.unwrap().as_ptr(),
                    material.as_ptr(),
                    true,
                    ShapeFlags::empty(),
                ),
                None,
            )
            .unwrap()
        };
        static_entity.actor.attach_shape(&mut shape);
        static_entity.shape = Some(shape);
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
    controller: Arc<RwLock<PhysxPhysicsController>>,
}

impl PhysicsContainer<PhysxPhysicsRigidBodyEntity, PhysxPhysicsStaticEntity> for PhysxPhysicsContainer {
    fn create() -> Self {
        Self {
            controller: Arc::new(RwLock::new(PhysxPhysicsController::create())),
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
        PhysxPhysicsStaticEntity::create(actor, self.controller.clone())
    }
}

pub struct PhysxPhysicsController {
    physics: PhysicsFoundation<DefaultAllocator, PxShape>,
    scene: Owner<PxScene>,
}

impl PhysxPhysicsController {
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
