use godot::prelude::Vector3;
use rapier3d::control::KinematicCharacterController;
use rapier3d::prelude::*;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub type PhysicsContainerLock = Rc<RefCell<PhysicsController>>;
pub type RigidBodySetLock = Rc<RefCell<RigidBodySet>>;
pub type ColliderSetLock = Rc<RefCell<ColliderSet>>;
pub type QueryPipelineLock = Rc<RefCell<QueryPipeline>>;

pub struct PhysicsEntity {
    world_physics: PhysicsContainerLock,
    rigid_body_set: RigidBodySetLock,
    collider_set: ColliderSetLock,
    query_pipeline: QueryPipelineLock,

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
            query_pipeline: physics_container.query_pipeline.clone(),

            rigid_handle,
            collider_handle,
        }
    }

    pub fn controller_move(&mut self, delta: f64, impulse: Vector<Real>) {
        let collider = self.get_collider().unwrap().clone();

        let character_controller = KinematicCharacterController::default();
        let corrected_movement = character_controller.move_shape(
            delta as f32,
            &self.rigid_body_set.borrow(),
            &self.collider_set.borrow(),
            &self.query_pipeline.borrow(),
            collider.shape(),
            collider.position(),
            impulse,
            QueryFilter::default()
                // Make sure the the character we are trying to move isn’t considered an obstacle.
                .exclude_rigid_body(self.rigid_handle.unwrap()),
            |_| {}, // We don’t care about events in this example.
        );
        let mut body = self.get_rigid_body_mut().unwrap();
        let translation = body.translation().clone();
        body.set_translation(translation + corrected_movement.translation, true);
    }

    pub fn set_lock(&mut self, state: bool) {
        let mut body = self.get_rigid_body_mut().unwrap();
        if state && !body.is_fixed() {
            body.set_body_type(RigidBodyType::Fixed, true);
        }
        if !state && !body.is_dynamic() {
            body.set_body_type(RigidBodyType::Dynamic, true);
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vector<Real>) {
        let mut body = self.get_rigid_body_mut().unwrap();
        body.apply_impulse(impulse, true);
    }

    pub fn get_position(&self) -> Vector3 {
        let body = self.get_rigid_body().unwrap();
        PhysicsEntity::transform_to_vector3(&body.translation())
    }

    pub fn set_position(&mut self, position: Vector3) {
        println!("position {}", position);
        let mut body = self.get_rigid_body_mut().unwrap();
        body.set_translation(vector![position.x, position.y, position.z], true);
    }

    pub fn get_collider(&self) -> Option<Ref<Collider>> {
        Ref::filter_map(self.collider_set.borrow(), |p| p.get(self.collider_handle)).ok()
    }

    pub fn get_rigid_body(&self) -> Option<Ref<RigidBody>> {
        Ref::filter_map(self.rigid_body_set.borrow(), |p| p.get(self.rigid_handle.unwrap())).ok()
    }

    pub fn get_rigid_body_mut(&mut self) -> Option<RefMut<RigidBody>> {
        RefMut::filter_map(self.rigid_body_set.borrow_mut(), |p| {
            p.get_mut(self.rigid_handle.unwrap())
        })
        .ok()
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

    pub fn create_capsule(&self, position: &Vector3, half_height: Real, radius: Real) -> PhysicsEntity {
        let mut rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![position.x, position.y, position.z])
            .build();
        rigid_body.set_enabled_rotations(false, false, false, true);

        let collider = ColliderBuilder::capsule_y(half_height, radius);
        let rigid_handle = self.rigid_body_set.borrow_mut().insert(rigid_body);

        let mut collider_set = self.collider_set.borrow_mut();
        let mut rigid_body_set = self.rigid_body_set.borrow_mut();

        let collider_handle = collider_set.insert_with_parent(collider, rigid_handle, &mut rigid_body_set);

        PhysicsEntity::create(&self, Some(rigid_handle), collider_handle)
    }

    pub fn create_mesh(&self, collider: ColliderBuilder, position: &Vector3) -> PhysicsEntity {
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
