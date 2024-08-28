pub mod physics;

#[cfg(feature = "physics-rapier")]
pub mod rapier;

#[cfg(feature = "physics-rapier")]
pub type PhysicsRigidBody = rapier::rigid_body::RapierPhysicsRigidBody;

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
