pub use renet;

use bevy::prelude::*;

use renet::{RenetServer, ServerEvent};

#[cfg(feature = "transport")]
pub mod transport;

/// Set for networking systems.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum RenetSet {
    /// Runs when server resource available.
    Server,
    /// Runs when client resource available.
    _Client,
}

pub struct RenetServerPlugin;

impl Plugin for RenetServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Events<ServerEvent>>();

        app.configure_set(RenetSet::Server.run_if(resource_exists::<RenetServer>()));

        app.add_system(Self::update_system.in_base_set(CoreSet::PreUpdate).in_set(RenetSet::Server));
    }
}

impl RenetServerPlugin {
    pub fn update_system(mut server: ResMut<RenetServer>, time: Res<Time>, mut server_events: EventWriter<ServerEvent>) {
        server.update(time.delta());

        while let Some(event) = server.get_event() {
            server_events.send(event);
        }
    }
}
