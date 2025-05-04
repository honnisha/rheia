use ahash::HashSet;
use common::blocks::block_info::BlockIndexType;
use godot::prelude::*;
use godot::{
    classes::{input::MouseMode, Input},
    obj::Gd,
};

use crate::ui::tabs::tabs_component::TabsUIComponent;
use crate::ui::window::WindowUIComponent;
use crate::world::block_storage::BlockStorage;

use super::block_icons_storage::BlockIconsStorage;

#[derive(GodotClass)]
#[class(no_init, tool, base=Node)]
pub struct BlockSelection {
    base: Base<Node>,

    window: Gd<WindowUIComponent>,
    tabs: Gd<TabsUIComponent>,

    selected_block_id: Option<BlockIndexType>,
}

#[godot_api]
impl BlockSelection {
    #[signal]
    fn on_closed();
}

impl BlockSelection {
    pub fn create(base: Base<Node>) -> Self {
        let mut window = WindowUIComponent::create();
        let tabs = TabsUIComponent::create();
        window.bind_mut().add_component(&tabs);
        Self {
            base,
            window,
            tabs,
            selected_block_id: None,
        }
    }

    pub fn toggle(&mut self, state: bool) {
        self.window.bind_mut().toggle(state);

        if state {
            Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        } else {
            Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        }
    }

    pub fn set_blocks(&mut self, block_icons_storage: &mut BlockIconsStorage, block_storage: &BlockStorage) {
        // Collect all block categories
        let mut categories: HashSet<String> = HashSet::default();
        for (_block_id, block_type) in block_storage.iter() {
            categories.insert(block_type.get_category().clone());
        }
        for category in categories.iter() {
            let tab_category = self.tabs.bind_mut().add_category(category.clone(), category.clone());
        }
    }

    pub fn is_active(&self) -> bool {
        self.window.bind().is_visible()
    }
    pub fn get_selected_block_id(&self) -> &Option<BlockIndexType> {
        &self.selected_block_id
    }
}

#[godot_api]
impl INode for BlockSelection {
    fn ready(&mut self) {
        self.toggle(false);
    }
}
