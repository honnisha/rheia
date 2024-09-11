use physx::{cooking::PxTriangleMeshDesc, math::PxVec3};
use physx_sys::{PxCapsuleGeometry, PxCapsuleGeometry_new};
use std::ffi::c_void;

use crate::{network::messages::Vector3 as NetworkVector3, physics::physics::IPhysicsColliderBuilder};

pub enum GeometryInner {
    Capsule(PxCapsuleGeometry),
    TriangleMesh(PxTriangleMeshDesc),
}

pub struct PhysxPhysicsColliderBuilder {
    geometry: GeometryInner,
}

impl IPhysicsColliderBuilder for PhysxPhysicsColliderBuilder {
    fn cylinder(half_height: f32, radius: f32) -> Self {
        Self {
            geometry: GeometryInner::Capsule(unsafe { PxCapsuleGeometry_new(radius, half_height) }),
        }
    }

    fn trimesh(verts: Vec<NetworkVector3>, indices: Vec<[u32; 3]>) -> Self {
        let verts: Vec<PxVec3> = verts.into_iter().map(|v| PxVec3::new(v.x, v.y, v.z)).collect();

        let mut desc = PxTriangleMeshDesc::new();
        desc.obj.points.count = verts.len() as u32;
        desc.obj.points.stride = std::mem::size_of::<PxVec3>() as u32;
        desc.obj.points.data = verts.as_ptr() as *const c_void;

        desc.obj.triangles.count = indices.len() as u32;
        desc.obj.triangles.stride = std::mem::size_of::<[u32; 3]>() as u32;
        desc.obj.triangles.data = indices.as_ptr() as *const c_void;

        Self {
            geometry: GeometryInner::TriangleMesh(desc),
        }
    }
}
