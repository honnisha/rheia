use std::collections::HashMap;

use bevy::prelude::{Event, EventReader};

#[derive(Event)]
pub struct ResourceLoadedEvent {
    slug: String,
    scripts: HashMap<String, String>,
}

impl ResourceLoadedEvent {
    pub fn new(slug: String, scripts: HashMap<String, String>) -> Self {
        Self { slug, scripts }
    }
}

pub fn on_resource_loaded(mut resource_loaded_event: EventReader<ResourceLoadedEvent>) {
    for event in resource_loaded_event.iter() {}
}
