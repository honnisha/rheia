use common::chunks::chunk_data::BlockDataInfo;
use godot::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, GodotClass)]
#[class(no_init)]
pub struct SelectedItemGd {
    item: Option<SelectedItem>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectedItem {
    BlockPlacing(BlockDataInfo),
}
impl SelectedItemGd {
    pub fn create(item: Option<SelectedItem>) -> Self {
        Self { item }
    }

    pub fn get_selected_item(&self) -> Option<&SelectedItem> {
        self.item.as_ref()
    }
}
