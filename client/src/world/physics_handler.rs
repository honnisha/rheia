use std::sync::Arc;

use godot::prelude::Vector3;
use rapier3d::{prelude::*};
use std::cell::RefCell;

pub type PhysicsContainerLock = Arc<RefCell<PhysicsController>>;

pub struct PhysicsEntity {
    world_physics: PhysicsContainerLock,

    rigid_handle: Option<RigidBodyHandle>,
    collider_handle: ColliderHandle,
}

impl PhysicsEntity {
    pub fn create(
        physics: PhysicsContainerLock,
        rigid_handle: Option<RigidBodyHandle>,
        collider_handle: ColliderHandle,
    ) -> Self {
        Self {
            world_physics: physics,
            rigid_handle,
            collider_handle,
        }
    }

    pub fn get_rigid_body(&self) -> &RigidBody {
        self.world_physics
            .borrow()
            .get_rigid_body(&self.rigid_handle.expect("rigid_handle is None"))
            .expect("rigid body has not been created yet")
    }

    pub fn transform_to_vector3(translation: &Vector<Real>) -> Vector3 {
        Vector3::new(translation[0], translation[1], translation[2])
    }
}

#[derive(Default)]
pub struct PhysicsContainer {
    world_physics: PhysicsContainerLock,
}

impl PhysicsContainer {
    pub fn step(&self) {
        self.world_physics.borrow_mut().step();
    }

    pub fn create_cylinder(&mut self, position: &Vector3, half_height: Real, radius: Real) -> PhysicsEntity {
        let mut physics = self.world_physics.borrow_mut();

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![position.x, position.y, position.z])
            .build();
        let collider = ColliderBuilder::cylinder(half_height, radius);
        let rigid_handle = physics.rigid_body_set.insert(rigid_body);
        let collider_handle =
            physics
                .collider_set
                .insert_with_parent(collider, rigid_handle, &mut physics.rigid_body_set);

        PhysicsEntity::create(self.world_physics.clone(), Some(rigid_handle), collider_handle)
    }

    pub fn create_mesh(&mut self, collider: ColliderBuilder, position: &Vector3) -> PhysicsEntity {
        let mut physics = self.world_physics.borrow_mut();

        let collider = collider.translation(vector![position.x, position.y, position.z]);
        let collider_handle = physics.collider_set.insert(collider);
        PhysicsEntity::create(self.world_physics.clone(), None, collider_handle)
    }
}

struct PhysicsController {
    gravity: Vector<Real>,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl Default for PhysicsController {
    fn default() -> Self {
        Self {
            gravity: vector![0.0, -9.81, 0.0],
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }
}

impl PhysicsController {
    pub fn get_rigid_body(&self, handle: &RigidBodyHandle) -> Option<&RigidBody> {
        self.rigid_body_set.get(*handle)
    }

    pub fn get_rigid_body_mut(&mut self, handle: &RigidBodyHandle) -> Option<&mut RigidBody> {
        self.rigid_body_set.get_mut(*handle)
    }

    pub fn step(&mut self) {
        let physics_hooks = ();
        let event_handler = ();
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &physics_hooks,
            &event_handler,
        );
    }
}
