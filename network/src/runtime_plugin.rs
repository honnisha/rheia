use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use bevy::{prelude::{Plugin, EventWriter, App, IntoSystemConfig, CoreSet}, app::AppExit};
use lazy_static::lazy_static;


lazy_static! {
    static ref SERVER_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
}

#[derive(Default)]
pub struct RuntimePlugin;

impl RuntimePlugin {
    pub fn is_active() -> bool {
        SERVER_ACTIVE.load(Ordering::Relaxed)
    }

    pub fn stop() {
        SERVER_ACTIVE.store(false, Ordering::Relaxed);
    }
}

impl Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_runtime.in_base_set(CoreSet::First));
    }
}

fn update_runtime(mut exit: EventWriter<AppExit>) {
    if !RuntimePlugin::is_active() {
        exit.send(AppExit)
    }
}
