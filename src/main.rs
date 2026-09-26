use bevy::prelude::*;

use crate::{background::spawn_background, camera::spawn_camera, player::spawn_player};

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
        .run();
}
