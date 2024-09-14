use crate::physics::IPhysicsCharacterController;

use super::bridge::{na_to_network, IntoNaVector3};
use super::collider::{RapierPhysicsCollider, RapierPhysicsShape};
use super::query_filter::RapierQueryFilter;
use common::chunks::position::Vector3;
use rapier3d::control::{CharacterCollision, CharacterLength, KinematicCharacterController};

pub struct RapierPhysicsCharacterController {
    character_controller: KinematicCharacterController,
    custom_mass: Option<f32>,
    grounded: bool,
}

impl<'a> IPhysicsCharacterController<RapierPhysicsShape, RapierPhysicsCollider, RapierQueryFilter<'a>>
    for RapierPhysicsCharacterController
{
    fn create(custom_mass: Option<f32>) -> Self {
        let mut character_controller = KinematicCharacterController::default();
        character_controller.offset = CharacterLength::Absolute(0.01);
        character_controller.snap_to_ground = Some(CharacterLength::Absolute(0.1));
        //character_controller.autostep = Some(CharacterAutostep {
        //    max_height: CharacterLength::Absolute(0.5),
        //    min_width: CharacterLength::Absolute(0.5),
        //    include_dynamic_bodies: false,
        //});
        Self {
            character_controller,
            custom_mass,
            grounded: false,
        }
    }

    fn move_shape(
        &mut self,
        collider: &RapierPhysicsCollider,
        filter: RapierQueryFilter,
        delta: f64,
        movement: Vector3,
    ) -> Vector3 {
        let physics_container = collider.physics_container.clone();
        let collider = physics_container
            .get_collider(&collider.collider_handle)
            .unwrap()
            .clone();

        let corrected_movement = self.character_controller.move_shape(
            delta as f32,
            &physics_container.rigid_body_set.read(),
            &physics_container.collider_set.read(),
            &physics_container.query_pipeline.read(),
            collider.shape(),
            collider.position(),
            movement.to_na(),
            filter.filter,
            |_| {},
        );
        self.grounded = corrected_movement.grounded;

        let _collisions: Vec<CharacterCollision> = vec![];
        if let Some(character_mass) = self.custom_mass {
            self.character_controller.solve_character_collision_impulses(
                delta as f32,
                &mut physics_container.rigid_body_set.write(),
                &physics_container.collider_set.read(),
                &physics_container.query_pipeline.read(),
                collider.shape(),
                character_mass,
                _collisions.iter(),
                filter.filter,
            );
        };

        na_to_network(&corrected_movement.translation)
    }

    fn is_grounded(&self) -> bool {
        self.grounded
    }

    fn get_custom_mass(&mut self) -> &Option<f32> {
        &self.custom_mass
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        physics::{IPhysicsCharacterController, IPhysicsColliderBuilder, IPhysicsContainer},
        rapier::{
            character_controller::RapierPhysicsCharacterController, collider_builder::RapierPhysicsColliderBuilder,
            container::RapierPhysicsContainer, query_filter::RapierQueryFilter,
        },
    };
    use common::chunks::position::Vector3;

    #[test]
    fn test_move_shape() {
        let physics = RapierPhysicsContainer::default();
        let collider_builder = RapierPhysicsColliderBuilder::cylinder(2.0, 1.0);
        let collider = physics.spawn_collider(collider_builder);

        let mut character_controller = RapierPhysicsCharacterController::create(Some(1.0));
        let filter = RapierQueryFilter::default();

        let result = character_controller.move_shape(&collider, filter, 0.5, Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(result, Vector3::new(0.0, 1.0, 0.0));
    }
}
