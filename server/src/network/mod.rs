use std::sync::atomic::Ordering;

use bevy::{
    prelude::{IntoSystemConfig, IntoSystemConfigs},
    time::Time,
};
use bevy_app::{App, CoreSet, Plugin};
use bevy_ecs::{
    prelude::{resource_exists, Events},
    schedule::SystemConfigs,
    system::Res,
    system::ResMut,
};
use renet::{RenetError, ServerEvent};

use crate::{client_resources::resources_manager::ResourceManager, ServerSettings, console_send};

use self::server::{NetworkServer, ServerRuntime};

pub mod player;
pub mod server;

pub struct NetworkPlugin {
    pub clear_events: bool,
}

impl Default for NetworkPlugin {
    fn default() -> Self {
        Self { clear_events: true }
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<ServerEvent>>();
        app.init_resource::<Events<RenetError>>();

        app.insert_resource(ServerRuntime::new());

        if self.clear_events {
            app.add_systems(Self::get_clear_event_systems().in_base_set(CoreSet::PreUpdate));
        }

        let server_settings = app.world.get_resource::<ServerSettings>().unwrap();
        let ip_port = format!("{}:{}", server_settings.get_args().ip, server_settings.get_args().port);
        app.insert_resource(NetworkServer::init(ip_port));

        app.add_system(
            Self::update_tick
                .in_base_set(CoreSet::PreUpdate)
                .run_if(resource_exists::<NetworkServer>()),
        );
    }
}

impl NetworkPlugin {
    pub fn update_tick(
        mut network_server: ResMut<NetworkServer>,
        resource_manager: Res<ResourceManager>,
        server_runtime: Res<ServerRuntime>,

        time: Res<Time>,
    ) {
        if server_runtime.server_active.load(Ordering::Relaxed) {
            network_server.update(time.delta(), resource_manager.as_ref());
        } else {
            network_server.stop();
        }
        console_send("update_tick".to_string());
    }

    pub fn get_clear_event_systems() -> SystemConfigs {
        (
            Events::<ServerEvent>::update_system,
            Events::<RenetError>::update_system,
        )
            .into_configs()
    }
}
