use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use godot::{
    engine::{Engine, HBoxContainer, MarginContainer, RichTextLabel, VBoxContainer},
    prelude::*,
};
use lazy_static::lazy_static;

use crate::{world::world_manager::WorldManager, network::client::NetworkContainer};

lazy_static! {
    static ref DEBUG_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

macro_rules! debug_first_string {
    () => {
        "FPS: {:.0}
Controller position: {}
Threads count: {}"
    };
}
macro_rules! debug_world_string {
    () => {
        "World: {}
Chunks loaded: {}
Chunk position: {}
Chunk info: {}"
    };
}
macro_rules! debug_network_string {
    () => {
        "Network connected: {}
Bytes received per second: {:.1}
Bytes received per sec: {:.1}
Bytes sent per sec: {:.1}
Packet loss: {:.1}"
    };
}

#[derive(GodotClass)]
#[class(base=MarginContainer)]
pub struct DebugInfo {
    #[base]
    base: Base<MarginContainer>,
    first_row: Gd<HBoxContainer>,
    world_row: Gd<HBoxContainer>,
    network_row: Gd<HBoxContainer>,
}

impl DebugInfo {
    pub fn load_row() -> Gd<HBoxContainer> {
        load::<PackedScene>("res://scenes/debug_row.tscn").instantiate_as::<HBoxContainer>()
    }

    pub fn change_text(row: &Gd<HBoxContainer>, new_text: String) {
        let mut text = row.get_node_as::<RichTextLabel>("PanelContainer/MarginContainer/RichTextLabel");
        text.set_text(GodotString::from(new_text));
    }

    pub fn is_active() -> bool {
        DEBUG_ACTIVE.load(Ordering::Relaxed)
    }

    pub fn toggle(&mut self, state: bool) {
        DEBUG_ACTIVE.store(state, Ordering::Relaxed);

        self.base.set_visible(DebugInfo::is_active());
    }

    pub fn update_debug(&mut self, world_manager: GdRef<WorldManager>, camera: &Gd<Camera3D>) {
        if !DebugInfo::is_active() {
            return;
        }

        let controller_positioin = match world_manager.get_player_controller().bind().get_handler() {
            Some(h) => {
                let controller_pos = h.get_position(&camera);
                format!(
                    "{:.2} {:.2} {:.2} yaw:{:.2} pitch:{:.2}",
                    controller_pos.x,
                    controller_pos.y,
                    controller_pos.z,
                    h.get_yaw(&camera),
                    h.get_pitch(&camera),
                )
            }
            None => "-".to_string(),
        };

        let first_text = format!(
            debug_first_string!(),
            Engine::singleton().get_frames_per_second(),
            controller_positioin,
            rayon::current_num_threads()
        );
        DebugInfo::change_text(&self.first_row, first_text);

        let camera_pos = camera.get_position();
        let chunk_pos =
            BlockPosition::new(camera_pos.x as i64, camera_pos.y as i64, camera_pos.z as i64).get_chunk_position();
        let second_text = match world_manager.get_world() {
            Some(w) => {
                let world = w.bind();
                let chunk_info = match world.get_chunk(&chunk_pos) {
                    Some(c) => {
                        let c = c.borrow();
                        format!("sended:{} loaded:{}", c.is_sended(), c.is_loaded())
                    }
                    None => "-".to_string(),
                };
                format!(
                    debug_world_string!(),
                    world.get_slug(),
                    world.get_chunks_count(),
                    chunk_pos,
                    chunk_info,
                )
            }
            None => "World: -".to_string(),
        };
        DebugInfo::change_text(&self.world_row, second_text);

        let network_text = if NetworkContainer::has_client() {
            let c = NetworkContainer::read();
            let container = c.as_ref().unwrap();
            let client = container.get_client();
            let network_info = client.network_info();

            format!(
                debug_network_string!(),
                !client.is_disconnected(),
                network_info.bytes_received_per_second,
                client.bytes_received_per_sec(),
                client.bytes_sent_per_sec(),
                client.packet_loss(),
            )
        } else {
            "Network connected: -".to_string()
        };
        DebugInfo::change_text(&self.network_row, network_text);
    }
}

#[godot_api]
impl NodeVirtual for DebugInfo {
    fn init(base: Base<MarginContainer>) -> Self {
        Self {
            base: base,
            first_row: DebugInfo::load_row(),
            world_row: DebugInfo::load_row(),
            network_row: DebugInfo::load_row(),
        }
    }

    fn ready(&mut self) {
        self.base.set_visible(false);

        let mut base = self.base.get_node_as::<VBoxContainer>("MarginContainer/VBoxContainer");
        base.add_child(self.first_row.share().upcast());
        base.add_child(self.world_row.share().upcast());
        base.add_child(self.network_row.share().upcast());
    }
}
