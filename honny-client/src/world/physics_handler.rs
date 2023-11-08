use godot::prelude::Vector3;
use parking_lot::{MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use rapier3d::control::{CharacterAutostep, CharacterLength, KinematicCharacterController};
use rapier3d::na::{Point3};
use rapier3d::prelude::*;
use std::sync::Arc;
use rapier3d::na;

use crate::controller::player_controller::{CONTROLLER_HEIGHT, CONTROLLER_MASS, CONTROLLER_RADIUS};

pub type PhysicsControllerLock = Arc<RwLock<PhysicsController>>;
pub type RigidBodySetLock = Arc<RwLock<RigidBodySet>>;
pub type ColliderSetLock = Arc<RwLock<ColliderSet>>;
pub type QueryPipelineLock = Arc<RwLock<QueryPipeline>>;
pub type IslandManagerLock = Arc<RwLock<IslandManager>>;

/// For bodies with physics
pub struct PhysicsEntity {
    rigid_body_set: RigidBodySetLock,
    collider_set: ColliderSetLock,
    query_pipeline: QueryPipelineLock,

    rigid_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    character_controller: KinematicCharacterController,
}

fn _distance(point: &Vector<Real>, target: &Vector<Real>) -> f32 {
    ((target.x as f32 - point.x as f32).powf(2.0)
        + (target.y as f32 - point.y as f32).powf(2.0)
        + (target.z as f32 - point.z as f32).powf(2.0))
    .sqrt()
}

fn vec_gd_to_na(from: Vector3) -> na::Vector3<f32> {
    na::Vector3::new(from.x, from.y, from.z)
}

impl PhysicsEntity {
    pub fn create(
        physics_container: &PhysicsContainer,
        rigid_handle: RigidBodyHandle,
        collider_handle: ColliderHandle,
    ) -> Self {
        let mut character_controller = KinematicCharacterController::default();
        character_controller.offset = CharacterLength::Relative(0.1);
        character_controller.autostep = Some(CharacterAutostep {
            max_height: CharacterLength::Relative(0.5),
            min_width: CharacterLength::Relative(0.5),
            include_dynamic_bodies: true,
        });

        Self {
            rigid_body_set: physics_container.rigid_body_set.clone(),
            collider_set: physics_container.collider_set.clone(),
            query_pipeline: physics_container.query_pipeline.clone(),

            rigid_handle,
            collider_handle,
            character_controller,
        }
    }

    pub fn controller_move(&mut self, delta: f64, impulse: Vector3) {
        let collider = self.get_collider().unwrap().clone();
        let filter = QueryFilter::default().exclude_rigid_body(self.rigid_handle);

        let corrected_movement = self.character_controller.move_shape(
            delta as f32,
            // &RigidBodySet::new(),
            &self.rigid_body_set.read(),
            // &ColliderSet::new(),
            &self.collider_set.read(),
            &self.query_pipeline.read(),
            collider.shape(),
            collider.position(),
            vec_gd_to_na(impulse),
            filter,
            |_| {},
        );
        let mut body = self.get_rigid_body_mut().unwrap();
        let translation = body.translation().clone();
        body.set_translation(translation + corrected_movement.translation, true);
    }

    pub fn set_enabled(&mut self, active: bool) {
        let mut body = self
            .get_rigid_body_mut()
            .expect("physics entity dosesn't have rigid body");
        body.set_enabled(active);
    }

    pub fn apply_impulse(&mut self, impulse: Vector3) {
        let mut body = self.get_rigid_body_mut().unwrap();
        body.apply_impulse(vec_gd_to_na(impulse), true);
    }

    pub fn get_position(&self) -> Vector3 {
        let body = self.get_rigid_body().unwrap();
        PhysicsEntity::transform_to_vector3(&body.translation())
    }

    pub fn set_position(&mut self, position: Vector3) {
        let mut body = self.get_rigid_body_mut().unwrap();
        // Reset velocity
        body.sleep();
        body.set_translation(vec_gd_to_na(position), true);
    }

    pub fn get_collider(&self) -> Option<MappedRwLockReadGuard<'_, Collider>> {
        RwLockReadGuard::try_map(self.collider_set.read(), |p| match p.get(self.collider_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn get_rigid_body(&self) -> Option<MappedRwLockReadGuard<RigidBody>> {
        RwLockReadGuard::try_map(self.rigid_body_set.read(), |p| match p.get(self.rigid_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn get_rigid_body_mut(&mut self) -> Option<MappedRwLockWriteGuard<RigidBody>> {
        RwLockWriteGuard::try_map(self.rigid_body_set.write(), |p| match p.get_mut(self.rigid_handle) {
            Some(c) => Some(c),
            None => None,
        })
        .ok()
    }

    pub fn transform_to_vector3(translation: &Vector<Real>) -> Vector3 {
        Vector3::new(translation[0], translation[1], translation[2])
    }

    // https://docs.godotengine.org/en/stable/classes/class_node3d.html#class-node3d-property-rotation
    pub fn raycast(&self, from: Vector3, to: Vector3) -> Option<(ColliderHandle, Point<Real>)> {
        let dir = to - from;
        let (dir, max_toi) = (dir.normalized(), dir.length());
        let origin = Point3::new(from.x, from.y, from.z);
        let direction = vec_gd_to_na(dir);

        let ray = Ray::new(origin, direction);

        let solid = true;
        let filter = QueryFilter::default().exclude_rigid_body(self.rigid_handle);

        let pipeline = self.query_pipeline.read();
        if let Some((handle, toi)) = pipeline.cast_ray(
            &self.rigid_body_set.read(),
            &self.collider_set.read(),
            &ray,
            max_toi,
            solid,
            filter,
        ) {
            // The first collider hit has the handle `handle` and it hit after
            // the ray travelled a distance equal to `ray.dir * toi`.
            // let hit_point = ; // Same as: `ray.origin + ray.dir * toi`
            // println!("Collider {:?} hit at point {}", handle, hit_point);
            return Some((handle, ray.point_at(toi)));
        }
        println!("Raycast not found origin:{} direction:{}", origin, direction);
        return None;
    }
}

/// For stationary bodies
pub struct PhysicsStaticEntity {
    collider_set: ColliderSetLock,
    rigid_body_set: RigidBodySetLock,
    island_manager: IslandManagerLock,

    collider_handle: Option<ColliderHandle>,
}

impl PhysicsStaticEntity {
    pub fn new(physics_container: &PhysicsContainer) -> Self {
        Self {
            collider_set: physics_container.collider_set.clone(),
            rigid_body_set: physics_container.rigid_body_set.clone(),
            island_manager: physics_container.island_manager.clone(),
            collider_handle: None,
        }
    }

    pub fn get_collider_mut(&self) -> Option<MappedRwLockWriteGuard<'_, Collider>> {
        if self.collider_handle.is_none() {
            return None;
        }
        RwLockWriteGuard::try_map(self.collider_set.write(), |p| {
            match p.get_mut(self.collider_handle.unwrap()) {
                Some(c) => Some(c),
                None => None,
            }
        })
        .ok()
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
                        self.collider_handle = Some(self.collider_set.write().insert(collider));
                    }
                }
            }
            None => {
                if let Some(stored_collider) = self.collider_handle {
                    // Remove old collider
                    self.collider_set.write().remove(
                        stored_collider,
                        &mut self.island_manager.write(),
                        &mut self.rigid_body_set.write(),
                        true,
                    );
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PhysicsContainer {
    world_physics: PhysicsControllerLock,
    rigid_body_set: RigidBodySetLock,
    collider_set: ColliderSetLock,
    query_pipeline: QueryPipelineLock,
    island_manager: IslandManagerLock,
}

impl Default for PhysicsContainer {
    fn default() -> Self {
        Self {
            world_physics: Default::default(),
            rigid_body_set: Arc::new(RwLock::new(RigidBodySet::new())),
            collider_set: Arc::new(RwLock::new(ColliderSet::new())),
            query_pipeline: Arc::new(RwLock::new(QueryPipeline::new())),
            island_manager: Arc::new(RwLock::new(IslandManager::new())),
        }
    }
}

impl PhysicsContainer {
    pub fn step(&self, delta: f32) {
        self.world_physics.as_ref().write().step(delta, &self);
    }

    pub fn create_controller(&self) -> PhysicsEntity {
        let mut rigid_body = RigidBodyBuilder::dynamic().build();
        rigid_body.set_enabled_rotations(false, false, false, true);

        let half_height = CONTROLLER_HEIGHT / 2.0;
        let radius = CONTROLLER_RADIUS;
        let collider = ColliderBuilder::cylinder(half_height, radius)
            .mass(CONTROLLER_MASS)
            .restitution(0.0);
        let rigid_handle = self.rigid_body_set.write().insert(rigid_body);

        let mut collider_set = self.collider_set.write();
        let mut rigid_body_set = self.rigid_body_set.write();

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
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }
}

impl PhysicsController {
    pub fn step(&mut self, delta: f32, physics_container: &PhysicsContainer) {
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
