use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime};

use crate::client_scripts::scripts_manager::ScriptsManager;
use crate::console::console_handler::{Console, CONSOLE_OUTPUT_CHANNEL};
use godot::engine::Engine;
use godot::prelude::*;
use renet::{ClientAuthentication, RenetClient, RenetConnectionConfig};

const PROTOCOL_ID: u64 = 7;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,
    scripts_manager: ScriptsManager,
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
        let delta_time = Duration::from_millis(16);
        let server_addr = ip_port.clone().parse().unwrap();
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let connection_config = RenetConnectionConfig::default();

        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: None,
        };
        let mut client = RenetClient::new(current_time, socket, connection_config, authentication).unwrap();
        let channel_id = 0;

        println!("Trying connect to {}", ip_port);

        // Your gameplay loop
        loop {
            // Receive new messages and update client
            client.update(delta_time).unwrap();

            if client.is_connected() {
                // Receive message from server
                while let Some(message) = client.receive_message(channel_id) {
                    // Handle received message
                }

                // Send message
                client.send_message(channel_id, "client text".as_bytes().to_vec());
            }

            // Send packets to server
            client.send_packets().unwrap();
        }
    }

    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        for message in CONSOLE_OUTPUT_CHANNEL.1.try_iter() {
            self.handle_console_command(message);
        }
    }
}
