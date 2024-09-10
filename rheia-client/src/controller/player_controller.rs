use crate::entities::entity::Entity;
use crate::entities::enums::generic_animations::GenericAnimations;
use crate::utils::bridge::{IntoChunkPositionVector, IntoGodotVector, IntoNetworkVector};
use crate::world::physics::{PhysicsProxy, PhysicsType};
use crate::world::world_manager::WorldManager;
use common::blocks::block_info::BlockInfo;
use common::blocks::blocks_storage::BlockType;
use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use common::network::messages::{ClientMessages, NetworkMessageType, Rotation};
use common::physics::physics::{IPhysicsCharacterController, IPhysicsColliderBuilder, IPhysicsRigidBody, IQueryFilter};
use common::physics::{
    PhysicsCharacterController, PhysicsCollider, PhysicsColliderBuilder, PhysicsRigidBody, QueryFilter,
};
use godot::global::{deg_to_rad, lerp_angle};
use godot::prelude::*;

use super::camera_controller::CameraController;
use super::controls::Controls;
use super::entity_movement::EntityMovement;

const TURN_SPEED: f64 = 6.0;
const MOVEMENT_SPEED: f32 = 4.0;
const GROUND_TIMER: f32 = 0.5;

const CHARACTER_GRAVITY: f32 = -10.0;
const JUMP_SPEED: f32 = 10.0;

pub(crate) const CAMERA_DISTANCE: f32 = 3.5;
pub(crate) const CONTROLLER_CAMERA_OFFSET_RIGHT: f32 = 0.45;
pub(crate) const CONTROLLER_CAMERA_OFFSET_VERTICAL: f32 = CONTROLLER_HEIGHT * 0.9;

const CONTROLLER_HEIGHT: f32 = 1.8;
const CONTROLLER_RADIUS: f32 = 0.4;
const CONTROLLER_MASS: f32 = 3.0;

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct PlayerController {
    pub(crate) base: Base<Node3D>,

    entity: Gd<Entity>,

    camera_controller: Gd<CameraController>,

    controls: Gd<Controls>,
    cache_movement: Option<Gd<EntityMovement>>,

    // Physics
    rigid_body: PhysicsRigidBody,
    collider: PhysicsCollider,
    character_controller: PhysicsCharacterController,

    vertical_movement: f32,
    grounded_timer: f32,

    look_at_message: String,
}

impl PlayerController {
    pub fn create(base: Base<Node3D>, physics: &PhysicsProxy) -> Self {
        let controls = Controls::new_alloc();
        let mut camera_controller =
            Gd::<CameraController>::from_init_fn(|base| CameraController::create(base, controls.clone()));
        camera_controller.set_position(Vector3::new(0.0, CONTROLLER_CAMERA_OFFSET_VERTICAL, 0.0));

        let collider_builder = PhysicsColliderBuilder::cylinder(CONTROLLER_HEIGHT / 2.0, CONTROLLER_RADIUS);
        let (rigid_body, collider) = physics.spawn_rigid_body(collider_builder);

        Self {
            base,

            entity: Gd::<Entity>::from_init_fn(|base| Entity::create(base)),
            camera_controller,

            controls,
            cache_movement: None,

            rigid_body,
            character_controller: PhysicsCharacterController::create(Some(CONTROLLER_MASS)),
            collider,

            vertical_movement: 0.0,
            grounded_timer: 0.0,

            look_at_message: Default::default(),
        }
    }

    pub fn get_current_animation(&self) -> String {
        self.entity.bind().get_current_animation()
    }

    // Get position of the character
    pub fn get_position(&self) -> Vector3 {
        self.base().get_position()
    }

    /// Horizontal degrees of character look
    pub fn get_yaw(&self) -> f32 {
        self.entity.bind().get_yaw()
    }

    /// Vertical degrees of character look
    pub fn get_pitch(&self) -> f32 {
        self.entity.bind().get_pitch()
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.base_mut().set_position(position);

        // The center of the physical collider at his center
        // So it shifts to half the height
        let physics_pos = Vector3::new(position.x, position.y + CONTROLLER_HEIGHT / 2.0, position.z);
        self.rigid_body.set_position(physics_pos.to_network());
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.camera_controller.bind_mut().rotate(rotation);

        // Rotate visible third_person body
        self.entity.bind_mut().rotate(rotation);
    }

    fn get_movement(&mut self, delta: f64) -> Vector3 {
        let mut movement = Vector3::ZERO;

        let controls = self.controls.bind();
        let mut direction = *controls.get_movement_vector();

        // Get camera vertical rotation
        let camera_yaw = self.camera_controller.bind().get_yaw();

        // Rotate movement direction according to the camera
        direction = direction.rotated(Vector3::UP, deg_to_rad(camera_yaw as f64) as f32);

        if direction != Vector3::ZERO {
            let mut new_yaw = -direction.x.atan2(-direction.z) % 360.0;
            new_yaw = lerp_angle(self.entity.get_rotation().y as f64, new_yaw as f64, TURN_SPEED * delta) as f32;

            // Update skin rotation for visual display
            let mut skin_rotation = self.entity.bind().get_rotation();
            skin_rotation.y = new_yaw;
            self.entity.bind_mut().set_rotation(skin_rotation);

            movement = self.entity.bind().get_transform().basis.col_c() * -1.0 * MOVEMENT_SPEED;
        }

        // Check physics ground check
        if self.character_controller.is_grounded() {
            self.grounded_timer = GROUND_TIMER;
            self.vertical_movement = 0.0;
        }

        // If we are grounded we can jump
        if self.grounded_timer > 0.0 {
            self.grounded_timer -= delta as f32;
            // If we jump we clear the grounded tolerance
            if controls.is_jumping() {
                self.entity.bind_mut().trigger_animation(GenericAnimations::Jump);
                self.vertical_movement = JUMP_SPEED;
                self.grounded_timer = 0.0;
            }
        }

        movement.y = self.vertical_movement;
        let custom_mass = self.character_controller.get_custom_mass().unwrap_or(1.0);
        self.vertical_movement += CHARACTER_GRAVITY * delta as f32 * custom_mass;
        movement *= delta as f32;

        movement
    }

    pub fn update_vision(&mut self) {
        let camera_controller = self.camera_controller.bind();
        let camera = camera_controller.get_camera();

        let screen = camera.get_viewport().unwrap().get_visible_rect().size;

        let from = camera.project_ray_origin(screen / 2.0);
        let to = from + camera.project_ray_normal(screen / 2.0) * 10.0;

        let dir = to - from;
        let (dir, max_toi) = (dir.normalized(), dir.length());

        let mut filter = QueryFilter::default();
        filter.exclude_rigid_body(&self.rigid_body);

        let mut world = self.base().get_parent().unwrap().cast::<WorldManager>();
        let mut w = world.bind_mut();
        let Some((cast_result, physics_type)) = w.get_physics_mut().cast_ray_and_get_normal(dir, max_toi, from, filter)
        else {
            self.look_at_message = "-".to_string();
            return;
        };
        self.look_at_message = match physics_type {
            PhysicsType::ChunkMeshCollider(_chunk_position) => {
                let selected_block = cast_result.get_selected_block();
                if self.controls.bind().is_main_action() {
                    let msg = ClientMessages::EditBlockRequest {
                        world_slug: w.get_slug().clone(),
                        position: cast_result.get_place_block(),
                        new_block_info: BlockInfo::new(BlockType::Stone),
                    };
                    w.get_main()
                        .bind()
                        .network_send_message(&msg, NetworkMessageType::ReliableOrdered);
                    log::info!("cast_result.normal:{:?}", cast_result.normal);
                }
                if self.controls.bind().is_second_action() {
                    let msg = ClientMessages::EditBlockRequest {
                        world_slug: w.get_slug().clone(),
                        position: selected_block,
                        new_block_info: BlockInfo::new(BlockType::Air),
                    };
                    w.get_main()
                        .bind()
                        .network_send_message(&msg, NetworkMessageType::ReliableOrdered);
                }
                let block_selection = w.get_block_selection_mut();
                block_selection.set_visible(true);
                block_selection.set_position(selected_block.get_position().to_godot());
                block_selection.set_rotation_degrees(cast_result.normal.to_godot() * 180.0);
                format!("block:{selected_block:?}")
            }
            PhysicsType::EntityCollider(entity_id) => {
                format!("entity_id:{entity_id}")
            }
        };
    }

    pub fn get_look_at_message(&self) -> &String {
        &self.look_at_message
    }
}

#[godot_api]
impl PlayerController {
    #[signal]
    fn on_player_move();
}

#[godot_api]
impl INode3D for PlayerController {
    fn ready(&mut self) {
        let controller = self.entity.clone().upcast();
        self.base_mut().add_child(controller);

        let camera_controller = self.camera_controller.clone().upcast();
        self.base_mut().add_child(camera_controller);

        let controls = self.controls.clone().upcast();
        self.base_mut().add_child(controls);
    }

    fn process(&mut self, delta: f64) {
        let world = self.base().get_parent().unwrap().cast::<WorldManager>();
        let pos = self.get_position();
        let chunk_pos = BlockPosition::new(pos.x as i64, pos.y as i64, pos.z as i64).get_chunk_position();
        let chunk_loaded = match world.bind().get_chunk_map().get_chunk(&chunk_pos) {
            Some(c) => c.read().is_loaded(),
            None => false,
        };

        // Set lock if chunk is in loading
        self.rigid_body.set_enabled(chunk_loaded);

        if chunk_loaded {
            let movement = self.get_movement(delta);

            let mut filter = QueryFilter::default();
            filter.exclude_rigid_body(&self.rigid_body);

            let translation =
                self.character_controller
                    .move_shape(&self.collider, filter, delta, movement.to_network());
            self.rigid_body
                .set_position(self.rigid_body.get_position() + translation);

            self.update_vision();
        }

        // Sync godot object position
        let physics_pos = self.rigid_body.get_position();
        // Controller position is lowered by half of the center of mass position
        self.base_mut().set_position(Vector3::new(
            physics_pos.x,
            physics_pos.y - CONTROLLER_HEIGHT / 2.0,
            physics_pos.z,
        ));

        // Handle player movement
        let new_movement = Gd::<EntityMovement>::from_init_fn(|_base| {
            EntityMovement::create(self.get_position(), Rotation::new(self.get_yaw(), self.get_pitch()))
        });

        if self.cache_movement.is_none() || *new_movement.bind() != *self.cache_movement.as_ref().unwrap().bind() {
            let new_chunk = if let Some(old) = self.cache_movement.as_ref() {
                let c1 = old.bind().get_position().to_chunk_position();
                let c2 = new_movement.bind().get_position().to_chunk_position();
                c1 != c2
            } else {
                false
            };

            let movement = if let Some(old) = self.cache_movement.as_ref() {
                *new_movement.bind().get_position() - *old.bind().get_position()
            } else {
                Vector3::ZERO
            };
            self.entity.bind_mut().handle_movement(movement);

            self.base_mut().emit_signal(
                "on_player_move".into(),
                &[new_movement.to_variant(), new_chunk.to_variant()],
            );
            self.cache_movement = Some(new_movement);
        }
    }
}
