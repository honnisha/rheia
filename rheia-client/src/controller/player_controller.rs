use crate::utils::bridge::IntoNetworkVector;
use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use common::network::messages::Vector3 as NetworkVector3;
use common::physics::physics::{PhysicsCharacterController, PhysicsContainer, PhysicsRigidBodyEntity};
use godot::global::{deg_to_rad, lerp, lerp_angle, lerpf};
use godot::prelude::*;

use crate::main_scene::{FloatType, PhysicsCharacterControllerType, PhysicsContainerType, PhysicsRigidBodyEntityType};
use crate::world::godot_world::World;

use super::body_controller::BodyController;
use super::camera_controller::CameraController;
use super::controls::Controls;
use super::player_movement::PlayerMovement;

pub const TURN_SPEED: f64 = 4.0;

const SPEED: f32 = 0.1;
const ACCELERATION: f64 = 10.0;

pub(crate) const CAMERA_DISTANCE: f32 = 3.5;

const CONTROLLER_HEIGHT: f32 = 1.8;

const CONTROLLER_RADIUS: f32 = 0.4;
const CONTROLLER_MASS: f32 = 1.0;
const JUMP_IMPULSE: f32 = 5.0;

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

    pub move_rot: FloatType,
    pub horizontal_velocity: Vector3,
}

impl PlayerController {
    pub fn create(base: Base<Node3D>, physics_container: &PhysicsContainerType) -> Self {
        let controls = Controls::new_alloc();
        let mut camera_controller =
            Gd::<CameraController>::from_init_fn(|base| CameraController::create(base, controls.clone()));
        camera_controller.set_position(Vector3::new(0.0, CONTROLLER_HEIGHT * 0.75, 0.0));
        Self {
            base,

            body_controller: Gd::<BodyController>::from_init_fn(|base| BodyController::create(base)),
            camera_controller,

            controls,
            cache_movement: None,

            physics_entity: physics_container.create_rigid_body(CONTROLLER_HEIGHT, CONTROLLER_RADIUS, CONTROLLER_MASS),
            physics_controller: PhysicsCharacterControllerType::create(),

            move_rot: Default::default(),
            horizontal_velocity: Default::default(),
        }
    }

    // Get position of the character
    pub fn get_position(&self) -> Vector3 {
        self.base().get_position()
    }

    /// Horizontal degrees of character look
    pub fn get_yaw(&self) -> f32 {
        self.body_controller.get_rotation_degrees().y
    }

    /// Vertical degrees of character look
    pub fn get_pitch(&self) -> f32 {
        self.camera_controller.bind().get_pitch()
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.base_mut().set_position(position);

        // The center of the physical collider at his center
        // So it shifts to half the height
        let physics_pos = Vector3::new(position.x, position.y + CONTROLLER_HEIGHT / 2.0, position.z);
        self.physics_entity.set_position(physics_pos.to_network());
    }

    pub fn set_rotation(&mut self, yaw: FloatType, pitch: FloatType) {
        self.camera_controller.bind_mut().rotate(yaw, pitch);

        // Rotate visible third_person body
        self.body_controller.rotate_y(yaw);
    }

    fn apply_controls(&mut self, delta: f64) {

        let controls = self.controls.bind();
        let mut direction = *controls.get_movement_vector();

        // Get camera vertical rotation
        let camera_yaw = self.camera_controller.bind().get_yaw();

        // Rotate movement direction according to the camera
        direction = direction.rotated(Vector3::UP, deg_to_rad(camera_yaw as f64) as f32);

        if direction != Vector3::ZERO {

            let mut new_yaw = -direction.x.atan2(-direction.z) % 360.0;
            new_yaw = lerp_angle(self.body_controller.get_rotation().y as f64, new_yaw as f64, TURN_SPEED * delta) as f32;

            // Update skil rotation for visual display
            let mut skin_rotation = self.body_controller.get_rotation();
            skin_rotation.y = new_yaw;
            self.body_controller.set_rotation(skin_rotation);

            let force = self.body_controller.get_transform().basis.col_c() * -1.0 * SPEED;

            self.physics_controller.controller_move(
                &mut self.physics_entity,
                delta,
                force.to_network(),
            );
        }

        if controls.is_jumping() {
            self.physics_entity
                .apply_impulse(NetworkVector3::new(0.0, JUMP_IMPULSE, 0.0));
        }
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

        let controls = self.controls.clone().upcast();
        self.base_mut().add_child(controls);
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

        if chunk_loaded {
            self.apply_controls(delta);
        }

        // Sync godot object position
        let physics_pos = self.physics_entity.get_position();
        // Controller position is lowered by half of the center of mass position
        self.base_mut().set_position(Vector3::new(
            physics_pos.x,
            physics_pos.y - CONTROLLER_HEIGHT / 2.0,
            physics_pos.z,
        ));

        if self.controls.bind().is_main_action() {
            let camera_controller = self.camera_controller.bind();
            let camera = camera_controller.get_camera();

            let screen = camera.get_viewport().unwrap().get_visible_rect().size;

            let from = camera.project_ray_origin(screen / 2.0);
            let to = from + camera.project_ray_normal(screen / 2.0) * 10.0;

            let dir = to - from;
            let (dir, max_toi) = (dir.normalized(), dir.length());

            if let Some((collider_handle, hit_point)) =
                self.physics_entity
                    .raycast(dir.to_network(), max_toi, from.to_network())
            {
                log::debug!(target: "player", "Collider {:?} hit at point {}", collider_handle, hit_point);
            }
        }

        // Handle player movement
        let new_movement = Gd::<PlayerMovement>::from_init_fn(|_base| {
            PlayerMovement::create(self.get_position(), self.get_yaw(), self.get_pitch())
        });

        if self.cache_movement.is_none() || *new_movement.bind() != *self.cache_movement.as_ref().unwrap().bind() {
            self.base_mut()
                .emit_signal("on_player_move".into(), &[new_movement.to_variant()]);
            self.cache_movement = Some(new_movement);
        }
    }
}
