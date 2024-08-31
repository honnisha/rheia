use crate::network::messages::Vector3;

pub trait IQueryFilter<T: IPhysicsRigidBody>: Default {
    fn exclude_rigid_body(&mut self, rigid_body: &T);
}

pub trait IPhysicsRigidBody: Clone {
    fn set_enabled(&mut self, active: bool);
    fn get_position(&self) -> Vector3;
    fn set_position(&mut self, position: Vector3);
}

pub trait IPhysicsCollider: Clone {
    fn set_position(&mut self, position: Vector3);
    fn set_enabled(&mut self, active: bool);
    fn get_index(&self) -> usize;
    fn remove(&self);
}

pub trait IPhysicsCharacterController<T: IPhysicsRigidBody, C: IPhysicsCollider, F: IQueryFilter<T>> {
    fn create(custom_mass: Option<f32>) -> Self;
    fn move_shape(&mut self, collider: &C, filter: F, delta: f64, impulse: Vector3) -> Vector3;
    fn is_grounded(&mut self) -> bool;
    fn get_custom_mass(&mut self) -> &Option<f32>;
}

pub trait IPhysicsColliderBuilder {
    fn cylinder(half_height: f32, radius: f32) -> Self;
    fn trimesh(verts: Vec<Vector3>, indices: Vec<[u32; 3]>) -> Self;
}

pub trait IPhysicsContainer<T: IPhysicsRigidBody, C: IPhysicsCollider, B: IPhysicsColliderBuilder, F: IQueryFilter<T>>:
    Clone + Default
{
    fn step(&self, delta: f32);

    fn spawn_rigid_body(&self, collider_builder: B) -> (T, C);
    fn spawn_collider(&self, collider_builder: B) -> C;

    fn raycast(&self, dir: Vector3, max_toi: f32, origin: Vector3, filter: F) -> Option<(usize, Vector3)>;
}
