use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use godot::engine::{
    global::Key, global::MouseButton, input::MouseMode, InputEvent, InputEventKey, InputEventMouseButton,
    InputEventMouseMotion,
};
use godot::prelude::*;
use rapier3d::prelude::*;

use crate::console::console_handler::Console;
use crate::main_scene::FloatType;
use crate::world::godot_world::World;
use crate::world::physics_handler::{PhysicsContainer, PhysicsEntity};

use super::body_controller::BodyController;
use super::{input_data::InputData, player_movement::PlayerMovement};

pub(crate) const ACCELERATION: f32 = 4.0;
pub(crate) const BOST_MULTIPLIER: f32 = 1.5;
pub(crate) const SENSITIVITY: f32 = 0.3;

pub const CONTROLLER_HEIGHT: f32 = 1.8;
const CAMERA_VERTICAL_OFFSET: f32 = 1.7;

pub const CONTROLLER_RADIUS: f32 = 0.4;
pub const CONTROLLER_MASS: f32 = 4.0;
const JUMP_IMPULSE: f32 = 20.0;

const THIRD_PERSON_OFFST: Vector3 = Vector3::new(0.4, 0.0, 3.0);

pub enum ContollerViewMode {
    FirstPersonView,
    ThirdPersonView,
}

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct PlayerController {
    #[base]
    pub(crate) base: Base<Node3D>,

    view_mode: ContollerViewMode,

    camera_anchor: Gd<Node3D>,
    camera: Gd<Camera3D>,

    input_data: InputData,
    cache_movement: Option<PlayerMovement>,

    // A full-length body
    body_controller: Gd<BodyController>,

    physics_entity: PhysicsEntity,
}

impl PlayerController {
    pub fn create(base: Base<Node3D>, physics_container: &PhysicsContainer) -> Self {
        let mut camera_anchor = Node3D::new_alloc();
        camera_anchor.set_position(Vector3::new(0.0, CAMERA_VERTICAL_OFFSET, 0.0));

        let camera = Camera3D::new_alloc();
        camera_anchor.add_child(camera.clone().upcast());

        let body_controller = Gd::<BodyController>::with_base(|base| BodyController::create(base));

        Self {
            base,
            view_mode: ContollerViewMode::FirstPersonView,
            camera,
            camera_anchor,
            input_data: Default::default(),
            cache_movement: None,
            physics_entity: physics_container.create_controller(),
            body_controller,
        }
    }

    // Get position of the controller
    pub fn get_position(&self) -> Vector3 {
        self.base.get_position()
    }

    /// Horizontal angle
    pub fn get_yaw(&self) -> f32 {
        self.camera_anchor.get_rotation().x
    }

    /// Vertical angle
    pub fn get_pitch(&self) -> f32 {
        self.camera_anchor.get_rotation().y
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.base.set_position(position);

        // The center of the physical collider at his center
        // So it shifts to half the height
        let physics_pos = vector![position.x, position.y + CONTROLLER_HEIGHT / 2.0, position.z];
        self.physics_entity.set_position(physics_pos);
    }

    pub fn set_rotation(&mut self, yaw: FloatType, pitch: FloatType) {
        self.camera_anchor.rotate_y(yaw);
        self.camera_anchor
            .rotate_object_local(Vector3::new(1.0, 0.0, 0.0), pitch as f32);

        // Rotate visible third_person body
        self.body_controller.rotate_y(yaw);
    }

    pub fn set_view_mode(&mut self, view_mode: ContollerViewMode) {
        self.view_mode = view_mode;
        match self.view_mode {
            ContollerViewMode::FirstPersonView => {
                self.body_controller.set_visible(false);
                self.camera.set_position(Vector3::ZERO);
            }
            ContollerViewMode::ThirdPersonView => {
                self.body_controller.set_visible(true);
                self.camera.set_position(THIRD_PERSON_OFFST);
            }
        }
    }

    pub fn get_view_mode(&self) -> &ContollerViewMode {
        &self.view_mode
    }
}

#[godot_api]
impl PlayerController {
    #[signal]
    fn on_player_move();
}

#[godot_api]
impl NodeVirtual for PlayerController {
    fn init(base: Base<Node3D>) -> Self {
        let physics = PhysicsContainer::default();
        Self::create(base, &physics)
    }

    fn ready(&mut self) {
        self.base.add_child(self.body_controller.clone().upcast());
        self.base.add_child(self.camera_anchor.clone().upcast());
        self.set_view_mode(ContollerViewMode::FirstPersonView);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if Console::is_active() {
            return;
        }

        if let Some(e) = event.clone().try_cast::<InputEventMouseMotion>() {
            self.input_data.mouse_position = e.get_relative();
        }

        if let Some(e) = event.clone().try_cast::<InputEventMouseButton>() {
            if e.get_button_index() == MouseButton::MOUSE_BUTTON_RIGHT {
                let mouse_mode = match e.is_pressed() {
                    true => MouseMode::MOUSE_MODE_CAPTURED,
                    false => MouseMode::MOUSE_MODE_VISIBLE,
                };
                Input::singleton().set_mouse_mode(mouse_mode);
            }
        }

        if let Some(e) = event.try_cast::<InputEventKey>() {
            match e.get_keycode() {
                Key::KEY_D => {
                    self.input_data.right = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_A => {
                    self.input_data.left = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_W => {
                    self.input_data.forward = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_S => {
                    self.input_data.back = e.is_pressed() as i32 as FloatType;
                }
                Key::KEY_SHIFT => {
                    self.input_data.multiplier = e.is_pressed();
                }
                Key::KEY_SPACE => {
                    self.input_data.space = e.is_pressed();
                }
                _ => (),
            };
        }
    }

    fn process(&mut self, delta: f64) {
        let now = std::time::Instant::now();

        let world = self.base.get_parent().unwrap().cast::<World>();
        let pos = self.get_position();
        let chunk_pos = BlockPosition::new(pos.x as i64, pos.y as i64, pos.z as i64).get_chunk_position();
        let chunk_loaded = match world.bind().get_chunk(&chunk_pos) {
            Some(c) => c.borrow().is_loaded(),
            None => false,
        };

        let console_active = Console::is_active();

        if !console_active {
            // Rotate camera look at
            if Input::singleton().get_mouse_mode() == MouseMode::MOUSE_MODE_CAPTURED {
                let (yaw, pitch) = self.input_data.get_mouselook_vector();
                self.set_rotation(yaw as f32, pitch as f32);
            }
        }

        let pitch = self.get_pitch();
        {
            // Set lock if chunk is in loading
            self.physics_entity.set_enabled(chunk_loaded);

            if !console_active {
                // Moving
                let vec = self.input_data.get_movement_vector(delta);
                let vec = vec.rotated(Vector3::new(0.0, 1.0, 0.0), pitch as f32);
                if vec != Vector3::ZERO {
                    self.physics_entity
                        .controller_move(delta, Vector::new(vec.x, vec.y, vec.z));
                }

                // Jump
                let input = Input::singleton();
                if input.is_action_just_pressed("jump".into()) {
                    self.physics_entity.apply_impulse(Vector::new(0.0, JUMP_IMPULSE, 0.0));
                }
            }

            // Sync godot object position
            let physics_pos = self.physics_entity.get_position();
            // Controller position is lowered by half of the center of mass position
            self.base.set_position(Vector3::new(
                physics_pos.x,
                physics_pos.y - CONTROLLER_HEIGHT / 2.0,
                physics_pos.z,
            ));
        }

        // Handle player movement
        let new_movement = PlayerMovement::create(self.get_position(), self.get_yaw(), self.get_pitch());
        if self.cache_movement.is_none() || new_movement != self.cache_movement.unwrap() {
            self.base
                .emit_signal("on_player_move".into(), &[new_movement.to_variant()]);
            self.cache_movement = Some(new_movement);
        }

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(3) {
            println!("PlayerController process: {:.2?}", elapsed);
        }
    }
}
