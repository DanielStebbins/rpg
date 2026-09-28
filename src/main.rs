use bevy::prelude::*;

use crate::{
    background::spawn_background,
    camera::{camera_follow_player, camera_zoom, spawn_camera},
    characters::{
        dog::spawn_dog,
        follows_player::follow_player,
        player::{player_movement, spawn_player},
    },
};

mod background;
mod camera;
mod characters;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(
            Startup,
            (spawn_background, spawn_player, spawn_dog, spawn_camera).chain(),
        )
        .add_systems(
            Update,
            (
                camera_zoom,
                player_movement,
                follow_player,
                camera_follow_player,
            )
                .chain(),
        )
        .run();
}
