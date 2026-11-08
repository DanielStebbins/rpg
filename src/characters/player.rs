use bevy::{
    picking::{hover::HoverMap, pointer::PointerId},
    prelude::*,
    sprite::Anchor,
};

use crate::characters::{Character, Position, Velocity};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Held;

#[derive(Component)]
pub struct InPlayerLeftHand;

#[derive(Component)]
pub struct InPlayerRightHand;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Character,
        Player,
        Sprite::from_image(asset_server.load("sprites/player.png")),
        Anchor::BOTTOM_CENTER,
        Position::default(),
        Velocity::default(),
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_velocity: Single<&mut Velocity, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction.y += 1.0;
    }
    let delta = direction.normalize_or_zero() * 200.0;
    player_velocity.0 = player_velocity.0.lerp(delta, 0.5);
}

pub fn player_held(
    mouse_input: Res<ButtonInput<MouseButton>>,
    hover_map: Res<HoverMap>,
    player_entity: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let Some(hovered) = hover_map.get(&PointerId::Mouse) else {
            return;
        };
        let Some(front_hovered) = hovered.iter().nth(0) else {
            return;
        };
        let front_hovered_entity = *front_hovered.0;
        commands
            .entity(*player_entity)
            .add_child(front_hovered_entity);
        commands.entity(front_hovered_entity).insert((
            Position(Vec3::new(5.0, 14.0, 0.0)),
            Held,
            InPlayerLeftHand,
        ));
    }
    if mouse_input.just_pressed(MouseButton::Right) {
        let Some(hovered) = hover_map.get(&PointerId::Mouse) else {
            return;
        };
        let Some(front_hovered) = hovered.iter().nth(0) else {
            return;
        };
        let front_hovered_entity = *front_hovered.0;
        commands
            .entity(*player_entity)
            .add_child(front_hovered_entity);
        commands.entity(front_hovered_entity).insert((
            Position(Vec3::new(-7.5, 14.0, 0.0)),
            Held,
            InPlayerRightHand,
        ));
    }
}
