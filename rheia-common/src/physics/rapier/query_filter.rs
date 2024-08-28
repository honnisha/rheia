use rapier3d::prelude::QueryFilter;

use crate::physics::physics::IQueryFilter;

use super::rigid_body::RapierPhysicsRigidBody;

#[derive(Default)]
pub struct RapierQueryFilter<'a> {
    pub(crate) filter: QueryFilter<'a>,
}

impl<'a> IQueryFilter<RapierPhysicsRigidBody> for RapierQueryFilter<'a> {
    fn exclude_rigid_body(&mut self, rigid_body: &RapierPhysicsRigidBody) {
        self.filter = self.filter.exclude_rigid_body(rigid_body.rigid_handle.clone());
    }
}
