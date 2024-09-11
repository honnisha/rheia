pub mod physics;

#[cfg(feature = "physics-rapier")]
pub mod rapier;

#[cfg(feature = "physics-rapier")]
pub type PhysicsCollider = rapier::collider::RapierPhysicsCollider;

#[cfg(feature = "physics-rapier")]
pub type PhysicsColliderBuilder = rapier::collider_builder::RapierPhysicsColliderBuilder;

#[cfg(feature = "physics-rapier")]
pub type PhysicsCharacterController = rapier::character_controller::RapierPhysicsCharacterController;

#[cfg(feature = "physics-rapier")]
pub type PhysicsContainer = rapier::container::RapierPhysicsContainer;

#[cfg(feature = "physics-rapier")]
pub type QueryFilter<'a> = rapier::query_filter::RapierQueryFilter<'a>;


#[cfg(feature = "physics-physx")]
pub mod physx;

#[cfg(feature = "physics-physx")]
pub type PhysicsCollider = physx::collider::PhysxPhysicsCollider;

#[cfg(feature = "physics-physx")]
pub type PhysicsColliderBuilder = physx::collider_builder::PhysxPhysicsColliderBuilder;

#[cfg(feature = "physics-physx")]
pub type PhysicsCharacterController = physx::character_controller::PhysxPhysicsCharacterController;

#[cfg(feature = "physics-physx")]
pub type PhysicsContainer = physx::container::PhysxPhysicsContainer;

#[cfg(feature = "physics-physx")]
pub type QueryFilter<'a> = physx::query_filter::PhysxQueryFilter;
