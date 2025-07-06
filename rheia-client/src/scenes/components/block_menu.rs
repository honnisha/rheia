use ahash::HashMap;
use common::chunks::chunk_data::BlockIndexType;
use godot::classes::control::{LayoutPreset, SizeFlags};
use godot::classes::{FlowContainer, ScrollContainer, Theme};
use godot::obj::Gd;
use godot::prelude::*;

use crate::scenes::main_scene::DEFAULT_THEME_PATH;
use crate::ui::tabs::tabs_component::TabsUIComponent;
use crate::ui::window::WindowUIComponent;
use crate::world::worlds_manager::BlockStorageType;

use super::block_icon::{BlockIcon, BlockIconSelect};
use super::block_mesh_storage::BlockMeshStorage;

macro_rules! tabs_footer_text {
    () => {
        "[font_size=14][color=#CDCDCD]#{} {}[/color]
[font_size=10][color=#8B8B8B]Content: {:?}[/color][/font_size]"
    };
}

#[derive(GodotClass)]
#[class(no_init, tool, base=Node)]
pub struct BlockMenu {
    base: Base<Node>,

    block_storage_lock: Option<BlockStorageType>,

    window: Gd<WindowUIComponent>,
    tabs: Gd<TabsUIComponent>,

    icons: HashMap<BlockIndexType, Gd<BlockIcon>>,
}

#[godot_api]
impl BlockMenu {
    #[signal]
    pub fn menu_closed();

    #[signal]
    pub fn block_clicked(block: Gd<BlockIconSelect>);

    #[func]
    fn on_icon_clicked(&mut self, block: Gd<BlockIconSelect>) {
        self.signals().block_clicked().emit(&block);
        self.signals().menu_closed().emit();
        self.toggle(false);
    }

    #[func]
    fn on_window_closed(&mut self) {
        self.signals().menu_closed().emit();
        self.toggle(false);
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
            block_storage_lock: None,
        }
    }

    pub fn is_active(&self) -> bool {
        self.window.bind().is_visible()
    }

    pub fn toggle(&mut self, state: bool) {
        self.window.bind_mut().toggle(state);

        // for (_block_id, icon) in self.icons.iter_mut() {
        //     icon.bind_mut().toggle_selected(false);
        // }
    }

    pub fn set_blocks(&mut self, block_mesh_storage: &BlockMeshStorage, block_storage_lock: BlockStorageType) {
        if self.icons.len() > 0 {
            panic!("block_menu set_blocks already called! Is suppose to be one time job!");
        }

        self.block_storage_lock = Some(block_storage_lock.clone());

        let block_storage = block_storage_lock.read();

        let gd = self.to_gd().clone();

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

            for (block_slug, block_type) in block_storage.iter() {
                let block_id = block_storage.get_block_id(block_slug).unwrap();
                if block_type.get_category() == category {
                    let mut icon = block_mesh_storage.generate_icon(&block_id).unwrap();
                    flow_container.add_child(&icon);

                    icon.signals()
                        .icon_clicked()
                        .connect_other(&gd, BlockMenu::on_icon_clicked);

                    let text = format!(
                        tabs_footer_text!(),
                        block_id,
                        block_type.get_slug(),
                        block_type.get_block_content(),
                    );
                    icon.bind_mut().set_hover_text(Some(text));

                    self.icons.insert(block_id.clone(), icon);
                }
            }
        }
    }
}

#[godot_api]
impl INode for BlockMenu {
    fn ready(&mut self) {
        let window = self.window.clone();

        window
            .signals()
            .window_closed()
            .connect_other(&self.to_gd().clone(), BlockMenu::on_window_closed);

        self.base_mut().add_child(&window);
        self.toggle(false);
    }
}
