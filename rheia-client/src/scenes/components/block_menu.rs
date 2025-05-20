use ahash::HashMap;
use common::blocks::block_info::BlockIndexType;
use godot::classes::control::{LayoutPreset, SizeFlags};
use godot::classes::{FlowContainer, ScrollContainer, Theme};
use godot::prelude::*;
use godot::{
    classes::{Input, input::MouseMode},
    obj::Gd,
};

use crate::scenes::main_scene::DEFAULT_THEME_PATH;
use crate::ui::tabs::tabs_component::TabsUIComponent;
use crate::ui::window::WindowUIComponent;
use crate::world::block_storage::BlockStorage;

use super::block_icon::{BlockIcon, BlockIconSelect};
use super::block_mesh_storage::BlockMeshStorage;

#[derive(GodotClass)]
#[class(no_init, tool, base=Node)]
pub struct BlockMenu {
    base: Base<Node>,

    window: Gd<WindowUIComponent>,
    tabs: Gd<TabsUIComponent>,

    icons: HashMap<BlockIndexType, Gd<BlockIcon>>,
}

#[godot_api]
impl BlockMenu {
    #[signal]
    pub fn closed();

    #[signal]
    pub fn block_clicked(block: Gd<BlockIconSelect>);

    #[func]
    fn on_icon_clicked(&mut self, block: Gd<BlockIconSelect>) {
        self.signals().closed().emit();
        self.toggle(false);
        self.signals().block_clicked().emit(&block);
    }

    #[func]
    fn on_window_closed(&mut self) {
        self.signals().closed().emit();
        self.window.bind_mut().toggle(false);
    }
}

impl BlockMenu {
    pub fn create(base: Base<Node>) -> Self {
        let mut window = WindowUIComponent::create("Block selection".to_string(), true);

        let tabs = TabsUIComponent::create();
        window.bind_mut().add_component(&tabs);
        Self {
            base,
            window,
            tabs,
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

    pub fn set_blocks(&mut self, block_mesh_storage: &BlockMeshStorage, block_storage: &BlockStorage) {
        let gd = self.base().to_godot();

        // Collect all block categories

        let default_theme = load::<Theme>(DEFAULT_THEME_PATH);

        let categories = block_storage.get_categories();
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
                    let icon = block_mesh_storage.get_icon(block_id).unwrap();
                    let mut icon = icon.clone();
                    // let mut icon = icon.duplicate().unwrap().cast::<BlockIcon>();
                    flow_container.add_child(&icon);

                    icon.connect("icon_clicked", &Callable::from_object_method(&gd, "on_icon_clicked"));
                    self.icons.insert(block_id.clone(), icon);
                }
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.window.bind().is_visible()
    }
}

#[godot_api]
impl INode for BlockMenu {
    fn ready(&mut self) {
        let mut window = self.window.clone();

        window
            .signals()
            .closed()
            .connect_obj(&self.to_gd().clone(), BlockMenu::on_window_closed);

        self.base_mut().add_child(&window);
        self.toggle(false);
    }
}
