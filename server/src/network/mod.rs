use std::sync::atomic::Ordering;

use bevy::{
    prelude::{IntoSystemConfig, IntoSystemConfigs, EventWriter},
    time::Time,
};
use bevy_app::{App, CoreSet, Plugin, AppExit};
use bevy_ecs::{system::Res, system::ResMut};

use crate::{client_resources::resources_manager::ResourceManager, ServerSettings};

use self::server::{NetworkServer, ServerRuntime};

pub mod player;
pub mod server;

pub struct NetworkPlugin;

impl Default for NetworkPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ServerRuntime::new());

        let server_settings = app.world.get_resource::<ServerSettings>().unwrap();
        let ip_port = format!("{}:{}", server_settings.get_args().ip, server_settings.get_args().port);
        app.insert_resource(NetworkServer::init(ip_port));

        app.add_system(Self::update_tick);
    }
}

impl NetworkPlugin {
    pub fn update_tick(
        mut network_server: ResMut<NetworkServer>,
        resource_manager: Res<ResourceManager>,
        server_runtime: Res<ServerRuntime>,

        time: Res<Time>,
        mut exit: EventWriter<AppExit>
    ) {
        if server_runtime.server_active.load(Ordering::Relaxed) {
            network_server.update(time.delta(), resource_manager.as_ref());
        } else {
            network_server.stop();
            exit.send(AppExit);
        }
    }
}
