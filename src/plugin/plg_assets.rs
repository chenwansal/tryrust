use crate::system::sys_init_load_asset;

use bevy::prelude::*;

pub struct PluginAssets;
impl Plugin for PluginAssets {
    fn build(&self, app: &mut App) {
        app.add_startup_system(sys_init_load_asset::init);
    }
}
