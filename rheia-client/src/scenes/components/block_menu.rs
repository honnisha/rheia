use ahash::HashMap;
use common::chunks::chunk_data::BlockIndexType;
use godot::classes::control::{LayoutPreset, SizeFlags};
use godot::classes::{FlowContainer, RichTextLabel, ScrollContainer, Theme};
use godot::obj::Gd;
use godot::prelude::*;

use crate::scenes::main_scene::DEFAULT_THEME_PATH;
use crate::ui::tabs::tabs_component::TabsUIComponent;
use crate::ui::window::WindowUIComponent;
use crate::world::worlds_manager::BlockStorageType;

use super::block_icon::{BlockIcon, BlockIconSelect};
use super::block_mesh_storage::BlockMeshStorage;

const TABS_FOOTER_HEIGHT: f32 = 50.0;

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

    help_text: Gd<RichTextLabel>,
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
    fn on_icon_mouse_entered(&mut self, block: Gd<BlockIconSelect>) {
        let b = block.bind();
        let block_id = b.get_block_id();

        let block_storage = self.block_storage_lock.as_ref().unwrap();
        let block_storage = block_storage.read();

        let block_type = block_storage.get(&block_id).unwrap();

        let text = format!(
            tabs_footer_text!(),
            block_id,
            block_type.get_slug(),
            block_type.get_block_content(),
        );
        self.help_text.set_text(&text);
    }

    #[func]
    fn on_icon_mouse_exited(&mut self, _block: Gd<BlockIconSelect>) {
        self.help_text.set_text("");
    }

    #[func]
    fn on_window_closed(&mut self) {
        self.signals().closed().emit();
        self.help_text.set_text("");
        self.window.bind_mut().toggle(false);
    }
}

impl BlockMenu {
    pub fn create(base: Base<Node>) -> Self {
        let mut window = WindowUIComponent::create("Block selection".to_string(), true);

        let mut tabs = TabsUIComponent::create();

        // Create help text
        let default_theme = load::<Theme>(DEFAULT_THEME_PATH);
        let mut help_text = RichTextLabel::new_alloc();
        help_text.set_use_bbcode(true);
        help_text.set_anchors_preset(LayoutPreset::BOTTOM_WIDE);
        help_text.set_custom_minimum_size(Vector2::new(0.0, TABS_FOOTER_HEIGHT));
        help_text.set_theme(&default_theme);

        // Add help text to tab footer
        {
            let mut t = tabs.bind_mut();
            let footer_holder = t.get_footer_holder_mut();
            footer_holder.add_child(&help_text);
        }

        window.bind_mut().add_component(&tabs);
        Self {
            base,
            window,
            tabs,
            icons: Default::default(),
            help_text,
            block_storage_lock: None,
        }
    }

    pub fn toggle(&mut self, state: bool) {
        self.window.bind_mut().toggle(state);

        for (_block_id, icon) in self.icons.iter_mut() {
            icon.bind_mut().toggle_selected(false);
        }
    }

    pub fn set_blocks(&mut self, block_mesh_storage: &BlockMeshStorage, block_storage_lock: BlockStorageType) {
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
                    let icon = block_mesh_storage.get_icon(&block_id).unwrap();
                    let icon = icon.clone();
                    // let mut icon = icon.duplicate().unwrap().cast::<BlockIcon>();
                    flow_container.add_child(&icon);

                    icon.signals()
                        .icon_clicked()
                        .connect_other(&gd, BlockMenu::on_icon_clicked);

                    icon.signals()
                        .icon_mouse_entered()
                        .connect_other(&gd, BlockMenu::on_icon_mouse_entered);

                    icon.signals()
                        .icon_mouse_exited()
                        .connect_other(&gd, BlockMenu::on_icon_mouse_exited);

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
        let window = self.window.clone();

        window
            .signals()
            .closed()
            .connect_other(&self.to_gd().clone(), BlockMenu::on_window_closed);

        self.base_mut().add_child(&window);
        self.toggle(false);
    }
}
