use ahash::HashMap;
use common::utils::uppercase_first;
use godot::{
    classes::{Control, IControl, VBoxContainer, control::LayoutPreset},
    prelude::*,
};

use super::tab_button::TabUIButton;

const TAB_COMPONENT_SCENE: &str = "res://scenes/ui/tab_content_component.tscn";
const TABS_COMPONENT_SCENE: &str = "res://scenes/ui/tabs_component.tscn";

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct TabContentUIComponent {
    base: Base<Control>,
}

impl TabContentUIComponent {
    pub fn create() -> Gd<Self> {
        load::<PackedScene>(TAB_COMPONENT_SCENE).instantiate_as::<Self>()
    }

    pub fn toggle(&mut self, state: bool) {
        self.base_mut().set_visible(state)
    }
}

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct TabsUIComponent {
    base: Base<Control>,

    #[export]
    tabs_holder: Option<Gd<VBoxContainer>>,

    #[export]
    tabs_content_holder: Option<Gd<Control>>,

    #[export]
    footer_holder: Option<Gd<Control>>,

    tabs_buttons: HashMap<String, Gd<TabUIButton>>,
    tabs_content: HashMap<String, Gd<TabContentUIComponent>>,

    active_tab: Option<String>,
}

impl TabsUIComponent {
    pub fn create() -> Gd<Self> {
        load::<PackedScene>(TABS_COMPONENT_SCENE).instantiate_as::<Self>()
    }

    pub fn get_footer_holder_mut(&mut self) -> &mut Gd<Control> {
        self.footer_holder
            .as_mut()
            .expect("Footer holder is required for TabsUIComponent")
    }

    pub fn add_category(&mut self, tab_key: String, title: String) -> Gd<TabContentUIComponent> {
        let tabs_content_holder = self.tabs_content_holder.as_mut().unwrap();
        let mut tab_content = TabContentUIComponent::create();
        tabs_content_holder.add_child(&tab_content);
        tab_content.set_anchors_preset(LayoutPreset::FULL_RECT);

        self.tabs_content.insert(tab_key.clone(), tab_content.clone());

        let tabs_holder = self.tabs_holder.as_mut().unwrap();

        let mut tab_button = TabUIButton::create(&title, title.clone());
        tab_button.set_text(&uppercase_first(&title).replace("-", " ").replace("_", " "));
        tabs_holder.add_child(&tab_button);
        self.tabs_buttons.insert(tab_key.clone(), tab_button.clone());

        tab_button.connect("tab_pressed", &Callable::from_object_method(&self.to_gd(), "on_tab_pressed"));

        match self.active_tab.as_ref() {
            Some(active_tab) => {
                tab_content.bind_mut().toggle(*active_tab == tab_key);
            }
            None => self.set_active_tab(&tab_key),
        }

        tab_content
    }

    pub fn set_active_tab(&mut self, new_tab_key: &String) {
        // If already active
        if let Some(active_tab) = self.active_tab.as_ref() {
            if active_tab == new_tab_key {
                return;
            }
        }
        for (tab_key, tab_button) in self.tabs_buttons.iter_mut() {
            tab_button.bind_mut().toggle_highlight(tab_key == new_tab_key);
        }
        for (tab_key, tab_content) in self.tabs_content.iter_mut() {
            tab_content.bind_mut().toggle(tab_key == new_tab_key);
        }
        self.active_tab = Some(new_tab_key.clone());
    }
}

#[godot_api]
impl TabsUIComponent {
    #[func]
    fn on_tab_pressed(&mut self, tab_key: String) {
        self.set_active_tab(&tab_key)
    }
}

#[godot_api]
impl IControl for TabsUIComponent {
    fn ready(&mut self) {
        let tabs_holder = self.tabs_holder.as_mut().unwrap();
        for child in tabs_holder.get_children().iter_shared() {
            child.free();
        }

        let tabs_content_holder = self.tabs_content_holder.as_mut().unwrap();
        for child in tabs_content_holder.get_children().iter_shared() {
            child.free();
        }
    }
}
