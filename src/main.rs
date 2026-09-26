use bevy::prelude::*;

use crate::{
    background::spawn_background,
    camera::{camera_follow_player, spawn_camera},
    player::{player_movement, spawn_player},
};

mod background;
mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_background, spawn_player, spawn_camera).chain(),
        )
        .add_systems(Update, (player_movement, camera_follow_player).chain())
        .run();
}
