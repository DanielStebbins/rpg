use bevy::prelude::*;

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
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
