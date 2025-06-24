use common::chunks::chunk_data::BlockIndexType;
use godot::{
    classes::{Camera3D, ColorRect, Control, IControl, InputEvent, InputEventMouseButton, TextureRect},
    global::MouseButton,
    meta::AsObjectArg,
    prelude::*,
};

const ICON_SCENE: &str = "res://scenes/components/block_icon.tscn";

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
    pub block_anchor: Option<Gd<Node3D>>,

    #[export]
    pub outline_texture: Option<Gd<TextureRect>>,

    #[export]
    pub backgroud_color: Option<Gd<ColorRect>>,

    #[export]
    camera: Option<Gd<Camera3D>>,
}

#[godot_api]
impl BlockIcon {
    pub fn create() -> Gd<Self> {
        load::<PackedScene>(ICON_SCENE).instantiate_as::<Self>()
    }

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
        self.toggle_selected(true);
        let Some(block_id) = self.block_id.as_ref() else {
            return;
        };
        let icon = Gd::<BlockIconSelect>::from_init_fn(|_base| BlockIconSelect::create(block_id.clone()));
        self.signals().icon_mouse_entered().emit(&icon);
    }

    #[func]
    fn on_mouse_exited(&mut self) {
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

    pub fn add_component(&mut self, node: impl AsObjectArg<Node>) {
        self.block_anchor.as_mut().unwrap().add_child(node);
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
}
