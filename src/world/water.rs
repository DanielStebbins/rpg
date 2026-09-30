use bevy::prelude::*;

enum WaterDepth {
    Shallow,
    // Deep,
}

#[derive(Component)]
pub struct Lake {
    pub radius: f32,
    pub depth: WaterDepth,
}

pub fn spawn_lake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Lake {
            radius: 200.0,
            depth: WaterDepth::Shallow,
        },
        Mesh2d(meshes.add(Circle::new(200.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.5, 0.8))),
        Transform::from_xyz(300.0, 0.0, 1.0),
    ));
}
