use crate::utils::position::GodotPositionConverter;
use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use common::network::messages::Vector3 as NetworkVector3;
use common::physics::physics::{PhysicsCharacterController, PhysicsContainer, PhysicsRigidBodyEntity};
use godot::prelude::*;
use utilities::{atan2, deg_to_rad, lerp, lerp_angle, lerpf};

use crate::console::console_handler::Console;
use crate::main_scene::{FloatType, PhysicsCharacterControllerType, PhysicsContainerType, PhysicsRigidBodyEntityType};
use crate::world::godot_world::World;

use super::body_controller::BodyController;
use super::camera_controller::CameraController;
use super::controls::Controls;
use super::enums::controller_actions::ControllerActions;
use super::player_movement::PlayerMovement;

pub const TURN_SPEED: f64 = 4.0;

pub(crate) const SPEED: f32 = 4.0;
pub(crate) const ACCELERATION: f32 = 4.0;
pub(crate) const SENSITIVITY: f32 = 0.3;

pub const CONTROLLER_HEIGHT: f32 = 1.8;

pub const CONTROLLER_RADIUS: f32 = 0.4;
pub const CONTROLLER_MASS: f32 = 4.0;
const JUMP_IMPULSE: f32 = 20.0;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct PlayerController {
    pub(crate) base: Base<Node3D>,

    // A full-length body; skin
    body_controller: Gd<BodyController>,

    camera_controller: Gd<CameraController>,

    controls: Gd<Controls>,
    cache_movement: Option<Gd<PlayerMovement>>,

    physics_entity: PhysicsRigidBodyEntityType,
    physics_controller: PhysicsCharacterControllerType,
}

impl PlayerController {
    pub fn create(base: Base<Node3D>, physics_container: &PhysicsContainerType) -> Self {
        let controls = Controls::new_alloc();
        Self {
            base,

            body_controller: Gd::<BodyController>::from_init_fn(|base| BodyController::create(base)),
            camera_controller: Gd::<CameraController>::from_init_fn(|base| {
                CameraController::create(base, controls.clone())
            }),

            controls,
            cache_movement: None,

            physics_entity: physics_container.create_rigid_body(CONTROLLER_HEIGHT, CONTROLLER_RADIUS, CONTROLLER_MASS),
            physics_controller: PhysicsCharacterControllerType::create(),
        }
    }

    // Get position of the character
    pub fn get_position(&self) -> Vector3 {
        self.body_controller.get_position()
    }

    /// Horizontal angle of character look
    pub fn get_yaw(&self) -> f32 {
        self.body_controller.get_rotation().x
    }

    /// Vertical angle of character look
    pub fn get_pitch(&self) -> f32 {
        self.body_controller.get_rotation().y
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.base_mut().set_position(position);

        // The center of the physical collider at his center
        // So it shifts to half the height
        let physics_pos = Vector3::new(position.x, position.y + CONTROLLER_HEIGHT / 2.0, position.z);
        self.physics_entity
            .set_position(GodotPositionConverter::vector_network_from_gd(&physics_pos));
    }

    pub fn set_rotation(&mut self, yaw: FloatType, pitch: FloatType) {
        self.camera_controller.rotate_y(yaw);
        self.camera_controller
            .rotate_object_local(Vector3::new(1.0, 0.0, 0.0), pitch as f32);

        // Rotate visible third_person body
        self.body_controller.rotate_y(yaw);
    }

    fn process_physics(&mut self, delta: f64) {
        let yaw = self.get_yaw();

        {
            let mut controls = self.controls.bind_mut();

            // Moving
            let mut direction = controls.get_movement_vector();

            // make the player move towards where the camera is facing by lerping the current movement rotation
            // towards the camera's horizontal rotation and rotating the raw movement direction with that angle
            controls.move_rot = lerpf(
                controls.move_rot as f64,
                deg_to_rad(yaw as f64),
                (4.0 * delta) as f64,
            ) as f32;
            direction = direction.rotated(Vector3::UP, controls.move_rot as f32);

            // lerp the player's current horizontal velocity towards the horizontal velocity as determined by
            // the input direction and the given horizontal speed
            let horizontal_velocity = Vector3::from_variant(&lerp(
                controls.horizontal_velocity.to_variant(),
                (direction * SPEED).to_variant(),
                (ACCELERATION as f64 * delta).to_variant(),
            ));
            controls.horizontal_velocity = horizontal_velocity;

            // if the player has any amount of movement, lerp the player model's rotation towards the current
            // movement direction based checked its angle towards the X+ axis checked the XZ plane
            if direction != Vector3::ZERO {
                let new_skin_y = lerp_angle(
                    self.body_controller.get_rotation().y as f64,
                    atan2(-direction.x as f64, -direction.z as f64),
                    TURN_SPEED * delta,
                );
                self.body_controller.rotate_y(new_skin_y as f32);

                self.physics_controller.controller_move(
                    &mut self.physics_entity,
                    delta,
                    GodotPositionConverter::vector_network_from_gd(&direction),
                );
            }

            if controls.is_jumping() {
                self.physics_entity
                    .apply_impulse(NetworkVector3::new(0.0, JUMP_IMPULSE, 0.0));
            }
        }

        // Sync godot object position
        let physics_pos = self.physics_entity.get_position();
        // Controller position is lowered by half of the center of mass position
        self.base_mut().set_position(Vector3::new(
            physics_pos.x,
            physics_pos.y - CONTROLLER_HEIGHT / 2.0,
            physics_pos.z,
        ));
    }
}

#[godot_api]
impl PlayerController {
    #[signal]
    fn on_player_move();
}

#[godot_api]
impl INode3D for PlayerController {
    fn init(base: Base<Node3D>) -> Self {
        let physics = PhysicsContainerType::create();
        Self::create(base, &physics)
    }

    fn ready(&mut self) {
        let controller = self.body_controller.clone().upcast();
        self.base_mut().add_child(controller);

        let camera_controller = self.camera_controller.clone().upcast();
        self.base_mut().add_child(camera_controller);
    }

    fn process(&mut self, delta: f64) {
        let world = self.base().get_parent().unwrap().cast::<World>();
        let pos = self.get_position();
        let chunk_pos = BlockPosition::new(pos.x as i64, pos.y as i64, pos.z as i64).get_chunk_position();
        let chunk_loaded = match world.bind().get_chunk(&chunk_pos) {
            Some(c) => c.borrow().is_loaded(),
            None => false,
        };

        // Set lock if chunk is in loading
        self.physics_entity.set_enabled(chunk_loaded);

        self.process_physics(delta);

        let input = Input::singleton();
        if input.is_action_just_pressed(ControllerActions::ActionMain.as_str().into()) {
            let camera_controller = self.camera_controller.bind();
            let camera = camera_controller.get_camera();

            let screen = camera.get_viewport().unwrap().get_visible_rect().size;

            let from = camera.project_ray_origin(screen / 2.0);
            let to = from + camera.project_ray_normal(screen / 2.0) * 10.0;

            let dir = to - from;
            let (dir, max_toi) = (dir.normalized(), dir.length());

            if let Some((collider_handle, hit_point)) = self.physics_entity.raycast(
                GodotPositionConverter::vector_network_from_gd(&dir),
                max_toi,
                GodotPositionConverter::vector_network_from_gd(&from),
            ) {
                log::debug!(target: "player", "Collider {:?} hit at point {}", collider_handle, hit_point);
            }
        }

        // Handle player movement
        let new_movement = Gd::<PlayerMovement>::from_init_fn(|base| {
            PlayerMovement::create(self.get_position(), self.get_yaw(), self.get_pitch())
        });

        if self.cache_movement.is_none() || *new_movement.bind() != *self.cache_movement.as_ref().unwrap().bind() {
            self.base_mut()
                .emit_signal("on_player_move".into(), &[new_movement.to_variant()]);
            self.cache_movement = Some(new_movement);
        }
    }
}
