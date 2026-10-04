use bevy::input::mouse::MouseWheel;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::characters::player::Player;
use crate::world::background::{HALF_BACKGROUND_HEIGHT, HALF_BACKGROUND_WIDTH};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera_follow_player(
    player: Single<(&Transform, &Sprite), With<Player>>,
    camera: Single<(&mut Transform, &Projection), (With<Camera2d>, Without<Player>)>,
    window: Single<&Window, With<PrimaryWindow>>,
    images: Res<Assets<Image>>,
) {
    let (player_transform, player_sprite) = player.into_inner();
    let Some(player_sprite_image) = images.get(&player_sprite.image) else {
        panic!("Player sprite should have an image");
    };
    let player_sprite_height = player_sprite_image.size_f32().y;
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
    // Camera should focus on the middle of the player's sprite, not their feet.
    let player_y = player_transform.translation.y + player_sprite_height * 0.5;
    let half_window_height = (window.height() / 2.0) * projection.scale;
    if player_y - half_window_height > -HALF_BACKGROUND_HEIGHT
        && player_y + half_window_height < HALF_BACKGROUND_HEIGHT
    {
        camera_transform.translation.y = player_y;
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
