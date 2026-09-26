use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/player.png")),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
