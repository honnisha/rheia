use bevy_ecs::world::World;

pub struct WorldManager {
    world: World,
}

impl WorldManager {
    pub fn new() -> Self {
        WorldManager {
            world: World::new(),
        }
    }
}
