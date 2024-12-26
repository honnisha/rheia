use common::chunks::position::Vector3;
use rapier3d::prelude::*;

use crate::physics::IPhysicsColliderBuilder;

use super::collider::RapierPhysicsShape;

pub struct RapierPhysicsColliderBuilder {
    pub(crate) builder: ColliderBuilder,
}

impl IPhysicsColliderBuilder<RapierPhysicsShape> for RapierPhysicsColliderBuilder {
    fn cuboid(hx: f32, hy: f32, hz: f32) -> Self {
        Self {
            builder: ColliderBuilder::cuboid(hx * 0.5, hy * 0.5, hz * 0.5),
        }
    }

    fn cylinder(half_height: f32, radius: f32) -> Self {
        Self {
            builder: ColliderBuilder::cylinder(half_height, radius),
        }
    }

    fn trimesh(verts: Vec<Vector3>, indices: Vec<[u32; 3]>) -> Self {
        let verts = verts.into_iter().map(|v| Point::new(v.x, v.y, v.z)).collect();
        Self {
            builder: ColliderBuilder::trimesh(verts, indices),
        }
    }

    fn get_shape(&self) -> RapierPhysicsShape {
        RapierPhysicsShape::create(self.builder.shape.as_ref())
    }
}
