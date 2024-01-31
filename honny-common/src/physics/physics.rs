use crate::network::messages::Vector3;

pub trait PhysicsRigidBodyEntity {
    fn create() -> Self;
    fn set_enabled(&mut self, active: bool);
    fn apply_impulse(&mut self, impulse: Vector3);
    fn get_position(&self) -> Vector3;
    fn set_position(&mut self, position: Vector3);
    fn raycast(&self, from: Vector3, to: Vector3) -> Option<(u64, Vector3)>;
}

pub trait PhysicsCharacterController<T: PhysicsRigidBodyEntity> {
    fn create() -> Self;
    fn controller_move(&mut self, entity: &mut T, delta: f64, impulse: Vector3);
}

/// For stationary bodies
pub trait PhysicsStaticEntity {
    fn create() -> Self;
}

pub trait PhysicsColliderBuilder<T: PhysicsStaticEntity> {
    fn create() -> Self;
    fn push_indexes(&mut self, index: [u32; 3]);
    fn push_verts(&mut self, x: f32, y: f32 ,z: f32);
    fn len(&self) -> usize;
    fn update_collider(&mut self, static_entity: &T, position: &Vector3);
    fn compile(&mut self);
}

pub trait PhysicsContainer<T: PhysicsRigidBodyEntity, U: PhysicsStaticEntity> : Clone {
    fn create() -> Self;
    fn step(&self, delta: f32);
    fn create_controller(&self, height: f32, radius: f32, mass: f32) -> T;
    fn create_static(&self) -> U;
}

pub trait PhysicsController<T: PhysicsRigidBodyEntity, U: PhysicsStaticEntity> {
    fn create() -> Self;
    fn step(
        &mut self,
        delta: f32,
        physics_container: impl PhysicsContainer<T, U>,
    );
}
