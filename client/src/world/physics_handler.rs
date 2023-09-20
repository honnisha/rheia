use godot::prelude::Vector3;
use rapier3d::prelude::*;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub type PhysicsContainerLock = Rc<RefCell<PhysicsController>>;
pub type RigidBodySetLock = Rc<RefCell<RigidBodySet>>;
pub type ColliderSetLock = Rc<RefCell<ColliderSet>>;

pub struct PhysicsEntity {
    world_physics: PhysicsContainerLock,
    rigid_body_set: RigidBodySetLock,
    collider_set: ColliderSetLock,

    rigid_handle: Option<RigidBodyHandle>,
    collider_handle: ColliderHandle,
}

impl PhysicsEntity {
    pub fn create(
        physics_container: &PhysicsContainer,
        rigid_handle: Option<RigidBodyHandle>,
        collider_handle: ColliderHandle,
    ) -> Self {
        Self {
            world_physics: physics_container.world_physics.clone(),
            rigid_body_set: physics_container.rigid_body_set.clone(),
            collider_set: physics_container.collider_set.clone(),

            rigid_handle,
            collider_handle,
        }
    }

    pub fn get_rigid_body(&self) -> Option<Ref<RigidBody>> {
        Ref::filter_map(self.rigid_body_set.borrow(), |p| p.get(self.rigid_handle.unwrap())).ok()
    }

    pub fn get_rigid_body_mut(&mut self) -> Option<RefMut<RigidBody>> {
        RefMut::filter_map(self.rigid_body_set.borrow_mut(), |p| p.get_mut(self.rigid_handle.unwrap())).ok()
    }

    pub fn transform_to_vector3(translation: &Vector<Real>) -> Vector3 {
        Vector3::new(translation[0], translation[1], translation[2])
    }

    pub fn rotation_to_vector3(rotation: &Rotation<Real>) -> Vector3 {
        Vector3::new(rotation[0], rotation[1], rotation[2])
    }
}

pub struct PhysicsContainer {
    world_physics: PhysicsContainerLock,
    rigid_body_set: RigidBodySetLock,
    collider_set: ColliderSetLock,
}

impl Default for PhysicsContainer {
    fn default() -> Self {
        Self {
            world_physics: Default::default(),
            rigid_body_set: Rc::new(RefCell::new(RigidBodySet::new())),
            collider_set: Rc::new(RefCell::new(ColliderSet::new())),
        }
    }
}

impl PhysicsContainer {
    pub fn step(&self) {
        self.world_physics
            .as_ref()
            .borrow_mut()
            .step(self.rigid_body_set.borrow_mut(), self.collider_set.borrow_mut());
    }

    pub fn get_physics(&self) -> Ref<PhysicsController> {
        self.world_physics.borrow()
    }

    pub fn get_physics_mut(&self) -> RefMut<PhysicsController> {
        self.world_physics.borrow_mut()
    }

    pub fn create_capsule(&self, position: &Vector3, half_height: Real, radius: Real) -> PhysicsEntity {
        let mut physics = self.get_physics_mut();

        let mut rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![position.x, position.y, position.z])
            .build();
        rigid_body.restrict_rotations(false, false, false, true);

        let collider = ColliderBuilder::capsule_y(half_height, radius);
        let rigid_handle = self.rigid_body_set.borrow_mut().insert(rigid_body);

        let mut collider_set = self.collider_set.borrow_mut();
        let mut rigid_body_set = self.rigid_body_set.borrow_mut();

        let collider_handle = collider_set.insert_with_parent(collider, rigid_handle, &mut rigid_body_set);

        PhysicsEntity::create(&self, Some(rigid_handle), collider_handle)
    }

    pub fn create_mesh(&self, collider: ColliderBuilder, position: &Vector3) -> PhysicsEntity {
        let mut physics = self.get_physics_mut();

        let collider = collider.translation(vector![position.x, position.y, position.z]);
        let collider_handle = self.collider_set.borrow_mut().insert(collider);
        PhysicsEntity::create(&self, None, collider_handle)
    }
}

pub struct PhysicsController {
    gravity: Vector<Real>,
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
    pub fn step(&mut self, mut rigid_body_set: RefMut<RigidBodySet>, mut collider_set: RefMut<ColliderSet>) {
        let physics_hooks = ();
        let event_handler = ();
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &physics_hooks,
            &event_handler,
        );
    }
}
