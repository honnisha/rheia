use rapier3d::prelude::QueryFilter;

use crate::physics::physics::IQueryFilter;

use super::collider::RapierPhysicsCollider;

#[derive(Default)]
pub struct RapierQueryFilter<'a> {
    pub(crate) filter: QueryFilter<'a>,
}

impl<'a> IQueryFilter<RapierPhysicsCollider> for RapierQueryFilter<'a> {
    fn exclude_exclude_collider(&mut self, collider: &RapierPhysicsCollider) {
        self.filter = self.filter.exclude_collider(collider.collider_handle.clone())
    }
}
