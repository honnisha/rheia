use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::world::worlds_manager::WorldsManager;
use common::chunks::block_position::{BlockPosition, BlockPositionTrait};
use godot::{
    classes::{
        rendering_server::RenderingInfo, Engine, HBoxContainer, IMarginContainer, MarginContainer, RenderingServer,
        RichTextLabel, VBoxContainer,
    },
    prelude::*,
};
use lazy_static::lazy_static;
use network::client::NetworkInfo;

lazy_static! {
    static ref DEBUG_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

macro_rules! debug_first_string {
    () => {
        "FPS: {:.0}
Currently rendering:
{} objects
{:.1}K primitive indices
{} draw calls
{:.1} MB video mem used"
    };
}
macro_rules! debug_world_string {
    () => {
        "World: {}
Controller position: {}
Character state: {}
Chunks loaded: {}
Chunk position: {}
Chunk info: {}
Look at: {}
"
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
        text.set_text(&new_text);
    }

    pub fn is_active() -> bool {
        DEBUG_ACTIVE.load(Ordering::Relaxed)
    }

    pub fn toggle(&mut self, state: bool) {
        DEBUG_ACTIVE.store(state, Ordering::Relaxed);

        self.base_mut().set_visible(DebugInfo::is_active());
    }

    pub fn update_debug(&mut self, worlds_manager: &Gd<WorldsManager>, network_info: NetworkInfo) {
        if !DebugInfo::is_active() {
            return;
        }

        let mut rendering_server = RenderingServer::singleton();
        let first_text = format!(
            debug_first_string!(),
            Engine::singleton().get_frames_per_second(),
            rendering_server.get_rendering_info(RenderingInfo::TOTAL_OBJECTS_IN_FRAME),
            rendering_server.get_rendering_info(RenderingInfo::TOTAL_PRIMITIVES_IN_FRAME) as f32 * 0.001,
            rendering_server.get_rendering_info(RenderingInfo::TOTAL_DRAW_CALLS_IN_FRAME),
            rendering_server.get_rendering_info(RenderingInfo::VIDEO_MEM_USED) as f32 / (1024.0 * 1024.0),
        );
        DebugInfo::change_text(&self.first_row, first_text);

        let wm = worlds_manager.bind();
        let world_text = match wm.get_world() {
            Some(w) => {
                let player_controller = wm.get_player_controller().as_ref().unwrap().bind();
                let world = w.bind();
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

                let chunk_info = match world.get_chunk_map().get_chunk(&chunk_pos) {
                    Some(c) => {
                        let c = c.read();
                        format!("loaded:{}", c.is_loaded())
                    }
                    None => "-".to_string(),
                };

                format!(
                    debug_world_string!(),
                    world.get_slug(),
                    controller_positioin,
                    player_controller.get_current_animation(),
                    world.get_chunks_count(),
                    chunk_pos,
                    chunk_info,
                    player_controller.get_look_at_message(),
                )
            }
            None => "World: -".to_string(),
        };
        DebugInfo::change_text(&self.world_row, world_text);

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
impl IMarginContainer for DebugInfo {
    fn init(base: Base<MarginContainer>) -> Self {
        Self {
            base: base,
            first_row: DebugInfo::load_row(),
            world_row: DebugInfo::load_row(),
            network_row: DebugInfo::load_row(),
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_visible(false);

        let mut base = self
            .base()
            .get_node_as::<VBoxContainer>("MarginContainer/VBoxContainer");
        base.add_child(&self.first_row);
        base.add_child(&self.world_row);
        base.add_child(&self.network_row);
    }
}
