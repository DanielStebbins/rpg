use bevy::prelude::*;

use crate::characters::{Velocity, follows_player::FollowsPlayer};

#[derive(Component)]
pub struct Dog;

pub fn spawn_dog(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Dog,
        FollowsPlayer {
            outer_radius: 1000.0,
            inner_radius: 20.0,
            speed: 175.0,
        },
        Sprite::from_image(asset_server.load("sprites/dog.png")),
        Velocity::default(),
        Transform::from_xyz(-30.0, 0.0, 1.0),
    ));
}
