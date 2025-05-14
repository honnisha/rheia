use ahash::{HashMap, HashSet};
use common::blocks::block_info::BlockIndexType;
use godot::classes::control::{LayoutPreset, SizeFlags};
use godot::classes::{FlowContainer, ScrollContainer, Theme};
use godot::prelude::*;
use godot::{
    classes::{input::MouseMode, Input},
    obj::Gd,
};

use crate::scenes::main_scene::DEFAULT_THEME_PATH;
use crate::ui::tabs::tabs_component::TabsUIComponent;
use crate::ui::window::WindowUIComponent;
use crate::world::block_storage::BlockStorage;

use super::block_icon::{BlockIcon, BlockIconSelect};
use super::block_icons_storage::BlockIconsStorage;

#[derive(GodotClass)]
#[class(no_init, tool, base=Node)]
pub struct BlockSelection {
    base: Base<Node>,

    window: Gd<WindowUIComponent>,
    tabs: Gd<TabsUIComponent>,

    selected_block_id: Option<BlockIndexType>,

    icons: HashMap<BlockIndexType, Gd<BlockIcon>>,
}

#[godot_api]
impl BlockSelection {
    #[signal]
    fn on_closed();

    #[func]
    fn on_icon_clicked(&mut self, block: Gd<BlockIconSelect>) {
        self.selected_block_id = Some(*block.bind().get_block_id());
        self.toggle(false);

        self.signals().on_closed().emit();
    }
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
            icons: Default::default(),
        }
    }

    pub fn toggle(&mut self, state: bool) {
        self.window.bind_mut().toggle(state);

        for (_block_id, icon) in self.icons.iter_mut() {
            icon.bind_mut().toggle_selected(false);
        }

        if state {
            Input::singleton().set_mouse_mode(MouseMode::VISIBLE);
        } else {
            Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        }
    }

    pub fn set_blocks(&mut self, block_icons_storage: &BlockIconsStorage, block_storage: &BlockStorage) {
        let gd = self.base().to_godot();

        // Collect all block categories
        let mut categories: HashSet<String> = HashSet::default();
        for (_block_id, block_type) in block_storage.iter() {
            categories.insert(block_type.get_category().clone());
        }

        let default_theme = load::<Theme>(DEFAULT_THEME_PATH);

        for category in categories.iter() {
            let mut tab_category = self.tabs.bind_mut().add_category(category.clone(), category.clone());

            let mut scroll = ScrollContainer::new_alloc();
            tab_category.add_child(&scroll);
            scroll.set_anchors_preset(LayoutPreset::FULL_RECT);
            scroll.set_theme(&default_theme);

            let mut flow_container = FlowContainer::new_alloc();
            scroll.add_child(&flow_container);
            flow_container.set_h_size_flags(SizeFlags::EXPAND_FILL);
            flow_container.set_v_size_flags(SizeFlags::EXPAND_FILL);
            flow_container.set_theme(&default_theme);

            for (block_id, block_type) in block_storage.iter() {
                if block_type.get_category() == category {

                    let icon = block_icons_storage.get_icon(block_id).unwrap();
                    let mut icon = icon.clone();
                    // let mut icon = icon.duplicate().unwrap().cast::<BlockIcon>();
                    flow_container.add_child(&icon);

                    icon.connect("icon_clicked", &Callable::from_object_method(&gd, "on_icon_clicked"));

                    // icon.connect(
                    //     "icon_mouse_entered",
                    //     &Callable::from_object_method(&gd, "on_icon_mouse_entered"),
                    // );
                    // icon.connect(
                    //     "icon_mouse_exited",
                    //     &Callable::from_object_method(&gd, "on_icon_mouse_exited"),
                    // );
                    self.icons.insert(block_id.clone(), icon);
                }
            }
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
        let window = self.window.clone();
        self.base_mut().add_child(&window);
        self.toggle(false);
    }
}
