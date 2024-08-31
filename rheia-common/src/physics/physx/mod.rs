pub mod bridge;
pub mod types;

use super::physics::{
    IPhysicsCharacterController, IPhysicsCollider, IPhysicsColliderBuilder, IPhysicsContainer, IPhysicsRigidBody,
    IQueryFilter,
};
use crate::network::messages::{IntoNetworkVector, Vector3 as NetworkVector3};
use bridge::IntoPxVec3;
use parking_lot::RwLock;
use physx::cooking::PxTriangleMeshDesc;
use physx::{cooking, foundation::DefaultAllocator, owner::Owner, prelude::*, traits::Class};
use physx_sys::{
    PxHitFlags, PxMeshScale_new, PxPhysics_createShape_mut, PxQueryFilterData, PxQueryFilterData_new,
    PxSceneQueryExt_raycastSingle, PxScene_addActor_mut, PxShape_setLocalPose_mut,
};
use std::{ffi::c_void, mem::MaybeUninit, ptr::null_mut};
use types::*;

// https://github.com/EmbarkStudios/physx-rs

pub struct PhysxQueryFilter {
    filter: PxQueryFilterData,
}

impl Default for PhysxQueryFilter {
    fn default() -> Self {
        Self {
            filter: unsafe { PxQueryFilterData_new() },
        }
    }
}

impl IQueryFilter<PhysxPhysicsRigidBody> for PhysxQueryFilter {
    fn exclude_rigid_body(&mut self, _rigid_body: &PhysxPhysicsRigidBody) {
        todo!()
    }
}

#[derive(Clone)]
pub struct PhysxPhysicsRigidBody {
    actor: Owner<PxRigidDynamic>,
    controller: Arc<RwLock<PhysxPhysicsController>>,
}

impl PhysxPhysicsRigidBody {
    fn create(actor: Owner<PxRigidDynamic>, controller: Arc<RwLock<PhysxPhysicsController>>) -> Self {
        Self { actor, controller }
    }
}

impl IPhysicsRigidBody for PhysxPhysicsRigidBody {
    fn set_enabled(&mut self, active: bool) {
        self.actor.enable_gravity(active)
    }

    fn get_position(&self) -> NetworkVector3 {
        self.actor.get_global_position().to_network()
    }

    fn set_position(&mut self, position: NetworkVector3) {
        self.actor
            .set_global_pose(&PxTransform::from_translation(&position.to_physx()), true);
    }
}

pub struct PhysxPhysicsCharacterController {
    custom_mass: Option<f32>,
}

impl IPhysicsCharacterController<PhysxPhysicsRigidBody, PhysxPhysicsCollider, PhysxQueryFilter>
    for PhysxPhysicsCharacterController
{
    fn create(custom_mass: Option<f32>) -> Self {
        Self { custom_mass }
    }

    fn move_shape(
        &mut self,
        _collider: &PhysxPhysicsCollider,
        _filter: PhysxQueryFilter,
        _delta: f64,
        _impulse: NetworkVector3,
    ) -> NetworkVector3 {
        // https://github.com/rlidwka/bevy_mod_physx/blob/ef9e56023fb7500c7e5d1f2b66057a16a3caf8d7/examples/kinematic.rs
        todo!()
    }

    fn is_grounded(&mut self) -> bool {
        false
    }

    fn get_custom_mass(&mut self) -> &Option<f32> {
        &self.custom_mass
    }
}

#[derive(Clone)]
pub struct PhysxPhysicsCollider {
    actor: Owner<PxRigidStatic>,
    controller: Arc<RwLock<PhysxPhysicsController>>,

    // Attached shape
    shape: Option<Owner<PxShape>>,
}

impl PhysxPhysicsCollider {
    fn create(actor: Owner<PxRigidStatic>, controller: Arc<RwLock<PhysxPhysicsController>>) -> Self {
        Self {
            actor,
            controller,
            shape: Default::default(),
        }
    }
}

impl IPhysicsCollider for PhysxPhysicsCollider {
    fn set_enabled(&mut self, _active: bool) {
        todo!();
    }

    fn get_index(&self) -> usize {
        todo!()
    }

    fn set_position(&mut self, _position: NetworkVector3) {
        todo!()
    }

    fn remove(&self) {
        if self.shape.is_some() {
            self.actor.detach_shape(&mut self.shape.take().unwrap());
        }
    }
}

pub struct PhysxPhysicsColliderBuilder {}

impl IPhysicsColliderBuilder for PhysxPhysicsColliderBuilder {
    fn cylinder(half_height: f32, radius: f32) -> Self {
        let geometry = PxCapsuleGeometry::new(radius, half_height);
    }

    fn trimesh(verts: Vec<NetworkVector3>, indices: Vec<[u32; 3]>) -> Self {
        let mut controller = static_entity.controller.write();

        let verts = verts.into_iter().map(|v| PxVec3::new(v.x, v.y, v.z)).collect();

        let mut desc = PxTriangleMeshDesc::new();
        desc.obj.points.count = verts.len() as u32;
        desc.obj.points.stride = std::mem::size_of::<PxVec3>() as u32;
        desc.obj.points.data = verts.as_ptr() as *const c_void;

        desc.obj.triangles.count = indices.len() as u32;
        desc.obj.triangles.stride = std::mem::size_of::<[u32; 3]>() as u32;
        desc.obj.triangles.data = indices.as_ptr() as *const c_void;

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
    }

    fn update_collider(&mut self, static_entity: &mut PhysxPhysicsStaticEntity, position: &NetworkVector3) {
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
}

#[derive(Clone)]
pub struct PhysxPhysicsContainer {
    controller: Arc<RwLock<PhysxPhysicsController>>,
}

impl Default for PhysxPhysicsContainer {
    fn default() -> Self {
        let controller = Arc::new(RwLock::new(PhysxPhysicsController::create()));
        Self { controller }
    }
}

impl IPhysicsContainer<PhysxPhysicsRigidBody, PhysxPhysicsCollider, PhysxPhysicsColliderBuilder, PhysxQueryFilter>
    for PhysxPhysicsContainer
{
    fn step(&self, delta: f32) {
        self.controller.as_ref().write().step(delta, self);
    }

    fn spawn_rigid_body(
        &self,
        collider_builder: PhysxPhysicsColliderBuilder,
    ) -> (PhysxPhysicsRigidBody, PhysxPhysicsCollider) {
        let mut controller = self.controller.as_ref().write();

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
        PhysxPhysicsRigidBody::create(actor, self.controller.clone())
    }

    fn spawn_collider(&self, collider_builder: PhysxPhysicsColliderBuilder) -> PhysxPhysicsCollider {
        let mut controller = self.controller.as_ref().write();

        let actor: Owner<PxRigidStatic> = controller
            .physics
            .create_static(PxTransform::from_translation(&PxVec3::new(0.0, 0.0, 0.0)), ())
            .unwrap();
        PhysxPhysicsCollider::create(actor, self.controller.clone())
    }

    fn raycast(
        &self,
        dir: NetworkVector3,
        max_toi: f32,
        origin: NetworkVector3,
        filter: PhysxQueryFilter,
    ) -> Option<(usize, NetworkVector3)> {
        let controller = self.controller.as_ref().read();

        let mut raycast_hit = MaybeUninit::uninit();

        if !unsafe {
            PxSceneQueryExt_raycastSingle(
                controller.scene.as_ptr(),
                &origin.to_physx_sys(),
                &dir.to_physx_sys(),
                max_toi,
                PxHitFlags::Default,
                raycast_hit.as_mut_ptr(),
                &filter.filter as *const _,
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
    use crate::physics::physics::{IPhysicsColliderBuilder, IPhysicsContainer};

    use super::{PhysxPhysicsColliderBuilder, PhysxPhysicsContainer};
    use crate::network::messages::Vector3 as NetworkVector3;

    #[test]
    fn test_collider() {
        let vertices = vec![
            NetworkVector3::new(0., 1., 0.),
            NetworkVector3::new(0., -1., 0.),
            NetworkVector3::new(1., 0., 0.),
            NetworkVector3::new(-1., 0., 0.),
            NetworkVector3::new(0., 0., 1.),
            NetworkVector3::new(0., 0., -1.),
        ];
        let indices: Vec<[u32; 3]> = Default::default();
        let collider = PhysxPhysicsColliderBuilder::trimesh(vertices, indices);

        let container = PhysxPhysicsContainer::default();
        let _actor = container.spawn_collider(collider);
    }
}
