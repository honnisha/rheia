use crate::{chunks::block_position::BlockPosition, network::messages::Vector3};

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

    fn cast_ray_and_get_normal(
        &self,
        dir: Vector3,
        max_toi: f32,
        origin: Vector3,
        filter: F,
    ) -> Option<RayCastResultNormal>;
}

#[derive(Debug)]
pub struct RayCastResultNormal {
    pub collider_id: usize,
    pub point: Vector3,
    pub normal: Vector3,
}

impl RayCastResultNormal {
    pub fn get_place_block(&self) -> BlockPosition {
        let p = self.point.clone() + self.normal.clone() * 0.5;
        BlockPosition::from_position(&p)
    }

    pub fn get_selected_block(&self) -> BlockPosition {
        let p = self.point.clone() - self.normal.clone() * 0.5;
        BlockPosition::from_position(&p)
    }
}

#[cfg(test)]
mod tests {
    use super::RayCastResultNormal;
    use crate::{chunks::block_position::BlockPosition, network::messages::Vector3};

    #[test]
    fn test_normal() {
        let result = RayCastResultNormal {
            collider_id: 0,
            point: Vector3::new(0.8913913, 25.0, 0.7640153),
            normal: Vector3::new(0.0, 1.0, 0.0),
        };
        assert_eq!(result.get_place_block(), BlockPosition::new(0, 25, 0));
        assert_eq!(result.get_selected_block(), BlockPosition::new(0, 24, 0));
    }
}
