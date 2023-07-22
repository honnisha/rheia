use common::{blocks::block_info::BlockInfo, chunks::block_position::BlockPosition};
use godot::prelude::*;

use super::chunks::godot_chunks_container::ChunksContainer;

/// Godot world
/// Contains all things inside world
#[derive(GodotClass)]
#[class(base=Node)]
pub struct World {
    #[base]
    base: Base<Node>,
    slug: String,
    chunks_container: Option<Gd<ChunksContainer>>,
}

#[godot_api]
impl World {
    pub fn modify_block(&mut self, pos: &BlockPosition, block_info: BlockInfo) {
        self.chunks_container
            .as_mut()
            .unwrap()
            .bind_mut()
            .modify_block(pos, block_info);
    }
}

impl World {
    pub fn create(base: Base<Node>, slug: String) -> Self {
        World {
            base,
            slug: slug,
            chunks_container: Default::default(),
        }
    }

    pub fn get_slug(&self) -> &String {
        &self.slug
    }

    pub fn init_chunks_container(&mut self) {
        let mut container = Gd::<ChunksContainer>::with_base(|base| ChunksContainer::create(base));

        let container_name = GodotString::from("ChunksContainer");
        container.bind_mut().set_name(container_name.clone());

        self.base.add_child(container.upcast());
        self.chunks_container = Some(self.base.get_node_as::<ChunksContainer>(container_name));
    }
}

#[godot_api]
impl NodeVirtual for World {
    /// For default godot init; only World::create is using
    fn init(base: Base<Node>) -> Self {
        World::create(base, "Godot".to_string())
    }

    fn ready(&mut self) {
        self.init_chunks_container();
    }
}
