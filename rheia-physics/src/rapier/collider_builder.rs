use common::chunks::position::Vector3;
use rapier3d::prelude::*;

use crate::physics::IPhysicsColliderBuilder;

pub struct RapierPhysicsColliderBuilder {
    pub(crate) builder: ColliderBuilder,
}

impl IPhysicsColliderBuilder for RapierPhysicsColliderBuilder {
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
}