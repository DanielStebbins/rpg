use bevy::prelude::*;

use crate::characters::Velocity;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Sprite::from_image(asset_server.load("sprites/player.png")),
        Transform::from_xyz(0.0, 0.0, 100.0),
        Velocity::default(),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_velocity: Single<&mut Velocity, With<Player>>,
) {
    let mut direction = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    let delta = direction.normalize_or_zero() * 200.0;
    player_velocity.0 = player_velocity.0.lerp(delta, 0.5);
}
