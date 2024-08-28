use super::bridge::IntoNaVector3;
use super::container::RapierPhysicsContainer;
use crate::network::messages::Vector3 as NetworkVector3;
use crate::physics::physics::IPhysicsCollider;
use rapier3d::prelude::*;

/// For stationary bodies
#[derive(Clone)]
pub struct RapierPhysicsCollider {
    pub physics_container: RapierPhysicsContainer,
    pub collider_handle: ColliderHandle,
}

impl RapierPhysicsCollider {
    pub fn create(physics_container: &RapierPhysicsContainer, collider_handle: ColliderHandle) -> Self {
        Self {
            physics_container: physics_container.clone(),
            collider_handle,
        }
    }
}

impl IPhysicsCollider for RapierPhysicsCollider {
    fn set_position(&mut self, position: NetworkVector3) {
        let mut collider = self.physics_container.get_collider_mut(&self.collider_handle).unwrap();
        collider.set_translation(position.to_na());
    }

    fn set_enabled(&mut self, active: bool) {
        let mut collider = self
            .physics_container
            .get_collider_mut(&self.collider_handle)
            .expect("physics entity dosesn't have collider_handle");
        collider.set_enabled(active);
    }

    fn get_index(&self) -> usize {
        self.collider_handle.into_raw_parts().0 as usize
    }

    fn remove(&self) {
        self.physics_container.collider_set.write().remove(
            self.collider_handle,
            &mut self.physics_container.island_manager.write(),
            &mut self.physics_container.rigid_body_set.write(),
            true,
        );
    }
}
