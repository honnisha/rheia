use super::collider::PhysxPhysicsCollider;
use crate::physics::physics::IQueryFilter;
use physx::traits::Class;
use physx_sys::{
    create_raycast_filter_callback, PxQueryFilterCallback, PxQueryFilterCallback_delete, PxQueryFilterData,
    PxQueryFilterData_new,
};
use std::ptr::drop_in_place;

pub struct PhysxQueryFilter {
    pub(crate) filter: PxQueryFilterData,
    pre_filter_callback: Option<*mut PxQueryFilterCallback>,
}

impl Default for PhysxQueryFilter {
    fn default() -> Self {
        Self {
            filter: unsafe { PxQueryFilterData_new() },
            pre_filter_callback: Default::default(),
        }
    }
}

impl IQueryFilter<PhysxPhysicsCollider> for PhysxQueryFilter {
    fn exclude_exclude_collider(&mut self, collider: &PhysxPhysicsCollider) {
        self.pre_filter_callback = Some(unsafe { create_raycast_filter_callback(collider.actor.as_ptr()) });
    }
}

impl Drop for PhysxQueryFilter {
    fn drop(&mut self) {
        if let Some(ptr) = self.pre_filter_callback.take() {
            unsafe { PxQueryFilterCallback_delete(ptr) };
            unsafe {
                drop_in_place(ptr);
            }
        }
    }
}
