use super::building_visualizer::BuildingVisualizer;
use super::camera_controller::{CameraController, RayDirection};
use super::controls::Controls;
use super::entity_movement::EntityMovement;
use super::enums::camera_mode::CameraMode;
use super::look_at::LookAt;
use super::player_action::{PlayerAction, PlayerActionType};
use super::selected_item::{SelectedItem, SelectedItemGd};
use crate::console::console_handler::Console;
use crate::entities::entity::Entity;
use crate::entities::enums::generic_animations::GenericAnimations;
use crate::scenes::components::block_icon::BlockIconSelect;
use crate::scenes::components::block_menu::BlockMenu;
use crate::utils::bridge::{IntoChunkPositionVector, IntoGodotVector, IntoNetworkVector};
use crate::world::physics::{PhysicsProxy, PhysicsType};
use crate::world::worlds_manager::WorldsManager;
use common::blocks::block_info::BlockFace;
use common::chunks::chunk_data::BlockDataInfo;
use common::chunks::rotation::Rotation;
use godot::classes::input::MouseMode;
use godot::classes::Input;
use godot::global::{deg_to_rad, lerp_angle};
use godot::prelude::*;
use network::entities::EntityNetworkComponent;
use network::messages::NetworkEntitySkin;
use physics::physics::{IPhysicsCharacterController, IPhysicsCollider, IPhysicsColliderBuilder, IQueryFilter};
use physics::{PhysicsCharacterController, PhysicsCollider, PhysicsColliderBuilder, QueryFilter};

const TURN_SPEED: f64 = 6.0;
const MOVEMENT_SPEED: f32 = 4.0;

const CHARACTER_GRAVITY: f32 = -10.0;
const JUMP_SPEED: f32 = 8.0;
const SNAP_TO_GROUND: f32 = 0.1;

pub(crate) const CAMERA_DISTANCE: f32 = 2.5;
pub(crate) const CONTROLLER_CAMERA_OFFSET_RIGHT: f32 = 0.45;
pub(crate) const CONTROLLER_CAMERA_OFFSET_VERTICAL: f32 = CONTROLLER_HEIGHT * 0.95;

const CONTROLLER_HEIGHT: f32 = 1.8;
const CONTROLLER_RADIUS: f32 = 0.4;
const CONTROLLER_MASS: f32 = 3.0;

#[derive(GodotClass)]
#[class(no_init, base=Node3D)]
pub struct PlayerController {
    pub(crate) base: Base<Node3D>,
    physics: PhysicsProxy,

    entity: Option<Gd<Entity>>,

    camera_controller: Gd<CameraController>,

    controls: Gd<Controls>,
    cache_movement: Option<Gd<EntityMovement>>,

    // Physics
    collider: PhysicsCollider,
    character_controller: PhysicsCharacterController,

    vertical_movement: f32,
    is_grounded: bool,
    grounded_timer: f32,

    look_at_message: String,

    building_visualizer: Gd<BuildingVisualizer>,

    selected_item: Option<SelectedItem>,

    block_menu: Gd<BlockMenu>,

    // To prevent actions after ui windows closed
    ui_lock: f32,

    window_focus: bool,

    camera_mode: CameraMode,
}

impl PlayerController {
    pub fn create(base: Base<Node3D>, physics: PhysicsProxy) -> Self {
        let controls = Controls::new_alloc();
        let mut camera_controller =
            Gd::<CameraController>::from_init_fn(|base| CameraController::create(base, controls.clone()));

        // Change vertical offset
        camera_controller.set_position(Vector3::new(0.0, CONTROLLER_CAMERA_OFFSET_VERTICAL, 0.0));

        let collider_builder = PhysicsColliderBuilder::cylinder(CONTROLLER_HEIGHT / 2.0, CONTROLLER_RADIUS);
        let collider = physics.clone().create_collider(collider_builder, None);

        let building_visualizer = Gd::<BuildingVisualizer>::from_init_fn(|base| BuildingVisualizer::create(base));

        let block_menu = Gd::<BlockMenu>::from_init_fn(|base| BlockMenu::create(base));

        Self {
            base,
            physics,

            entity: None,
            camera_controller,

            controls,
            cache_movement: None,

            character_controller: PhysicsCharacterController::create(Some(CONTROLLER_MASS), Some(SNAP_TO_GROUND)),
            collider,

            vertical_movement: 0.0,
            is_grounded: false,
            grounded_timer: 0.0,

            look_at_message: Default::default(),
            building_visualizer,

            selected_item: None,

            block_menu: block_menu,

            ui_lock: 0.0,

            window_focus: true,
            camera_mode: CameraMode::FirstPerson,
        }
    }

    pub fn set_selected_item(&mut self, new_item: Option<SelectedItem>) {
        self.selected_item = new_item.clone();
        self.building_visualizer.bind_mut().set_selected_item(new_item);
    }

    pub fn get_selected_item(&self) -> &Option<SelectedItem> {
        &self.selected_item
    }

    pub fn update_skin(&mut self, skin: Option<NetworkEntitySkin>) {
        match skin {
            Some(skin) => match self.entity.as_mut() {
                Some(e) => {
                    e.bind_mut().change_skin(skin);
                }
                None => {
                    let components = vec![EntityNetworkComponent::Skin(Some(skin))];
                    let mut entity = Gd::<Entity>::from_init_fn(|base| Entity::create(base, components));
                    self.base_mut().add_child(&entity);
                    let entity_visible = match self.camera_mode {
                        CameraMode::FirstPerson => false,
                        CameraMode::ThirdPerson => true,
                    };
                    entity.set_visible(entity_visible);
                    self.entity = Some(entity);
                }
            },
            None => {
                if let Some(mut e) = self.entity.take() {
                    e.queue_free();
                }
            }
        }
    }

    pub fn get_current_animation(&self) -> Option<String> {
        match self.entity.as_ref() {
            Some(entity) => Some(entity.bind().get_current_animation()),
            None => None,
        }
    }

    // Get position of the character
    pub fn get_position(&self) -> Vector3 {
        self.base().get_position()
    }

    /// Horizontal degrees of character look
    pub fn get_yaw(&self) -> f32 {
        match self.entity.as_ref() {
            Some(entity) => entity.bind().get_yaw(),
            None => self.camera_controller.bind().get_yaw(),
        }
    }

    /// Vertical degrees of character look
    pub fn get_pitch(&self) -> f32 {
        match self.entity.as_ref() {
            Some(entity) => entity.bind().get_pitch(),
            None => self.camera_controller.bind().get_pitch(),
        }
    }

    pub fn set_position(&mut self, position: Vector3) {
        self.base_mut().set_position(position);

        // The center of the physical collider at his center
        // So it shifts to half the height
        let physics_pos = Vector3::new(position.x, position.y + CONTROLLER_HEIGHT / 2.0, position.z);
        self.collider.set_position(physics_pos.to_network());
    }

    pub fn change_camera_mode(&mut self, camera_mode: CameraMode) {
        self.camera_mode = camera_mode;
        if let Some(entity) = self.entity.as_mut() {
            let entity_visible = match self.camera_mode {
                CameraMode::FirstPerson => false,
                CameraMode::ThirdPerson => true,
            };
            entity.set_visible(entity_visible);
        }
        match self.camera_mode {
            CameraMode::FirstPerson => self.camera_controller.bind_mut().set_camera_distance(0.0, 0.0),
            CameraMode::ThirdPerson => self
                .camera_controller
                .bind_mut()
                .set_camera_distance(CAMERA_DISTANCE, CONTROLLER_CAMERA_OFFSET_RIGHT),
        }
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.camera_controller.bind_mut().rotate(rotation);

        // Rotate visible third_person body
        if let Some(entity) = self.entity.as_mut() {
            entity.bind_mut().rotate(rotation);
        }
    }

    /// Grouded check performs by cast_shape
    ///
    /// grounded_timer counting the time:
    /// positive - how long grounded
    /// negative - how long not grounded
    fn detect_is_grounded(&mut self, delta: f64) {
        let ray_direction = RayDirection {
            dir: Vector3::new(0.0, -1.0, 0.0),
            from: self.collider.get_position().to_godot(),
            max_toi: SNAP_TO_GROUND,
        };
        let mut filter = QueryFilter::default();
        filter.exclude_collider(&self.collider);

        // Only solid ground
        filter.exclude_sensors();

        self.is_grounded = self
            .physics
            .cast_shape(self.collider.get_shape(), ray_direction, filter)
            .is_some();

        if self.is_grounded {
            self.grounded_timer = self.grounded_timer.max(0.0);
            self.grounded_timer += delta as f32;
        } else {
            self.grounded_timer = self.grounded_timer.min(0.0);
            self.grounded_timer -= delta as f32;
        }
    }

    fn get_movement(&mut self, delta: f64) -> Vector3 {
        let Some(entity) = self.entity.as_mut() else {
            panic!("get_movement available only with entity");
        };
        let controls = self.controls.bind();
        let mut direction = if Console::is_active() {
            Vector3::ZERO
        } else {
            *controls.get_movement_vector()
        };

        let mut movement = Vector3::ZERO;

        // Get camera vertical rotation
        let camera_yaw = self.camera_controller.bind().get_yaw();
        let camera_pitch = self.camera_controller.bind().get_pitch();

        // Привязано ли движение к фиксации по направлению к камере
        // Is the movement tied to the fixation towards the camera
        let camera_locked = match self.camera_mode {
            CameraMode::FirstPerson => true,
            CameraMode::ThirdPerson => false,
        };

        // Rotate movement direction according to the camera
        direction = direction.rotated(Vector3::UP, deg_to_rad(camera_yaw as f64) as f32);

        if camera_locked {
            movement = direction * MOVEMENT_SPEED;
        } else {
            if direction != Vector3::ZERO {
                let mut new_rotate = -direction.x.atan2(-direction.z) % 360.0;
                new_rotate = lerp_angle(entity.get_rotation().y as f64, new_rotate as f64, TURN_SPEED * delta) as f32;

                // Update skin rotation for visual display
                entity
                    .bind_mut()
                    .rotate(Rotation::new(new_rotate.to_degrees(), camera_pitch));

                movement = entity.bind().get_transform().basis.col_c() * -1.0 * MOVEMENT_SPEED;
            }
        }
        if self.grounded_timer > 0.0 {
            self.vertical_movement = 0.0;
        } else {
            let custom_mass = self.character_controller.get_custom_mass().unwrap_or(1.0);
            self.vertical_movement += CHARACTER_GRAVITY * delta as f32 * custom_mass;
        }

        // Check physics ground check
        if controls.is_jumping() && self.grounded_timer > -0.1 && !Console::is_active() {
            entity.bind_mut().trigger_animation(GenericAnimations::Jump);
            self.vertical_movement = JUMP_SPEED;
        }

        movement.y = self.vertical_movement;
        movement *= delta as f32;

        movement
    }

    pub fn update_vision(&mut self) -> Option<Gd<LookAt>> {
        let mut filter = QueryFilter::default();
        filter.exclude_collider(&self.collider);

        let camera_controller = self.camera_controller.bind();
        let ray_direction = camera_controller.get_ray_from_center();
        let Some((cast_result, physics_type)) = self.physics.cast_ray(ray_direction, filter) else {
            self.look_at_message = "-".to_string();
            return None;
        };

        self.look_at_message = match physics_type {
            PhysicsType::ChunkMeshCollider(_chunk_position) => {
                let selected_block = cast_result.get_selected_block();
                format!("block:{selected_block:?}")
            }
            PhysicsType::EntityCollider(entity_id) => {
                format!("entity_id:{entity_id}")
            }
        };
        let look_at = Gd::<LookAt>::from_init_fn(|_base| LookAt::create(cast_result, physics_type));
        return Some(look_at);
    }

    pub fn get_look_at_message(&self) -> &String {
        &self.look_at_message
    }

    pub fn custom_process(&mut self, delta: f64, chunk_loaded: bool, world_slug: &String) {
        #[cfg(feature = "trace")]
        let _span = tracy_client::span!("player_controller");

        let now = std::time::Instant::now();
        let mut move_shape_elapsed: std::time::Duration = std::time::Duration::ZERO;
        let mut detect_is_grounded_elapsed: std::time::Duration = std::time::Duration::ZERO;
        let mut vision_elapsed: std::time::Duration = std::time::Duration::ZERO;

        // Set lock if chunk is in loading
        self.collider.set_enabled(chunk_loaded);

        if chunk_loaded {
            let detect_is_grounded_now = std::time::Instant::now();
            self.detect_is_grounded(delta);
            detect_is_grounded_elapsed = detect_is_grounded_now.elapsed();

            let movement = self.get_movement(delta);

            let mut filter = QueryFilter::default();

            // Only solid ground
            filter.exclude_sensors();

            filter.exclude_collider(&self.collider);

            let move_shape_now = std::time::Instant::now();
            let translation =
                self.character_controller
                    .move_shape(&self.collider, filter, delta, movement.to_network());
            move_shape_elapsed = move_shape_now.elapsed();

            self.collider.set_position(self.collider.get_position() + translation);

            let vision_now = std::time::Instant::now();
            let hit = self.update_vision();
            self.signals().look_at_update().emit(hit.as_ref());
            vision_elapsed = vision_now.elapsed();

            let action_type = if self.controls.bind().is_main_action() {
                Some(PlayerActionType::Main)
            } else if self.controls.bind().is_second_action() {
                Some(PlayerActionType::Second)
            } else {
                None
            };

            if let Some(action_type) = action_type {
                let action = Gd::<PlayerAction>::from_init_fn(|_base| {
                    PlayerAction::create(hit, action_type, world_slug.clone())
                });
                let captured = Input::singleton().get_mouse_mode() == MouseMode::CAPTURED;
                if captured && self.ui_lock <= 0.0 && self.window_focus {
                    let selected_item = Gd::<SelectedItemGd>::from_init_fn(|_base| {
                        SelectedItemGd::create(self.get_selected_item().clone())
                    });
                    self.signals().player_action().emit(&action, &selected_item);
                }
            }
        }

        // Sync godot object position
        let physics_pos = self.collider.get_position();
        // Controller position is lowered by half of the center of mass position
        self.base_mut().set_position(Vector3::new(
            physics_pos.x,
            physics_pos.y - CONTROLLER_HEIGHT / 2.0,
            physics_pos.z,
        ));

        self.update_cache_movement();

        let elapsed = now.elapsed();
        #[cfg(debug_assertions)]
        if elapsed >= crate::WARNING_TIME {
            log::warn!(
                target: "player_controller",
                "&7custom_process lag:{:.2?} move_shape:{:.2?} detect_is_grounded:{:.2?} vision:{:.2?}",
                elapsed, move_shape_elapsed, detect_is_grounded_elapsed, vision_elapsed,
            );
        }
    }

    fn update_cache_movement(&mut self) {
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
            if let Some(entity) = self.entity.as_mut() {
                entity.bind_mut().handle_movement(movement);
            }

            self.signals().player_move().emit(&new_movement, new_chunk);
            self.cache_movement = Some(new_movement);
        }
    }

    pub fn set_blocks(&mut self, worlds_manager: &WorldsManager) {
        let block_storage_lock = worlds_manager.get_block_storage_lock();

        let block_mesh_storage = worlds_manager.get_block_mesh_storage().unwrap();
        self.block_menu
            .bind_mut()
            .set_blocks(&*block_mesh_storage.bind(), block_storage_lock.clone());

        self.building_visualizer
            .bind_mut()
            .set_block_mesh_storage(block_mesh_storage.clone());
    }
}

#[godot_api]
impl PlayerController {
    #[signal]
    pub fn look_at_update(new_look: Option<Gd<LookAt>>);

    #[signal]
    pub fn player_move(new_movement: Gd<EntityMovement>, new_chunk: bool);

    #[signal]
    pub fn player_action(action: Gd<PlayerAction>, item: Gd<SelectedItemGd>);

    #[func]
    fn on_block_selected(&mut self, block: Gd<BlockIconSelect>) {
        let block_info = BlockDataInfo::create(*block.bind().get_block_id(), None);
        self.set_selected_item(Some(SelectedItem::BlockPlacing(block_info)));
    }

    #[func]
    fn on_block_menu_closed(&mut self) {
        self.ui_lock = 0.1;
    }

    #[func]
    fn on_window_focus_entered(&mut self) {
        self.window_focus = true;
        self.ui_lock = 0.1;
    }

    #[func]
    fn on_window_focus_exited(&mut self) {
        self.window_focus = false;
    }

    /// Handle camera rotation from Rotation object
    #[func]
    fn on_camera_rotation(&mut self, yaw: f32, pitch: f32) {
        if let Some(entity) = self.entity.as_mut() {
            match self.camera_mode {
                CameraMode::FirstPerson => {
                    entity.bind_mut().rotate(Rotation::new(yaw, pitch));
                }
                CameraMode::ThirdPerson => {
                    entity.bind_mut().set_pitch(pitch);
                }
            }
        }
    }
}

#[godot_api]
impl INode3D for PlayerController {
    fn ready(&mut self) {
        let camera_controller = self.camera_controller.clone();
        camera_controller
            .signals()
            .on_camera_rotation()
            .connect_other(&self.to_gd(), Self::on_camera_rotation);
        self.base_mut().add_child(&camera_controller);

        self.base()
            .get_window()
            .unwrap()
            .signals()
            .focus_entered()
            .connect_other(&self.to_gd(), PlayerController::on_window_focus_entered);

        self.base()
            .get_window()
            .unwrap()
            .signals()
            .focus_exited()
            .connect_other(&self.to_gd(), PlayerController::on_window_focus_exited);

        let controls = self.controls.clone();
        self.base_mut().add_child(&controls);

        let block_menu = self.block_menu.clone();
        self.base_mut().add_child(&block_menu);

        block_menu
            .signals()
            .menu_closed()
            .connect_other(&self.to_gd(), PlayerController::on_block_menu_closed);
        block_menu
            .signals()
            .block_clicked()
            .connect_other(&self.to_gd(), PlayerController::on_block_selected);

        let building_visualizer = self.building_visualizer.clone();
        self.signals()
            .look_at_update()
            .connect_other(&building_visualizer, BuildingVisualizer::on_look_at_update);
        self.base_mut().add_child(&building_visualizer);

        self.change_camera_mode(self.camera_mode.clone())
    }

    fn process(&mut self, delta: f64) {
        #[cfg(feature = "trace")]
        let _span = tracy_client::span!("player_controller");

        let now = std::time::Instant::now();

        self.ui_lock = (self.ui_lock - delta as f32).max(0.0);

        if self.controls.bind().is_toggle_block_selection() && !Console::is_active() {
            let is_active = self.block_menu.bind().is_active();
            self.block_menu.bind_mut().toggle(!is_active);
        }

        if self.controls.bind().is_switch_camera_mode() && !Console::is_active() {
            let new_mode = match self.camera_mode {
                CameraMode::FirstPerson => CameraMode::ThirdPerson,
                CameraMode::ThirdPerson => CameraMode::FirstPerson,
            };
            self.change_camera_mode(new_mode);
        }

        // Rotation of the selected object
        let mut selected_item_updated = false;
        if let Some(selected_item) = self.selected_item.as_mut() {
            match selected_item {
                SelectedItem::BlockPlacing(block_info) => {
                    let face = match block_info.get_face() {
                        Some(f) => f.clone(),
                        None => BlockFace::default(),
                    };
                    if self.controls.bind().is_rotate_left() {
                        block_info.set_face(Some(face.rotate_left()));
                        selected_item_updated = true;
                    } else if self.controls.bind().is_rotate_right() {
                        block_info.set_face(Some(face.rotate_right()));
                        selected_item_updated = true;
                    }
                }
            }
        }
        if self.controls.bind().is_cancel_selection() || self.controls.bind().is_escape() {
            self.selected_item = None;
            selected_item_updated = true;
        }
        if selected_item_updated {
            self.set_selected_item(self.selected_item.clone());
        }

        let elapsed = now.elapsed();
        #[cfg(debug_assertions)]
        if elapsed >= crate::WARNING_TIME {
            log::warn!(target: "player_controller", "&7process lag: {:.2?}", elapsed);
        }
    }
}
