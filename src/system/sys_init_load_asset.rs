use crate::component::com_global_input::ComGlobalInput;
use bevy::prelude::*;

pub fn init(mut command: Commands, assets_server: Res<AssetServer>) {
    command.spawn(Camera2dBundle::default());
    command.spawn(ComGlobalInput {
        move_dir: Vec2::ZERO,
    });
    command.spawn(SpriteBundle {
        texture: assets_server.load("icon/icon.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
