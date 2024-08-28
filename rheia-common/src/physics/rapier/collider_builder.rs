use crate::network::messages::Vector3 as NetworkVector3;
use crate::physics::physics::IPhysicsColliderBuilder;
use rapier3d::prelude::*;

pub struct RapierPhysicsColliderBuilder {
    pub(crate) builder: ColliderBuilder,
}

impl IPhysicsColliderBuilder for RapierPhysicsColliderBuilder {
    fn cylinder(half_height: f32, radius: f32) -> Self {
        Self {
            builder: ColliderBuilder::cylinder(half_height, radius),
        }
    }

    fn trimesh(verts: Vec<NetworkVector3>, indices: Vec<[u32; 3]>) -> Self {
        let verts = verts.into_iter().map(|v| Point::new(v.x, v.y, v.z)).collect();
        Self {
            builder: ColliderBuilder::trimesh(verts, indices),
        }
    }
}
