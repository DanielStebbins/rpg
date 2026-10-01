use bevy::prelude::*;

pub mod dog;
pub mod follows_player;
pub mod player;
pub mod status_effects;

// Camera Angle: 45 degrees.
// Angles closer to 90 mean more top-down.
// Values closer to 0 mean more side-profile.
// Paste the sine and cosine of the above angle here (`.sin()` and `.cos()` are non-const).
const Y_SCALE: f32 = 0.70710678;
const Z_SCALE: f32 = 0.70710678;

#[derive(Component)]
pub struct Character;

#[derive(Component, Default)]
pub struct Position(Vec3);

#[derive(Component, Default)]
pub struct Velocity(Vec3);

pub fn apply_velocity(time: Res<Time>, mut q_characters: Query<(&mut Position, &Velocity)>) {
    let delta_secs = time.delta_secs();
    for (mut position, velocity) in q_characters.iter_mut() {
        position.0 += velocity.0 * delta_secs;
        // TODO: Make gravity better.
        if position.0.z > 0.0 {
            position.0.z -= 50.0 * delta_secs;
        }
    }
}

pub fn positions_to_transforms(mut commands: Commands, q_characters: Query<(Entity, &Position)>) {
    for (character_entity, character_position) in q_characters.iter() {
        let translation = Vec3::new(
            character_position.0.x,
            character_position.0.y * Y_SCALE + character_position.0.z * Z_SCALE,
            10.0,
        );
        commands
            .entity(character_entity)
            .insert(Transform::from_translation(translation));
    }
}
