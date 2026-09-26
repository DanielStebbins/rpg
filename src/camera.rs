use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    background::{HALF_BACKGROUND_HEIGHT, HALF_BACKGROUND_WIDTH},
    player::Player,
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera_follow_player(
    player_transform: Single<&Transform, With<Player>>,
    mut camera_transform: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let player_x = player_transform.translation.x;
    let half_window_width = window.width() / 2.0;
    if player_x - half_window_width > -HALF_BACKGROUND_WIDTH
        && player_x + half_window_width < HALF_BACKGROUND_WIDTH
    {
        camera_transform.translation.x = player_x;
    }
    let player_y = player_transform.translation.y;
    let half_window_height = window.height() / 2.0;
    if player_y - half_window_height > -HALF_BACKGROUND_HEIGHT
        && player_y + half_window_height < HALF_BACKGROUND_HEIGHT
    {
        camera_transform.translation.y = player_transform.translation.y;
    }
}
