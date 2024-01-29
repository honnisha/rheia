use crate::network::messages::Vector3;


pub trait PhysicsRigidBodyEntity {
    fn create() -> Self;
    fn set_enabled(&mut self, active: bool);
    fn apply_impulse(&mut self, impulse: Vector3);
    fn get_position(&self) -> Vector3;
    fn set_position(&mut self, position: Vector3);
    fn raycast(&self, from: Vector3, to: Vector3) -> Option<(ColliderHandle, Point<Real>)>;
}

pub trait PhysicsCharacterController {
    fn create() -> Self;
    fn controller_move(&mut self, entity: &mut dyn PhysicsRigidBodyEntity, delta: f64, impulse: Vector3);
}

/// For stationary bodies
pub trait PhysicsStaticEntity {
    fn new(physics_container: &dyn PhysicsContainer) -> Self;
    fn update_collider(&mut self, collider: Option<ColliderBuilder>, position: &Vector3);
}

pub trait PhysicsContainer {
    fn create() -> Self;
    fn step(&self, delta: f32);
    fn create_controller(&self) -> dyn PhysicsRigidBodyEntity;
    fn create_static(&self) -> dyn PhysicsStaticEntity;
}

pub trait PhysicsController {
    fn create() -> Self;
    fn step(&mut self, delta: f32, physics_container: &dyn PhysicsContainer);
}
