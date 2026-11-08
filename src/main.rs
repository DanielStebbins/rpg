use bevy::{picking::PickingSettings, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{
    camera::{camera_follow_player, camera_zoom, spawn_camera},
    characters::{
        Position, apply_velocity,
        dog::spawn_dog,
        flower::spawn_flowers,
        follows_player::follow_player,
        player::{player_held, player_movement, spawn_player},
        positions_to_transforms,
        status_effects::apply_wading,
    },
    world::{
        background::spawn_background,
        water::{set_wading, spawn_lakes},
    },
};

mod camera;
mod characters;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(PickingSettings {
            is_window_picking_enabled: false,
            ..default()
        })
        .register_type::<Position>()
        .add_systems(
            Startup,
            (
                spawn_background,
                spawn_lakes,
                spawn_flowers,
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
                set_wading,
                player_movement,
                follow_player,
                apply_wading,
                apply_velocity,
                player_held,
                positions_to_transforms,
                camera_follow_player,
            )
                .chain(),
        )
        .run();
}
