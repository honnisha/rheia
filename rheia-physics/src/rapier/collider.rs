use std::borrow::Borrow;

use super::bridge::{na_to_network, IntoNaVector3};
use super::collider_builder::RapierPhysicsColliderBuilder;
use super::container::RapierPhysicsContainer;
use crate::physics::{IPhysicsCollider, IPhysicsShape};
use common::chunks::position::Vector3 as NetworkVector3;
use rapier3d::prelude::*;

pub struct RapierPhysicsShape {
    shape: Box<dyn Shape>,
}

impl RapierPhysicsShape {
    pub(crate) fn create(shape: &dyn Shape) -> Self {
        Self {
            shape: shape.clone_dyn(),
        }
    }

    pub(crate) fn get_shape(&self) -> &dyn Shape {
        self.shape.borrow()
    }
}

impl IPhysicsShape for RapierPhysicsShape {}

/// For stationary bodies
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

impl IPhysicsCollider<RapierPhysicsShape> for RapierPhysicsCollider {
    fn get_position(&self) -> NetworkVector3 {
        let collider = self.physics_container.get_collider_mut(&self.collider_handle).unwrap();
        na_to_network(&collider.translation())
    }

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

    fn remove(&mut self) {
        self.physics_container.collider_set.write().remove(
            self.collider_handle,
            &mut self.physics_container.island_manager.write(),
            &mut self.physics_container.rigid_body_set.write(),
            true,
        );
    }

    fn get_shape(&self) -> RapierPhysicsShape {
        let physics_container = self.physics_container.clone();
        let collider = physics_container
            .get_collider(&self.collider_handle)
            .unwrap()
            .clone();
        RapierPhysicsShape::create(collider.shape())
    }

    fn set_shape(&mut self, shape: RapierPhysicsShape) {
        let mut collider = self.physics_container.get_collider_mut(&self.collider_handle).unwrap();
        let s = shape.get_shape().clone_dyn();
        let shape = SharedShape(s.into());
        collider.set_shape(shape);
    }
}
