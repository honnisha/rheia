use common::blocks::block_type::BlockCategory;
use godot::{
    classes::{Control, FlowContainer, IControl, Material, VBoxContainer},
    prelude::*,
};

use crate::{
    client_scripts::resource_manager::ResourceManager,
    scenes::components::{block_icon::BlockIcon, button::CustomButton},
    utils::textures::texture_mapper::TextureMapper,
    world::block_storage::BlockStorage,
};

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct BlockSelection {
    base: Base<Control>,

    #[export]
    categories_holder: Option<Gd<VBoxContainer>>,

    #[export]
    icons_grid: Option<Gd<FlowContainer>>,

    #[export]
    button_scene: Option<Gd<PackedScene>>,

    #[export]
    block_icon_scene: Option<Gd<PackedScene>>,
}

#[godot_api]
impl BlockSelection {
    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state);
    }

    pub fn init_blocks(
        &mut self,
        block_storage: &BlockStorage,
        material: &Gd<Material>,
        resource_manager: &ResourceManager,
        texture_mapper: &TextureMapper,
    ) {
        let icons_grid = self.icons_grid.as_mut().unwrap();

        for (block_id, block_type) in block_storage.iter() {
            let mut icon = self.block_icon_scene.as_ref().unwrap().instantiate_as::<BlockIcon>();
            icon.set_custom_minimum_size(Vector2::new(75.0, 75.0));
            icon.bind_mut().setup_icons(
                *block_id,
                block_type,
                material,
                texture_mapper,
                block_storage,
                &*resource_manager.get_resources_storage(),
            );
            icons_grid.add_child(&icon);
        }
    }

    #[signal]
    fn block_selected();
}

#[godot_api]
impl IControl for BlockSelection {
    fn ready(&mut self) {
        let categories_holder = self.categories_holder.as_mut().unwrap();
        for child in categories_holder.get_children().iter_shared() {
            child.free();
        }

        for category in BlockCategory::external_iter() {
            let mut button = self.button_scene.as_ref().unwrap().instantiate_as::<CustomButton>();
            button.set_text(&*category.to_str());
            categories_holder.add_child(&button);
        }

        let icons_grid = self.icons_grid.as_mut().unwrap();
        for child in icons_grid.get_children().iter_shared() {
            child.free();
        }
    }
}
