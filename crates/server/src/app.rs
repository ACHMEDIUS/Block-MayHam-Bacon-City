use bevy::prelude::*;

use crate::plugins::core_plugin::CorePlugin;

pub fn run() {
    App::new().add_plugins(CorePlugin).run();
}
