use common::blocks::{block_info::BlockIndexType, block_type::BlockCategory};
use godot::{
    classes::{input::MouseMode, Control, FlowContainer, IControl, Material, VBoxContainer},
    prelude::*,
};

use crate::{
    client_scripts::resource_manager::ResourceManager,
    scenes::components::{
        block_icon::{BlockIcon, BlockIconSelect},
        button::CustomButton,
    },
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

    selected_block_id: Option<BlockIndexType>,
}

#[godot_api]
impl BlockSelection {
    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state);

        if state {
            Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        } else {
            Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        }
    }

    pub fn is_active(&self) -> bool {
        self.base().is_visible()
    }

    pub fn get_selected_block_id(&self) -> &Option<BlockIndexType> {
        &self.selected_block_id
    }

    pub fn init_blocks(
        &mut self,
        block_storage: &BlockStorage,
        material: &Gd<Material>,
        resource_manager: &ResourceManager,
        texture_mapper: &TextureMapper,
    ) {
        let mut icons_grid = self.icons_grid.as_mut().unwrap().clone();

        for (block_id, block_type) in block_storage.iter() {
            if self.selected_block_id.is_none() {
                self.selected_block_id = Some(*block_id);
            }

            let mut icon = self.block_icon_scene.as_ref().unwrap().instantiate_as::<BlockIcon>();
            icon.set_custom_minimum_size(Vector2::new(75.0, 75.0));
            icon.bind_mut().setup_icon(
                *block_id,
                block_type,
                material,
                texture_mapper,
                block_storage,
                &*resource_manager.get_resources_storage(),
            );

            icon.connect(
                "icon_clicked",
                &Callable::from_object_method(&self.base().to_godot(), "on_icon_clicked"),
            );
            icons_grid.add_child(&icon);
        }
    }

    #[func]
    fn on_icon_clicked(&mut self, block: Gd<BlockIconSelect>) {
        self.selected_block_id = Some(*block.bind().get_block_id());
        self.toggle(false);
    }
}

#[godot_api]
impl IControl for BlockSelection {
    fn ready(&mut self) {
        self.base_mut().set_visible(false);

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
