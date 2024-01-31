use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use rapier3d::control::{CharacterAutostep, CharacterLength, KinematicCharacterController};
use rapier3d::na;
use rapier3d::na::Point3;
use rapier3d::prelude::*;
use std::sync::Arc;

use crate::network::messages::Vector3;

use super::physics::{PhysicsContainer, PhysicsController, PhysicsRigidBodyEntity, PhysicsStaticEntity, PhysicsCharacterController};

fn _distance(point: &Vector<Real>, target: &Vector<Real>) -> f32 {
    ((target.x as f32 - point.x as f32).powf(2.0)
        + (target.y as f32 - point.y as f32).powf(2.0)
        + (target.z as f32 - point.z as f32).powf(2.0))
    .sqrt()
}

fn vec_gd_to_na(from: Vector3) -> na::Vector3<f32> {
    na::Vector3::new(from.x, from.y, from.z)
}

fn vec_na_to_gd(from: &na::Vector3<f32>) -> Vector3 {
    Vector3::new(from.x, from.y, from.z)
}

    let mut collider_verts: Vec<Point<Real>> = Default::default();
let mut collider_indices: Vec<[u32; 3]> = Default::default();

pub struct RapierPhysicsColliderBuilder {}
impl PhysicsColliderBuilder for RapierPhysicsColliderBuilder {
    fn create() -> Self {
        todo!()
    }

    fn push_indexes(&mut self, index: [u32; 3]) {
        todo!()
    }

    fn push_collider_verts(&mut self, x: f32, y: f32, z: f32) {
        todo!()
    }
}

/// For bodies with physics
pub struct RapierPhysicsRigidBodyEntity {
    physics_container: RapierPhysicsContainer,

    rigid_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
}

impl PhysicsRigidBodyEntity for RapierPhysicsRigidBodyEntity {
    fn create(
        physics_container: &RapierPhysicsContainer,
        rigid_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    ) -> Self {
        Self {
            physics_container: physics_container.clone(),
            rigid_handle,
            collider_handle,
        }
    }

    fn set_enabled(&mut self, active: bool) {
        let mut body = self
            .physics_container
            .get_rigid_body_mut(&self.rigid_handle)
            .expect("physics entity dosesn't have rigid body");
        body.set_enabled(active);
    }

    fn apply_impulse(&mut self, impulse: Vector3) {
        let mut body = self.physics_container.get_rigid_body_mut(&self.rigid_handle).unwrap();
        body.apply_impulse(vec_gd_to_na(impulse), true);
    }

    fn get_position(&self) -> Vector3 {
        let body = self.physics_container.get_rigid_body(&self.rigid_handle).unwrap();
        vec_na_to_gd(&body.translation())
    }

    fn set_position(&mut self, position: Vector3) {
        let mut body = self.physics_container.get_rigid_body_mut(&self.rigid_handle).unwrap();
        // Reset velocity
        body.sleep();
        body.set_translation(vec_gd_to_na(position), true);
    }

    // https://docs.godotengine.org/en/stable/classes/class_node3d.html#class-node3d-property-rotation
    fn raycast(&self, from: Vector3, to: Vector3) -> Option<(ColliderHandle, Point<Real>)> {
        let dir = to - from;
        let (dir, max_toi) = (dir.normalized(), dir.length());
        let origin = Point3::new(from.x, from.y, from.z);
        let direction = vec_gd_to_na(dir);

        let ray = Ray::new(origin, direction);

        let solid = true;
        let filter = QueryFilter::default().exclude_rigid_body(self.rigid_handle);

        let pipeline = self.physics_container.query_pipeline.read();
        if let Some((handle, toi)) = pipeline.cast_ray(
            &self.physics_container.rigid_body_set.read(),
            &self.physics_container.collider_set.read(),
            &ray,
            max_toi,
            solid,
            filter,
        ) {
            return Some((handle, ray.point_at(toi)));
        }
        return None;
    }
}

pub struct RapierPhysicsCharacterController {
    character_controller: KinematicCharacterController,
}

impl PhysicsCharacterController for RapierPhysicsCharacterController {
    fn create() -> Self {
        let mut character_controller = KinematicCharacterController::default();
        character_controller.offset = CharacterLength::Relative(0.1);
        character_controller.autostep = Some(CharacterAutostep {
            max_height: CharacterLength::Relative(0.5),
            min_width: CharacterLength::Relative(0.5),
            include_dynamic_bodies: false,
        });
        Self { character_controller }
    }

    fn controller_move(&mut self, entity: &mut dyn PhysicsRigidBodyEntity, delta: f64, impulse: Vector3) {
        let collider = entity
            .physics_container
            .get_collider(&entity.collider_handle)
            .unwrap()
            .clone();
        let filter = QueryFilter::default().exclude_rigid_body(entity.rigid_handle);

        let corrected_movement = self.character_controller.move_shape(
            delta as f32,
            &entity.physics_container.rigid_body_set.read(),
            &entity.physics_container.collider_set.read(),
            &entity.physics_container.query_pipeline.read(),
            collider.shape(),
            collider.position(),
            vec_gd_to_na(impulse),
            filter,
            |_| {},
        );
        let mut body = entity
            .physics_container
            .get_rigid_body_mut(&entity.rigid_handle)
            .unwrap();
        let translation = body.translation().clone();
        body.set_translation(translation + corrected_movement.translation, true);
    }
}

/// For stationary bodies
pub struct RapierPhysicsStaticEntity {
    physics_container: RapierPhysicsRigidBodyEntity,

    collider_handle: Option<ColliderHandle>,
}

impl RapierPhysicsStaticEntity {
    pub fn new(physics_container: &dyn PhysicsContainer<dyn PhysicsRigidBodyEntity, dyn PhysicsStaticEntity>) -> Self {
        Self {
            physics_container: physics_container.clone(),
            collider_handle: None,
        }
    }

    pub fn _has_collider(&self) -> bool {
        self.collider_handle.is_some()
    }

    // This function causes a thread lock with collider_set
    pub fn update_collider(&mut self, collider: Option<ColliderBuilder>, position: &Vector3) {
        match collider {
            Some(c) => {
                match self.collider_handle {
                    Some(_old_collider) => {
                        // Update existing collider
                        todo!()
                    }
                    None => {
                        // Spawn new collider
                        let collider = c.translation(vector![position.x, position.y, position.z]);
                        self.collider_handle = Some(self.physics_container.collider_set.write().insert(collider));
                    }
                }
            }
            None => {
                if let Some(stored_collider) = self.collider_handle {
                    // Remove old collider
                    self.physics_container.collider_set.write().remove(
                        stored_collider,
                        &mut self.physics_container.island_manager.write(),
                        &mut self.physics_container.rigid_body_set.write(),
                        true,
                    );
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct RapierPhysicsContainer {
    world_physics: Arc<RwLock<RapierPhysicsContainer>>,
    rigid_body_set: Arc<RwLock<RigidBodySet>>,
    collider_set: Arc<RwLock<ColliderSet>>,
    query_pipeline: Arc<RwLock<QueryPipeline>>,
    island_manager: Arc<RwLock<IslandManager>>,
}

impl RapierPhysicsContainer {
    pub fn get_collider(&self, collider_handle: &ColliderHandle) -> Option<MappedRwLockReadGuard<'_, Collider>> {
        RwLockReadGuard::try_map(self.collider_set.read(), |p| match p.get(*collider_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn get_collider_mut(&self, collider_handle: &ColliderHandle) -> Option<MappedRwLockWriteGuard<'_, Collider>> {
        RwLockWriteGuard::try_map(self.collider_set.write(), |p| match p.get_mut(*collider_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn get_rigid_body(&self, rigid_handle: &RigidBodyHandle) -> Option<MappedRwLockReadGuard<RigidBody>> {
        RwLockReadGuard::try_map(self.rigid_body_set.read(), |p| match p.get(*rigid_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn get_rigid_body_mut(&mut self, rigid_handle: &RigidBodyHandle) -> Option<MappedRwLockWriteGuard<RigidBody>> {
        RwLockWriteGuard::try_map(self.rigid_body_set.write(), |p| match p.get_mut(*rigid_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }
}

impl PhysicsContainer<RapierPhysicsRigidBodyEntity, RapierPhysicsStaticEntity> for RapierPhysicsContainer {
    fn create() -> Self {
        Self {
            world_physics: Default::default(),
            rigid_body_set: Arc::new(RwLock::new(RigidBodySet::new())),
            collider_set: Arc::new(RwLock::new(ColliderSet::new())),
            query_pipeline: Arc::new(RwLock::new(QueryPipeline::new())),
            island_manager: Arc::new(RwLock::new(IslandManager::new())),
        }
    }

    fn step(&self, delta: f32) {
        self.world_physics.as_ref().write().step(delta, &self);
    }

    fn create_controller(&self, height: f32, radius: f32, mass: f32) -> RapierPhysicsRigidBodyEntity {
        let mut rigid_body = RigidBodyBuilder::dynamic().build();
        rigid_body.set_enabled_rotations(false, false, false, true);

        let half_height = height / 2.0;
        let radius = radius;
        let collider = ColliderBuilder::cylinder(half_height, radius)
            .mass(mass)
            .restitution(0.0);
        let rigid_handle = self.rigid_body_set.write().insert(rigid_body);

        let mut collider_set = self.collider_set.write();
        let mut rigid_body_set = self.rigid_body_set.write();

        let collider_handle = collider_set.insert_with_parent(collider, rigid_handle, &mut rigid_body_set);

        PhysicsRigidBodyEntity::create(&self, rigid_handle, collider_handle)
    }

    fn create_static(&self) -> RapierPhysicsStaticEntity {
        PhysicsStaticEntity::new(&self)
    }
}

pub struct RapierPhysicsController {
    gravity: Vector<Real>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl PhysicsController<RapierPhysicsRigidBodyEntity, RapierPhysicsStaticEntity> for RapierPhysicsController {
    fn create() -> Self {
        Self {
            gravity: vector![0.0, -9.81, 0.0],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }

    fn step(&mut self, delta: f32, physics_container: &RapierPhysicsContainer) {
        self.integration_parameters.dt = delta;

        let physics_hooks = ();
        let event_handler = ();
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut physics_container.island_manager.write(),
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut physics_container.rigid_body_set.write(),
            &mut physics_container.collider_set.write(),
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut physics_container.query_pipeline.write()),
            &physics_hooks,
            &event_handler,
        );
    }
}
