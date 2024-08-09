mod types;

use super::physics::{
    PhysicsCharacterController, PhysicsColliderBuilder, PhysicsContainer, PhysicsRigidBodyEntity, PhysicsStaticEntity,
};
use crate::network::messages::{IntoNetworkVector, Vector3 as NetworkVector3};
use parking_lot::RwLock;
use physx::{cooking, foundation::DefaultAllocator, owner::Owner, prelude::*, traits::Class};
use physx_sys::{
    PxConvexFlags, PxHitFlags, PxMeshScale_new, PxPhysics_createShape_mut, PxQueryFilterData_new,
    PxSceneQueryExt_raycastSingle, PxScene_addActor_mut, PxShape_setLocalPose_mut,
};
use std::sync::Arc;
use std::{ffi::c_void, mem::MaybeUninit, ptr::null_mut};
use types::*;

// https://github.com/EmbarkStudios/physx-rs

pub trait IntoPxVec3 {
    fn to_physx(&self) -> PxVec3;
    fn to_physx_sys(&self) -> physx_sys::PxVec3;
}

impl IntoPxVec3 for NetworkVector3 {
    fn to_physx(&self) -> PxVec3 {
        PxVec3::new(self.x, self.y, self.z)
    }

    fn to_physx_sys(&self) -> physx_sys::PxVec3 {
        physx_sys::PxVec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl IntoNetworkVector for PxVec3 {
    fn to_network(&self) -> NetworkVector3 {
        NetworkVector3::new(self.x(), self.y(), self.z())
    }
}

impl IntoNetworkVector for physx_sys::PxVec3 {
    fn to_network(&self) -> NetworkVector3 {
        NetworkVector3::new(self.x, self.y, self.z)
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
            .add_force(&impulse.to_physx(), ForceMode::Impulse, true);
    }

    fn get_position(&self) -> NetworkVector3 {
        self.actor.get_global_position().to_network()
    }

    fn set_position(&mut self, position: NetworkVector3) {
        self.actor
            .set_global_pose(&PxTransform::from_translation(&position.to_physx()), true);
    }

    fn raycast(&self, dir: NetworkVector3, max_toi: f32, origin: NetworkVector3) -> Option<(usize, NetworkVector3)> {
        let controller = self.controller.as_ref().read();

        let mut raycast_hit = MaybeUninit::uninit();

        let filter = unsafe { PxQueryFilterData_new() };
        if !unsafe {
            PxSceneQueryExt_raycastSingle(
                controller.scene.as_ptr(),
                &origin.to_physx_sys(),
                &dir.to_physx_sys(),
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
        Some((0_usize, raycast_hit.position.to_network()))

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

pub struct PhysxPhysicsCharacterController {
    custom_mass: Option<f32>
}

impl PhysicsCharacterController<PhysxPhysicsRigidBodyEntity> for PhysxPhysicsCharacterController {
    fn create(custom_mass: Option<f32>) -> Self {
        Self {
            custom_mass,
        }
    }

    fn move_shape(&mut self, _entity: &mut PhysxPhysicsRigidBodyEntity, _delta: f64, _impulse: NetworkVector3) {
        // https://github.com/rlidwka/bevy_mod_physx/blob/ef9e56023fb7500c7e5d1f2b66057a16a3caf8d7/examples/kinematic.rs
    }

    fn is_grounded(&mut self) -> bool {
        false
    }

    fn get_custom_mass(&mut self) -> &Option<f32> {
        &self.custom_mass
    }
}

pub struct PhysxPhysicsStaticEntity {
    pub actor: Owner<PxRigidStatic>,
    controller: Arc<RwLock<PhysxPhysicsController>>,

    // Attached shape
    pub shape: Option<Owner<PxShape>>,
}

impl PhysxPhysicsStaticEntity {
    fn create(actor: Owner<PxRigidStatic>, controller: Arc<RwLock<PhysxPhysicsController>>) -> Self {
        Self {
            actor,
            controller,
            shape: Default::default(),
        }
    }
}

impl PhysicsStaticEntity for PhysxPhysicsStaticEntity {
    fn remove_collider(&mut self) {
        if self.shape.is_some() {
            self.actor.detach_shape(&mut self.shape.take().unwrap());
        }
    }

    fn set_enabled(&mut self, active: bool) {
        todo!()
    }
}

pub struct PhysxPhysicsColliderBuilder {
    collider_verts: Vec<PxVec3>,
    collider_indices: Vec<[u32; 3]>,
}

impl PhysicsColliderBuilder<PhysxPhysicsStaticEntity> for PhysxPhysicsColliderBuilder {
    fn create() -> Self {
        Self {
            collider_verts: Default::default(),
            collider_indices: Default::default(),
        }
    }

    fn push_indexes(&mut self, index: [u32; 3]) {
        self.collider_indices.push(index);
    }

    fn push_verts(&mut self, x: f32, y: f32, z: f32) {
        self.collider_verts.push(PxVec3::new(x, y, z));
    }

    fn update_collider(&mut self, static_entity: &mut PhysxPhysicsStaticEntity, position: &NetworkVector3) {
        let mut controller = static_entity.controller.write();

        let mut desc = cooking::PxConvexMeshDesc::new();
        desc.obj.points.count = self.collider_verts.len() as u32;
        desc.obj.points.stride = std::mem::size_of::<PxVec3>() as u32;
        desc.obj.points.data = self.collider_verts.as_ptr() as *const c_void;
        desc.obj.flags = PxConvexFlags::ComputeConvex;

        let params = cooking::PxCookingParams::new(controller.physics.physics()).unwrap();
        let mesh = match cooking::create_convex_mesh(controller.physics.physics_mut(), &params, &desc) {
            cooking::ConvexMeshCookingResult::Success(mut mesh) => PxConvexMeshGeometry::new(
                &mut mesh,
                &unsafe { PxMeshScale_new() },
                ConvexMeshGeometryFlags::TightBounds,
            ),
            _ => panic!("create_convex_mesh error"),
        };

        let flags = ShapeFlags::SceneQueryShape | ShapeFlags::SimulationShape | ShapeFlags::Visualization;
        let mut material = controller.physics.create_material(0.5, 0.5, 0.6, ()).unwrap();

        //let mut shape = controller
        //    .physics
        //    .create_shape(&mesh, &mut [&mut material], true, flags, ())
        //    .unwrap();
        let mut shape: Owner<PxShape> = unsafe {
            physx::shape::Shape::from_raw(
                PxPhysics_createShape_mut(
                    controller.physics.as_mut_ptr(),
                    mesh.as_ptr(),
                    material.as_mut_ptr(),
                    true,
                    flags,
                ),
                (),
            )
            .unwrap()
        };

        unsafe {
            PxScene_addActor_mut(
                controller.scene.as_mut_ptr(),
                static_entity.actor.as_mut_ptr(),
                std::ptr::null(),
            );
        }

        //static_entity
        //    .actor
        //    .set_global_pose(&PxTransform::from_translation(&vec_px_from_network(&position)), true);

        static_entity.actor.attach_shape(&mut shape);

        unsafe {
            PxShape_setLocalPose_mut(
                shape.as_mut_ptr(),
                PxTransform::from_translation(&position.to_physx()).as_ptr(),
            );
        }
        static_entity.shape = Some(shape);
    }

    fn len(&self) -> usize {
        self.collider_indices.len()
    }

    fn compile(&mut self) {}
}

#[derive(Clone)]
pub struct PhysxPhysicsContainer {
    controller: Arc<RwLock<PhysxPhysicsController>>,
}

impl PhysicsContainer<PhysxPhysicsRigidBodyEntity, PhysxPhysicsStaticEntity> for PhysxPhysicsContainer {
    fn create() -> Self {
        let controller = Arc::new(RwLock::new(PhysxPhysicsController::create()));
        Self { controller }
    }

    fn step(&self, delta: f32) {
        self.controller.as_ref().write().step(delta, self);
    }

    fn create_rigid_body(&self, height: f32, radius: f32, mass: f32) -> PhysxPhysicsRigidBodyEntity {
        let mut controller = self.controller.as_ref().write();
        let geometry = PxCapsuleGeometry::new(radius, height / 2.0);
        let mut material = controller.physics.create_material(0.0, 0.0, 0.0, ()).unwrap();

        let mut actor = controller
            .physics
            .create_rigid_dynamic(
                PxTransform::from_translation(&PxVec3::new(0.0, 0.0, 0.0)),
                &geometry,
                material.as_mut(),
                10.0,
                PxTransform::default(),
                (),
            )
            .unwrap();
        actor.set_mass(mass);

        // Debug plane
        let ground_plane = controller
            .physics
            .create_plane(PxVec3::new(0.0, 1.0, 0.0), 0.0, material.as_mut(), ())
            .unwrap();
        controller.scene.add_static_actor(ground_plane);

        actor.set_rigid_dynamic_lock_flag(RigidDynamicLockFlag::LockLinearX, false);
        actor.set_rigid_dynamic_lock_flag(RigidDynamicLockFlag::LockLinearY, false);
        actor.set_rigid_dynamic_lock_flag(RigidDynamicLockFlag::LockLinearZ, false);

        unsafe {
            PxScene_addActor_mut(controller.scene.as_mut_ptr(), actor.as_mut_ptr(), std::ptr::null());
        }
        PhysxPhysicsRigidBodyEntity::create(actor, self.controller.clone())
    }

    fn create_static(&self) -> PhysxPhysicsStaticEntity {
        let mut controller = self.controller.as_ref().write();

        let actor: Owner<PxRigidStatic> = controller
            .physics
            .create_static(PxTransform::from_translation(&PxVec3::new(0.0, 0.0, 0.0)), ())
            .unwrap();
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

    fn step(&mut self, delta: f32, _physics_container: &PhysxPhysicsContainer) {
        self.scene
            .step(delta, None::<&mut physx_sys::PxBaseTask>, None, true)
            .expect("error occured during simulation");
    }
}

#[cfg(test)]
mod tests {
    use physx::math::PxVec3;

    use crate::physics::physics::{PhysicsColliderBuilder, PhysicsContainer};

    use super::{PhysxPhysicsColliderBuilder, PhysxPhysicsContainer};
    use crate::network::messages::Vector3 as NetworkVector3;

    #[test]
    fn test_collider() {
        let vertices = vec![
            PxVec3::new(0., 1., 0.),
            PxVec3::new(0., -1., 0.),
            PxVec3::new(1., 0., 0.),
            PxVec3::new(-1., 0., 0.),
            PxVec3::new(0., 0., 1.),
            PxVec3::new(0., 0., -1.),
        ];
        let mut collider = PhysxPhysicsColliderBuilder::create();
        for v in vertices {
            collider.push_verts(v.x(), v.y(), v.z());
        }

        let container = PhysxPhysicsContainer::create();
        let mut actor = container.create_static();
        collider.update_collider(&mut actor, &NetworkVector3::zero())
    }
}
