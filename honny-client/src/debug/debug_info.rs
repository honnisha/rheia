use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::{
    chunks::block_position::{BlockPosition, BlockPositionTrait}, network::client::ClientNetwork,
};
use godot::{
    engine::{Engine, HBoxContainer, MarginContainer, RichTextLabel, VBoxContainer},
    prelude::*,
};
use lazy_static::lazy_static;
use parking_lot::RwLockReadGuard;

use crate::{network::client::NetworkClientType, world::world_manager::WorldManager};

lazy_static! {
    static ref DEBUG_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

macro_rules! debug_first_string {
    () => {
        "FPS: {:.0}
Threads count: {}"
    };
}
macro_rules! debug_world_string {
    () => {
        "World: {}
Controller position: {}
Chunks loaded: {}
Chunk position: {}
Chunk info: {}"
    };
}
macro_rules! debug_network_string {
    () => {
        "Network connected: {}
KB received per second: {:.1}
KB received per sec: {:.1}
KB sent per sec: {:.1}
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

    pub fn update_debug(&mut self, world_manager: &WorldManager, network: RwLockReadGuard<NetworkClientType>) {
        if !DebugInfo::is_active() {
            return;
        }

        let first_text = format!(
            debug_first_string!(),
            Engine::singleton().get_frames_per_second(),
            rayon::current_num_threads()
        );
        DebugInfo::change_text(&self.first_row, first_text);

        let world_text = match world_manager.get_world() {
            Some(w) => {
                let world = w.bind();
                let player_controller = world.get_player_controller().bind();
                let controller_pos = player_controller.get_position();
                let controller_positioin = format!(
                    "{:.2} {:.2} {:.2} yaw:{:.2} pitch:{:.2}",
                    controller_pos.x,
                    controller_pos.y,
                    controller_pos.z,
                    player_controller.get_yaw(),
                    player_controller.get_pitch(),
                );

                let chunk_pos = BlockPosition::new(
                    controller_pos.x as i64,
                    controller_pos.y as i64,
                    controller_pos.z as i64,
                )
                .get_chunk_position();

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
                    controller_positioin,
                    world.get_chunks_count(),
                    chunk_pos,
                    chunk_info,
                )
            }
            None => "World: -".to_string(),
        };
        DebugInfo::change_text(&self.world_row, world_text);

        let network_info = network.get_network_info();
        let network_text = format!(
            debug_network_string!(),
            !network_info.is_disconnected,
            network_info.bytes_received_per_second / 1024.0,
            network_info.bytes_received_per_sec / 1024.0,
            network_info.bytes_sent_per_sec / 1024.0,
            network_info.packet_loss / 1024.0,
        );
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
        base.add_child(self.first_row.clone().upcast());
        base.add_child(self.world_row.clone().upcast());
        base.add_child(self.network_row.clone().upcast());
    }
}
