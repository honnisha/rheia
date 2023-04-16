use std::net::UdpSocket;
use std::time::{Instant, SystemTime};

use crate::client_scripts::scripts_manager::ScriptsManager;
use crate::console::console_handler::{Console, CONSOLE_OUTPUT_CHANNEL};
use godot::engine::Engine;
use godot::prelude::*;
use renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};

const PROTOCOL_ID: u64 = 7;
const CHANNEL_ID: u8 = 0;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    scripts_manager: ScriptsManager,
    client: Option<RenetClient>,
    last_updated: Instant,
}

#[godot_api]
impl Main {
    //self.scripts_manager.run_event(
    //    "onConsoleCommand".to_string(),
    //    vec![Dynamic::from(new_text.to_string())],
    //);

    fn handle_console_command(&mut self, command: String) {
        if command.len() == 0 {
            return;
        }
        let finded = self.scripts_manager.run_command(command.clone());
        if !finded {
            Console::send_message(format!("[color=#DE4747]Command \"{}\" not found[/color]", command));
        }
    }
}

#[godot_api]
impl NodeVirtual for Main {
    fn init(base: Base<Node>) -> Self {
        Main {
            base,
            scripts_manager: ScriptsManager::new(),
            client: None,
            last_updated: Instant::now(),
        }
    }

    fn ready(&mut self) {
        godot_print!("Start loading main scene;");
        if Engine::singleton().is_editor_hint() {
            return;
        }

        self.scripts_manager.rescan_scripts();
        godot_print!("Main scene loaded;");

        let ip_port = "127.0.0.1:14191";
        let server_addr = ip_port.clone().parse().unwrap();

        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

        let connection_config = RenetConnectionConfig::default();

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

        let client_id = current_time.as_millis() as u64;

        let authentication = ClientAuthentication::Unsecure {
            protocol_id: PROTOCOL_ID,
            client_id,
            server_addr,
            user_data: None,
        };
        let client = RenetClient::new(current_time, socket, connection_config, authentication).unwrap();
        self.client = Some(client);
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.try_iter() {
            self.handle_console_command(message);
        }

        if let Some(client) = self.client.as_mut() {
            // Receive new messages and update client
            let now = Instant::now();
            client.update(now - self.last_updated).unwrap();
            self.last_updated = now;

            if client.is_connected() {
                // Receive message from server
                while let Some(message) = client.receive_message(CHANNEL_ID) {
                    // Handle received message
                }

                // Send message
                client.send_message(CHANNEL_ID, "client text".as_bytes().to_vec());
            }

            // Send packets to server
            client.send_packets().unwrap();
        }
    }
}
