use bevy::prelude::*;

pub const BACKGROUND_WIDTH: f32 = 1500.0;
pub const BACKGROUND_HEIGHT: f32 = 1000.0;

pub const HALF_BACKGROUND_WIDTH: f32 = BACKGROUND_WIDTH / 2.0;
pub const HALF_BACKGROUND_HEIGHT: f32 = BACKGROUND_HEIGHT / 2.0;

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/grass.png"),
            custom_size: Some(Vec2::new(BACKGROUND_WIDTH, BACKGROUND_HEIGHT)),
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
