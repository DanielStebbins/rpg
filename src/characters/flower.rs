use bevy::{prelude::*, sprite::Anchor};

use crate::characters::Position;

#[derive(Component)]
pub struct Flower;

pub fn spawn_flowers(mut commands: Commands, asset_server: Res<AssetServer>) {
    let positions = [
        Vec3::new(-220.0, 0.0, 80.0),
        Vec3::new(-40.0, 0.0, -150.0),
        Vec3::new(-10.0, 0.0, 50.0),
        Vec3::new(15.0, 0.0, -220.0),
        Vec3::new(100.0, 0.0, 300.0),
    ];
    for position in positions.iter() {
        commands.spawn((
            Flower,
            Sprite::from_image(asset_server.load("sprites/flower.png")),
            Anchor::BOTTOM_CENTER,
            Position(*position),
        ));
    }
}
