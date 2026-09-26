use bevy::prelude::*;

use crate::{camera::spawn_camera, player::spawn_player};

mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_player, spawn_camera).chain())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}
