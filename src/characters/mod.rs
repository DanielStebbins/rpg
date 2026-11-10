use bevy::prelude::*;

pub mod dog;
pub mod flower;
pub mod follows_player;
pub mod player;
pub mod status_effects;

// Camera Angle: 45 degrees.
// Angles closer to 90 mean more top-down.
// Values closer to 0 mean more side-profile.
// Paste the sine and cosine of the above angle here (`.sin()` and `.cos()` are non-const).
const Z_SCALE: f32 = 0.70710678; // Sine of the angle.
const Y_SCALE: f32 = 0.70710678; // Cosine of the angle.

#[derive(Component)]
pub struct Character;

#[derive(Component, Default, Reflect)]
pub struct Position(Vec3);

#[derive(Component, Default)]
pub struct Velocity(Vec3);

pub fn apply_velocity(time: Res<Time>, mut q_characters: Query<(&mut Position, &Velocity)>) {
    let delta_secs = time.delta_secs();
    for (mut position, velocity) in q_characters.iter_mut() {
        position.0 += velocity.0 * delta_secs;
        // TODO: Make gravity better.
        if position.0.y > 0.0 {
            position.0.y -= 50.0 * delta_secs;
        }
    }
}

pub fn position_to_translation(position: &Position) -> Vec3 {
    let scaled_z = position.0.z * Z_SCALE;
    Vec3::new(position.0.x, position.0.y * Y_SCALE - scaled_z, scaled_z)
}

pub fn translation_to_position(translation: &Vec3) -> Position {
    Position(Vec3::new(
        translation.x,
        (translation.y + translation.z) / Y_SCALE,
        translation.z / Z_SCALE,
    ))
}

pub fn positions_to_transforms(
    mut commands: Commands,
    q_characters: Query<(Entity, &Position, Option<&ChildOf>), Changed<Position>>,
) {
    for (character_entity, character_position, maybe_character_child_of) in q_characters.iter() {
        let mut translation = position_to_translation(character_position);
        let z_offset = match maybe_character_child_of {
            Some(_) => 0.0,
            None => 500.0,
        };
        translation.z = translation.z * 0.01 + z_offset;
        commands
            .entity(character_entity)
            .insert(Transform::from_translation(translation));
    }
}
