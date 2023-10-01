use godot::prelude::Vector3;
use rapier3d::control::{CharacterLength, KinematicCharacterController};
use rapier3d::prelude::*;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

use crate::controller::player_controller::{CONTROLLER_HEIGHT, CONTROLLER_MASS, CONTROLLER_RADIUS};

pub type PhysicsContainerLock = Rc<RefCell<PhysicsController>>;
pub type RigidBodySetLock = Rc<RefCell<RigidBodySet>>;
pub type ColliderSetLock = Rc<RefCell<ColliderSet>>;
pub type QueryPipelineLock = Rc<RefCell<QueryPipeline>>;

/// For bodies with physics
pub struct PhysicsEntity {
    rigid_body_set: RigidBodySetLock,
    collider_set: ColliderSetLock,
    query_pipeline: QueryPipelineLock,

    rigid_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    character_controller: KinematicCharacterController,
}

impl PhysicsEntity {
    pub fn create(
        physics_container: &PhysicsContainer,
        rigid_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    ) -> Self {
        let mut character_controller = KinematicCharacterController::default();
        character_controller.offset = CharacterLength::Relative(0.05);
        Self {
            rigid_body_set: physics_container.rigid_body_set.clone(),
            collider_set: physics_container.collider_set.clone(),
            query_pipeline: physics_container.query_pipeline.clone(),

            rigid_handle,
            collider_handle,
            character_controller,
        }
    }

    pub fn controller_move(&mut self, delta: f64, impulse: Vector<Real>) {
        let collider = self.get_collider().unwrap().clone();
        let corrected_movement = self.character_controller.move_shape(
            delta as f32,
            &RigidBodySet::new(), // &self.rigid_body_set.borrow(),
            &ColliderSet::new(),  // &self.collider_set.borrow(),
            &self.query_pipeline.borrow(),
            collider.shape(),
            collider.position(),
            impulse,
            // Make sure the the character we are trying to move isn’t considered an obstacle.
            QueryFilter::default().exclude_rigid_body(self.rigid_handle),
            // We don’t care about events in this example.
            |_| {},
        );
        let mut body = self.get_rigid_body_mut().unwrap();
        let translation = body.translation().clone();
        body.set_translation(translation + corrected_movement.translation, true);
    }

    pub fn set_enabled(&mut self, active: bool) {
        let mut body = self.get_rigid_body_mut().unwrap();
        body.set_enabled(active);
    }

    pub fn apply_impulse(&mut self, impulse: Vector<Real>) {
        let mut body = self.get_rigid_body_mut().unwrap();
        body.apply_impulse(impulse, true);
    }

    pub fn get_position(&self) -> Vector3 {
        let body = self.get_rigid_body().unwrap();
        PhysicsEntity::transform_to_vector3(&body.translation())
    }

    pub fn set_position(&mut self, position: Vector<Real>) {
        let mut body = self.get_rigid_body_mut().unwrap();
        // Reset velocity
        body.sleep();
        body.set_translation(position, true);
    }

    pub fn get_collider(&self) -> Option<Ref<Collider>> {
        Ref::filter_map(self.collider_set.borrow(), |p| p.get(self.collider_handle)).ok()
    }

    pub fn get_rigid_body(&self) -> Option<Ref<RigidBody>> {
        Ref::filter_map(self.rigid_body_set.borrow(), |p| p.get(self.rigid_handle)).ok()
    }

    pub fn get_rigid_body_mut(&mut self) -> Option<RefMut<RigidBody>> {
        RefMut::filter_map(self.rigid_body_set.borrow_mut(), |p| p.get_mut(self.rigid_handle)).ok()
    }

    pub fn transform_to_vector3(translation: &Vector<Real>) -> Vector3 {
        Vector3::new(translation[0], translation[1], translation[2])
    }
}

/// For stationary bodies
pub struct PhysicsStaticEntity {
    collider_set: ColliderSetLock,
    collider_handle: Option<ColliderHandle>,
}

impl PhysicsStaticEntity {
    pub fn new(physics_container: &PhysicsContainer) -> Self {
        Self {
            collider_set: physics_container.collider_set.clone(),
            collider_handle: None,
        }
    }

    pub fn update_collider(&self, collider: ColliderBuilder, position: &Vector3) {
        let collider = collider.translation(vector![position.x, position.y, position.z]);
        self.collider_handle = Some(self.collider_set.borrow_mut().insert(collider));
    }
}

#[derive(Clone)]
pub struct PhysicsContainer {
    world_physics: PhysicsContainerLock,
    rigid_body_set: RigidBodySetLock,
    collider_set: ColliderSetLock,
    query_pipeline: QueryPipelineLock,
}

impl Default for PhysicsContainer {
    fn default() -> Self {
        Self {
            world_physics: Default::default(),
            rigid_body_set: Rc::new(RefCell::new(RigidBodySet::new())),
            collider_set: Rc::new(RefCell::new(ColliderSet::new())),
            query_pipeline: Rc::new(RefCell::new(QueryPipeline::new())),
        }
    }
}

impl PhysicsContainer {
    pub fn step(&self) {
        self.world_physics.as_ref().borrow_mut().step(
            self.rigid_body_set.borrow_mut(),
            self.collider_set.borrow_mut(),
            self.query_pipeline.borrow_mut(),
        );
    }

    pub fn create_controller(&self) -> PhysicsEntity {
        let mut rigid_body = RigidBodyBuilder::dynamic().build();
        rigid_body.set_enabled_rotations(false, false, false, true);

        let half_height = CONTROLLER_HEIGHT / 2.0; // CONTROLLER_HEIGHT - 1.8
        let radius = CONTROLLER_RADIUS; // CONTROLLER_RADIUS - 0.4
        let collider = ColliderBuilder::cylinder(half_height, radius)
            .mass(CONTROLLER_MASS)
            .restitution(0.0);
        let rigid_handle = self.rigid_body_set.borrow_mut().insert(rigid_body);

        let mut collider_set = self.collider_set.borrow_mut();
        let mut rigid_body_set = self.rigid_body_set.borrow_mut();

        let collider_handle = collider_set.insert_with_parent(collider, rigid_handle, &mut rigid_body_set);

        PhysicsEntity::create(&self, rigid_handle, collider_handle)
    }

    pub fn create_static(&self) -> PhysicsStaticEntity {
        PhysicsStaticEntity::new(&self)
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
    pub fn step(
        &mut self,
        mut rigid_body_set: RefMut<RigidBodySet>,
        mut collider_set: RefMut<ColliderSet>,
        mut query_pipeline: RefMut<QueryPipeline>,
    ) {
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
            Some(&mut query_pipeline),
            &physics_hooks,
            &event_handler,
        );
    }
}
