use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use godot::engine::{
    global::Key, global::MouseButton, input::MouseMode, InputEvent, InputEventKey, InputEventMouseButton,
    InputEventMouseMotion,
};
use godot::engine::{CapsuleMesh, MeshInstance3D};
use godot::prelude::*;
use rapier3d::prelude::RigidBodyType;

use crate::console::console_handler::Console;
use crate::main_scene::FloatType;
use crate::world::godot_world::World;
use crate::world::physics_handler::{PhysicsContainer, PhysicsEntity};

use super::{input_data::InputData, player_movement::PlayerMovement};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct PlayerController {
    #[base]
    pub(crate) base: Base<Node3D>,
    camera: Gd<Camera3D>,
    input_data: InputData,
    cache_movement: Option<PlayerMovement>,

    body: Gd<MeshInstance3D>,
    physics_entity: PhysicsEntity,
}

impl PlayerController {
    pub fn create(base: Base<Node3D>, physics_container: &mut PhysicsContainer) -> Self {
        let camera = load::<PackedScene>("res://scenes/camera_3d.tscn").instantiate_as::<Camera3D>();

        let mut body = MeshInstance3D::new_alloc();
        let mut mesh = CapsuleMesh::new();
        mesh.set_height(2.0);
        mesh.set_radius(0.5);
        body.set_mesh(mesh.upcast());
        body.set_position(Vector3 {
            x: 0.0,
            y: 60.0,
            z: 0.0,
        });

        Self {
            base,
            camera: camera,
            input_data: Default::default(),
            cache_movement: None,
            physics_entity: physics_container.create_capsule(&body.get_position(), 0.25, 0.5),
            body,
        }
    }

    /// Handle network packet for changing position
    pub fn teleport(&mut self, position: Vector3, yaw: FloatType, pitch: FloatType) {
        self.base.set_position(position);
        self.camera.rotate_y(yaw);
        self.camera
            .rotate_object_local(Vector3::new(1.0, 0.0, 0.0), pitch as f32);
    }

    // Get position of the controller
    pub fn get_position(&self) -> Vector3 {
        self.base.get_position()
    }

    pub fn get_yaw(&self) -> f32 {
        self.camera.get_rotation().x
    }

    pub fn get_pitch(&self) -> f32 {
        self.camera.get_rotation().y
    }

    pub fn get_camera_position(&self) -> Vector3 {
        self.camera.get_position()
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
        let mut physics = PhysicsContainer::default();
        Self::create(base, &mut physics)
    }

    fn ready(&mut self) {
        self.body.set_position(Vector3::new(0.0, 1.0, 0.0));
        self.base.add_child(self.body.share().upcast());

        self.camera.set_position(Vector3::new(0.0, 1.5, 0.0));
        self.base.add_child(self.camera.share().upcast());
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if Console::is_active() {
            return;
        }

        if let Some(e) = event.share().try_cast::<InputEventMouseMotion>() {
            self.input_data.mouse_position = e.get_relative();
        }

        if let Some(e) = event.share().try_cast::<InputEventMouseButton>() {
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
                _ => (),
            };
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        let now = std::time::Instant::now();

        if Console::is_active() {
            return;
        }

        // Rotate camera look at
        if Input::singleton().get_mouse_mode() == MouseMode::MOUSE_MODE_CAPTURED {
            let (yaw, pitch) = self.input_data.get_mouselook_vector();
            self.camera.rotate_y(yaw as f32);
            self.camera
                .rotate_object_local(Vector3::new(1.0, 0.0, 0.0), pitch as f32);
        }

        // self.camera.translate(self.input_data.get_movement_vector(delta));

        let world = self.base.get_parent().unwrap().cast::<World>();
        let pos = self.get_position();
        let chunk_pos = BlockPosition::new(pos.x as i64, pos.y as i64, pos.z as i64).get_chunk_position();
        let chunk_loaded = match world.bind().get_chunk(&chunk_pos) {
            Some(c) => c.borrow().is_loaded(),
            None => false,
        };

        let mut body = self.physics_entity.get_rigid_body_mut();
        let body = body.as_mut().unwrap();

        if chunk_loaded && !body.is_dynamic() {
            body.set_body_type(RigidBodyType::Dynamic);
        }
        else if !chunk_loaded && !body.is_fixed() {
            body.set_body_type(RigidBodyType::Fixed);
        }

        self.base.set_position(PhysicsEntity::transform_to_vector3(&body.translation()));
        self.base.set_rotation(PhysicsEntity::rotation_to_vector3(&body.rotation()));

        // Handle player movement
        let new_movement = PlayerMovement::create(
            self.base.get_position(),
            self.camera.get_rotation().y,
            self.camera.get_rotation().x,
        );
        if self.cache_movement.is_none() || new_movement != self.cache_movement.unwrap() {
            self.base
                .emit_signal("on_player_move".into(), &[new_movement.to_variant()]);
            self.cache_movement = Some(new_movement);
        }

        //body.apply_impulse(Vector::new(-0.5, 10.0, 0.0), true);

        let elapsed = now.elapsed();
        if elapsed > std::time::Duration::from_millis(3) {
            println!("PlayerController process: {:.2?}", elapsed);
        }
    }
}
