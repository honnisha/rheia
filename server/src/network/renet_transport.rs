use renet::{
    transport::{NetcodeServerTransport, NetcodeTransportError},
    RenetServer,
};

use bevy::{app::AppExit, prelude::*};

use super::renet_server::RenetSet;

/// Set for networking systems.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TransportSet {
    _Client,
    Server,
}

pub struct NetcodeServerPlugin;

impl Plugin for NetcodeServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NetcodeTransportError>();
        app.configure_set(
            TransportSet::Server
                .run_if(resource_exists::<NetcodeServerTransport>().and_then(resource_exists::<RenetServer>()))
                .after(RenetSet::Server),
        );

        app.add_system(
            Self::update_system
                .in_base_set(CoreSet::PreUpdate)
                .in_set(TransportSet::Server),
        );
        app.add_system(
            Self::send_packets
                .in_base_set(CoreSet::PostUpdate)
                .in_set(TransportSet::Server),
        );
        app.add_system(
            Self::disconnect_on_exit
                .in_base_set(CoreSet::PostUpdate)
                .in_set(TransportSet::Server),
        );
    }
}

impl NetcodeServerPlugin {
    pub fn update_system(
        mut transport: ResMut<NetcodeServerTransport>,
        mut server: ResMut<RenetServer>,
        time: Res<Time>,
        mut transport_errors: EventWriter<NetcodeTransportError>,
    ) {
        if let Err(e) = transport.update(time.delta(), &mut server) {
            transport_errors.send(e);
        }
    }

    pub fn send_packets(mut transport: ResMut<NetcodeServerTransport>, mut server: ResMut<RenetServer>) {
        transport.send_packets(&mut server);
    }

    pub fn disconnect_on_exit(
        exit: EventReader<AppExit>,
        mut transport: ResMut<NetcodeServerTransport>,
        mut server: ResMut<RenetServer>,
    ) {
        if !exit.is_empty() {
            transport.disconnect_all(&mut server);
        }
    }
}
