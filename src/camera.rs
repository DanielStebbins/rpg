use bevy::input::mouse::MouseWheel;
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
    camera: Single<(&mut Transform, &Projection), (With<Camera2d>, Without<Player>)>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let (mut camera_transform, camera_projection) = camera.into_inner();
    let Projection::Orthographic(ref projection) = *camera_projection else {
        panic!("Camera projection should be orthographic");
    };
    let player_x = player_transform.translation.x;
    let half_window_width = (window.width() / 2.0) * projection.scale;
    if player_x - half_window_width > -HALF_BACKGROUND_WIDTH
        && player_x + half_window_width < HALF_BACKGROUND_WIDTH
    {
        camera_transform.translation.x = player_x;
    }
    let player_y = player_transform.translation.y;
    let half_window_height = (window.height() / 2.0) * projection.scale;
    if player_y - half_window_height > -HALF_BACKGROUND_HEIGHT
        && player_y + half_window_height < HALF_BACKGROUND_HEIGHT
    {
        camera_transform.translation.y = player_transform.translation.y;
    }
}

pub fn camera_zoom(
    mut scroll_reader: MessageReader<MouseWheel>,
    mut camera_projection: Single<&mut Projection, With<Camera2d>>,
) {
    let mut zoom_delta = 0.0;
    for event in scroll_reader.read() {
        zoom_delta -= event.y * 0.1;
    }
    if let Projection::Orthographic(ref mut orthographic) = **camera_projection {
        orthographic.scale = (orthographic.scale + zoom_delta).clamp(0.05, 1.0);
    }
}
