use std::{sync::atomic::Ordering, time::Duration};

use bevy::{prelude::EventWriter, time::Time};
use bevy_app::{App, AppExit, Plugin, ScheduleRunnerPlugin, ScheduleRunnerSettings};
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

        let tick_rate = server_settings.get_args().tick_rate;

        let ip_port = format!("{}:{}", server_settings.get_args().ip, server_settings.get_args().port);
        app.insert_resource(NetworkServer::init(ip_port));

        let tick_period = Duration::from_secs_f64((tick_rate as f64).recip());
        app.insert_resource(ScheduleRunnerSettings::run_loop(tick_period));
        app.add_plugin(ScheduleRunnerPlugin);

        app.add_system(Self::update_tick);
    }
}

impl NetworkPlugin {
    pub fn update_tick(
        mut network_server: ResMut<NetworkServer>,
        resource_manager: Res<ResourceManager>,
        server_runtime: Res<ServerRuntime>,

        time: Res<Time>,
        mut exit: EventWriter<AppExit>,
    ) {
        if server_runtime.server_active.load(Ordering::Relaxed) {
            network_server.update(time.delta(), resource_manager.as_ref());
        } else {
            network_server.stop();
            exit.send(AppExit);
        }
    }
}
