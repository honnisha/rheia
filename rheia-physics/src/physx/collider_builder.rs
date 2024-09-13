use super::controller::PhysxPhysicsController;
use crate::physics::IPhysicsColliderBuilder;
use common::chunks::position::Vector3;
use physx::cooking;
use physx::prelude::{Geometry, TriangleMeshGeometry};
use physx::{cooking::PxTriangleMeshDesc, math::PxVec3};
use physx_sys::{
    PxCapsuleGeometry, PxCapsuleGeometry_new, PxMeshGeometryFlags, PxMeshScale_new, PxTriangleMeshGeometry,
};
use std::ffi::c_void;

pub(crate) enum GeometryInner {
    Capsule(PxCapsuleGeometry),
    TriangleMesh(PxTriangleMeshDesc),
}

pub struct PhysxPhysicsColliderBuilder {
    geometry: GeometryInner,
}

impl PhysxPhysicsColliderBuilder {
    pub fn get_geometry(&mut self, controller: &mut PhysxPhysicsController) -> Box<dyn Geometry> {
        match &self.geometry {
            GeometryInner::Capsule(g) => return Box::new(g.clone()),
            GeometryInner::TriangleMesh(desc) => {
                let params = cooking::PxCookingParams::new(controller.physics.physics()).unwrap();
                let mut mesh = match cooking::create_triangle_mesh(controller.physics.physics_mut(), &params, &desc) {
                    cooking::TriangleMeshCookingResult::Success(m) => m,
                    _ => panic!("create_convex_mesh error"),
                };
                let geometry = PxTriangleMeshGeometry::new(
                    &mut mesh,
                    &unsafe { PxMeshScale_new() },
                    PxMeshGeometryFlags::TightBounds,
                );
                return Box::new(geometry);
            }
        }
    }
}

impl IPhysicsColliderBuilder for PhysxPhysicsColliderBuilder {
    fn cylinder(half_height: f32, radius: f32) -> Self {
        Self {
            geometry: GeometryInner::Capsule(unsafe { PxCapsuleGeometry_new(radius, half_height) }),
        }
    }

    fn trimesh(verts: Vec<Vector3>, indices: Vec<[u32; 3]>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::PhysxPhysicsColliderBuilder;
    use crate::{
        physics::{IPhysicsCollider, IPhysicsColliderBuilder, IPhysicsContainer},
        physx::container::PhysxPhysicsContainer,
    };
    use common::chunks::position::Vector3;

    #[test]
    fn test_collider() {
        let verts = vec![
            Vector3::new(0., 1., 0.),
            Vector3::new(0., -1., 0.),
            Vector3::new(1., 0., 0.),
        ];
        let indices: Vec<[u32; 3]> = Default::default();
        let collider = PhysxPhysicsColliderBuilder::trimesh(verts, indices);

        let container = PhysxPhysicsContainer::default();
        let collider = container.spawn_collider(collider);
        assert_eq!(collider.get_index(), 0_usize);
        assert_eq!(collider.get_position(), Vector3::zero());
    }
}
