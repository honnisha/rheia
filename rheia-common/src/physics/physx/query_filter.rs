use physx_sys::{PxQueryFilterData, PxQueryFilterData_new};

use crate::physics::physics::IQueryFilter;

use super::rigid_body::PhysxPhysicsRigidBody;

pub struct PhysxQueryFilter {
    pub(crate) filter: PxQueryFilterData,
}

impl Default for PhysxQueryFilter {
    fn default() -> Self {
        Self {
            filter: unsafe { PxQueryFilterData_new() },
        }
    }
}

impl IQueryFilter<PhysxPhysicsRigidBody> for PhysxQueryFilter {
    fn exclude_rigid_body(&mut self, _rigid_body: &PhysxPhysicsRigidBody) {
        todo!()
    }
}
