use bevy::{
    picking::{hover::HoverMap, pointer::PointerId},
    prelude::*,
    sprite::Anchor,
    window::PrimaryWindow,
};

use crate::characters::{Character, Position, Velocity, translation_to_position};

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
    maybe_left_hand_entity: Option<Single<Entity, (With<Held>, With<InPlayerLeftHand>)>>,
    maybe_right_hand_entity: Option<Single<Entity, (With<Held>, With<InPlayerRightHand>)>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(left_hand_entity) = maybe_left_hand_entity {
            if let Some(cursor_position) = window.cursor_position() {
                let (camera, camera_transform) = *camera_query;
                if let Ok(cursor_translation) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    let position = translation_to_position(&cursor_translation.extend(0.0));
                    commands
                        .entity(*left_hand_entity)
                        .remove::<(ChildOf, Held, InPlayerLeftHand)>()
                        .insert(position);
                }
            }
        } else {
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
    }
    if mouse_input.just_pressed(MouseButton::Right) {
        if let Some(right_hand_entity) = maybe_right_hand_entity {
            if let Some(cursor_position) = window.cursor_position() {
                let (camera, camera_transform) = *camera_query;
                if let Ok(cursor_translation) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    let position = translation_to_position(&cursor_translation.extend(0.0));
                    commands
                        .entity(*right_hand_entity)
                        .remove::<(ChildOf, Held, InPlayerRightHand)>()
                        .insert(position);
                }
            }
        } else {
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
}
