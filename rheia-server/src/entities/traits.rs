use bevy_ecs::component::Component;
use network::entities::EntityNetworkComponent;

pub trait IEntityNetworkComponent: Component {
    fn to_network(&self) -> EntityNetworkComponent;
    fn get_empty() -> EntityNetworkComponent;
}
