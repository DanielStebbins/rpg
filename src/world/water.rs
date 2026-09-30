use bevy::prelude::*;

use crate::characters::{Character, status_effects::Wading};

#[derive(Component)]
pub struct Lake {
    pub radius: f32,
}

pub fn spawn_lakes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Lake { radius: 200.0 },
        Mesh2d(meshes.add(Circle::new(200.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 0.8))),
        Transform::from_xyz(300.0, 0.0, 1.0),
    ));
    commands.spawn((
        Lake { radius: 50.0 },
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 0.8))),
        Transform::from_xyz(30.0, -100.0, 1.0),
    ));
    commands.spawn((
        Lake { radius: 25.0 },
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 0.8))),
        Transform::from_xyz(-100.0, 30.0, 1.0),
    ));
}

pub fn set_wading(
    mut commands: Commands,
    q_characters: Query<(Entity, &Transform), With<Character>>,
    q_lakes: Query<(&Lake, &Transform)>,
) {
    for (character_entity, character_transform) in q_characters.iter() {
        let character_position = character_transform.translation.truncate();
        let mut wading = false;
        for (lake, lake_transform) in q_lakes.iter() {
            let lake_position = lake_transform.translation.truncate();
            if character_position.distance(lake_position) < lake.radius {
                wading = true;
                break;
            }
        }
        if wading {
            commands.entity(character_entity).insert(Wading);
        } else {
            commands.entity(character_entity).remove::<Wading>();
        }
    }
}
