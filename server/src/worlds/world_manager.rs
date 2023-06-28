use bevy_ecs::world::World;

pub struct WorldManager {
    slug: String,
    world: World,
}

impl WorldManager {
    pub fn new(slug: String) -> Self {
        WorldManager {
            slug: slug,
            world: World::new(),
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }
}
