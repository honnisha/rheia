use crate::ui::item_decription::ItemDescription;
use common::chunks::chunk_data::BlockIndexType;
use godot::{
    classes::{
        Camera3D, ColorRect, Control, IControl, InputEvent, InputEventMouseButton, SubViewportContainer, TextureRect,
    }, global::MouseButton, meta::AsArg, prelude::*
};

const ICON_SCENE: &str = "res://scenes/components/block_icon.tscn";
const ICON_DESC_OFFSET: Vector2 = Vector2::new(25.0, 10.0);

#[derive(Clone, Copy, Debug, PartialEq, GodotClass)]
#[class(no_init)]
pub struct BlockIconSelect {
    block_id: BlockIndexType,
}

impl BlockIconSelect {
    pub fn create(block_id: BlockIndexType) -> Self {
        Self { block_id }
    }

    pub fn get_block_id(&self) -> &BlockIndexType {
        &self.block_id
    }
}

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct BlockIcon {
    base: Base<Control>,

    block_id: Option<BlockIndexType>,

    #[export]
    pub viewport_container: Option<Gd<SubViewportContainer>>,

    #[export]
    pub block_anchor: Option<Gd<Node3D>>,

    #[export]
    pub outline_texture: Option<Gd<TextureRect>>,

    #[export]
    pub backgroud_color: Option<Gd<ColorRect>>,

    #[export]
    camera: Option<Gd<Camera3D>>,

    hover_text: Option<String>,

    item_description: Option<Gd<ItemDescription>>,
}

#[godot_api]
impl BlockIcon {
    pub fn create() -> Gd<Self> {
        load::<PackedScene>(ICON_SCENE).instantiate_as::<Self>()
    }

    pub fn set_hover_text(&mut self, hover_text: Option<String>) {
        self.hover_text = hover_text;
    }

    /*
    pub fn texturize(&mut self) {
        let Some(viewport_container) = self.viewport_container.as_mut() else {
            log::error!("block_icon; icon viewport_container is not set or already texturized");
            return;
        };
        let viewport = viewport_container.get_children().iter_shared().next().unwrap();
        let viewport = viewport.cast::<SubViewport>();
        let Some(texture) = viewport.get_texture() else {
            log::error!("block_icon; viewport texture found");
            return;
        };
        let image = texture.get_image().unwrap();

        let mut color_rect = TextureRect::new_alloc();
        let image_texture = ImageTexture::create_from_image(&image).unwrap();
        color_rect.set_texture(&image_texture);
        color_rect.set_expand_mode(ExpandMode::IGNORE_SIZE);
        color_rect.set_anchors_preset(LayoutPreset::FULL_RECT);

        self.base_mut().add_child(&color_rect);

        self.viewport_container.as_mut().unwrap().queue_free();
    }
    */

    #[func]
    fn on_gui_input(&mut self, event: Gd<InputEvent>) {
        let Some(block_id) = self.block_id.as_ref() else {
            return;
        };

        if let Ok(event) = event.clone().try_cast::<InputEventMouseButton>() {
            if event.get_button_index() == MouseButton::LEFT && event.is_pressed() {
                let icon = Gd::<BlockIconSelect>::from_init_fn(|_base| BlockIconSelect::create(block_id.clone()));
                self.signals().icon_clicked().emit(&icon)
            }
        }
    }

    #[func]
    fn on_mouse_entered(&mut self) {
        if let Some(hover_text) = self.hover_text.as_ref() {
            let mut item_description = ItemDescription::create();
            item_description.bind_mut().set_description(&hover_text);

            let mouse_position = self.base().get_viewport().unwrap().get_mouse_position();
            item_description.set_global_position(mouse_position + ICON_DESC_OFFSET);

            let mut root = self.base().get_tree().unwrap().get_root().unwrap();
            root.add_child(&item_description);

            self.item_description = Some(item_description);
        }

        self.toggle_selected(true);
        let Some(block_id) = self.block_id.as_ref() else {
            return;
        };

        let icon = Gd::<BlockIconSelect>::from_init_fn(|_base| BlockIconSelect::create(block_id.clone()));
        self.signals().icon_mouse_entered().emit(&icon);
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        if let Some(mut item_description) = self.item_description.take() {
            item_description.queue_free();
        }

        self.toggle_selected(false);
        let Some(block_id) = self.block_id.as_ref() else {
            return;
        };
        let icon = Gd::<BlockIconSelect>::from_init_fn(|_base| BlockIconSelect::create(block_id.clone()));
        self.signals().icon_mouse_exited().emit(&icon);
    }

    #[signal]
    pub fn icon_clicked(block: Gd<BlockIconSelect>);

    #[signal]
    pub fn icon_mouse_entered(block: Gd<BlockIconSelect>);

    #[signal]
    pub fn icon_mouse_exited(block: Gd<BlockIconSelect>);
}

impl BlockIcon {
    pub fn set_block_id(&mut self, block_id: BlockIndexType) {
        self.block_id = Some(block_id);
    }

    pub fn toggle_selected(&mut self, state: bool) {
        if let Some(outline_texture) = self.outline_texture.as_mut() {
            outline_texture.set_visible(state);
        }
        if let Some(backgroud_color) = self.backgroud_color.as_mut() {
            let mut color = backgroud_color.get_color();
            color.a = match state {
                true => 0.25,
                false => 0.15,
            };
            backgroud_color.set_color(color);
        }
    }

    pub fn set_camera_size(&mut self, size: f32) {
        if let Some(camera) = self.camera.as_mut() {
            camera.set_size(size);
        }
    }

    pub fn add_component(&mut self, node: impl AsArg<Option<Gd<Node>>>) {
        if let Some(block_anchor) = self.block_anchor.as_mut() {
            block_anchor.add_child(node);
        };
    }
}

#[godot_api]
impl IControl for BlockIcon {
    fn ready(&mut self) {
        self.signals().gui_input().connect_self(BlockIcon::on_gui_input);

        self.signals().mouse_entered().connect_self(BlockIcon::on_mouse_entered);
        self.signals().mouse_exited().connect_self(BlockIcon::on_mouse_exited);

        self.toggle_selected(false);
    }

    fn process(&mut self, _delta: f64) {
        if self.item_description.is_some() {
            let mouse_position = self.base().get_viewport().unwrap().get_mouse_position();
            let item_description = self.item_description.as_mut().unwrap();
            item_description.set_global_position(mouse_position + ICON_DESC_OFFSET);
        }
    }
}
