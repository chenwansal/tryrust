mod plugin;
mod component;
mod system;
mod resource;

use bevy::prelude::*;

use resource::reso_timer::ResoTimer;
use plugin::plg_assets::PluginAssets;
use system::sys_exe_input;

// ==== Main ====
fn main() {

    let fps = 1.0 / 60.0;

    let timer = ResoTimer {
        value: Timer::from_seconds(fps, TimerMode::Repeating),
    };

    App::new()
        .insert_resource(timer)
        .add_system(sys_exe_input::tick_keyboard_input)
        .add_plugins(DefaultPlugins)
        .add_plugin(PluginAssets)
        .run();
}
