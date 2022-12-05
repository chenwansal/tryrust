use bevy::prelude::*;

pub fn init(mut command: Commands, assets_server: Res<AssetServer>) {
    command.spawn(Camera2dBundle::default());
    command.spawn(SpriteBundle {
        texture: assets_server.load("icon/icon.png"),
        transform: Transform::from_xyz(100.0, 0.0, 0.0),
        ..Default::default()
    });
}