use common::blocks::block_info::BlockInfo;
use godot::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, GodotClass)]
#[class(no_init)]
pub struct SelectedItemGd {
    item: Option<SelectedItem>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectedItem {
    BlockPlacing(BlockInfo),
    BlockDestroy,
}
impl SelectedItemGd {
    pub fn create(item: Option<SelectedItem>) -> Self {
        Self { item }
    }

    pub fn get_selected_item(&self) -> Option<&SelectedItem> {
        self.item.as_ref()
    }
}
