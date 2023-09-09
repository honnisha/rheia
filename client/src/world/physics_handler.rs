use godot::prelude::Vector3;
use rapier3d::prelude::*;

pub(crate) struct PhysicsController {
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
    pub fn create_ball(&mut self, position: &Vector3) -> RigidBodyHandle {
        /* Create the bounding ball. */
        let rigid_body = RigidBodyBuilder::dynamic().translation(vector![position.x, position.y, position.z]).build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let body_handle = self.rigid_body_set.insert(rigid_body);
        self.collider_set.insert_with_parent(collider, body_handle, &mut self.rigid_body_set);
        body_handle
    }

    pub fn get_rigid_body(&self, handle: &RigidBodyHandle) -> Option<&RigidBody> {
        self.rigid_body_set.get(*handle)
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
