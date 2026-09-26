use bevy::prelude::*;

use crate::player::spawn_player;

mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_player).chain())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/grass.png"),
            custom_size: Some(Vec2::new(3000.0, 3000.0)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 0.5,
            },
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/player.png")),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
