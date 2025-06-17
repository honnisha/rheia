use rapier3d::prelude::QueryFilter;

use crate::physics::IQueryFilter;

use super::collider::{RapierPhysicsCollider, RapierPhysicsShape};

#[derive(Default)]
pub struct RapierQueryFilter<'a> {
    pub(crate) filter: QueryFilter<'a>,
}

impl<'a> IQueryFilter<RapierPhysicsShape, RapierPhysicsCollider> for RapierQueryFilter<'a> {
    fn exclude_collider(&mut self, collider: &RapierPhysicsCollider) {
        self.filter = self.filter.exclude_collider(collider.collider_handle.clone());
    }

    fn exclude_sensors(&mut self) {
        self.filter = self.filter.exclude_sensors();
    }
}
