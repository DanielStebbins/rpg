use bevy::{prelude::*, sprite::Anchor};

use crate::characters::{Character, Position, Velocity, follows_player::FollowsPlayer};

#[derive(Component)]
pub struct Dog;

pub fn spawn_dog(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Character,
        Dog,
        FollowsPlayer {
            outer_radius: 1000.0,
            inner_radius: 20.0,
            speed: 175.0,
        },
        Sprite::from_image(asset_server.load("sprites/dog.png")),
        Anchor::BOTTOM_CENTER,
        Position(Vec3::new(-30.0, 0.0, 0.0)),
        Velocity::default(),
    ));
}
