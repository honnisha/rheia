use common::chunks::{block_position::BlockPosition, position::Vector3};

pub trait IQueryFilter<S: IPhysicsShape, C: IPhysicsCollider<S>>: Default {
    fn exclude_collider(&mut self, collider: &C);
}

pub trait IPhysicsShape {}

pub trait IPhysicsCollider<S: IPhysicsShape> {
    fn get_position(&self) -> Vector3;
    fn set_position(&mut self, position: Vector3);
    fn set_enabled(&mut self, active: bool);
    fn get_index(&self) -> usize;
    fn remove(&mut self);
    fn get_shape(&self) -> S;
}

pub trait IPhysicsCharacterController<S: IPhysicsShape, C: IPhysicsCollider<S>, F: IQueryFilter<S, C>> {
    fn create(custom_mass: Option<f32>, snap_to_ground: Option<f32>) -> Self;
    fn move_shape(&mut self, collider: &C, filter: F, delta: f64, impulse: Vector3) -> Vector3;
    fn get_custom_mass(&mut self) -> &Option<f32>;
}

pub trait IPhysicsColliderBuilder {
    fn cuboid(hx: f32, hy: f32, hz: f32) -> Self;
    fn cylinder(half_height: f32, radius: f32) -> Self;
    fn trimesh(verts: Vec<Vector3>, indices: Vec<[u32; 3]>) -> Self;
}

pub trait IPhysicsContainer<S: IPhysicsShape, C: IPhysicsCollider<S>, B: IPhysicsColliderBuilder, F: IQueryFilter<S, C>>:
    Clone + Default
{
    fn step(&self, delta: f32);
    fn spawn_collider(&self, collider_builder: B) -> C;

    fn cast_ray(&self, origin: Vector3, dir: Vector3, max_toi: f32, filter: F) -> Option<RayCastResultNormal>;
    fn cast_shape(&self, shape: S, origin: Vector3, dir: Vector3, max_toi: f32, filter: F) -> Option<ShapeCastResult>;
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

pub struct ShapeCastResult {
    pub collider_id: usize,
    pub point: Vector3,
}

#[cfg(test)]
mod tests {
    use super::RayCastResultNormal;
    use common::chunks::{block_position::BlockPosition, position::Vector3};

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
