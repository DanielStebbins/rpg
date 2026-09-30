use bevy::prelude::*;

use crate::{
    camera::{camera_follow_player, camera_zoom, spawn_camera},
    characters::{
        apply_velocity,
        dog::spawn_dog,
        follows_player::follow_player,
        player::{player_movement, spawn_player},
    },
    world::{background::spawn_background, water::spawn_lake},
};

mod camera;
mod characters;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(
            Startup,
            (
                spawn_background,
                spawn_lake,
                spawn_player,
                spawn_dog,
                spawn_camera,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                camera_zoom,
                player_movement,
                follow_player,
                apply_velocity,
                camera_follow_player,
            )
                .chain(),
        )
        .run();
}
