use bevy::prelude::*;

pub mod dog;
pub mod follows_player;
pub mod player;

#[derive(Component, Default)]
pub struct Velocity(Vec2);

pub fn apply_velocity(time: Res<Time>, mut q_character: Query<(&mut Transform, &Velocity)>) {
    let delta_secs = time.delta_secs();
    for (mut transform, velocity) in q_character.iter_mut() {
        transform.translation.x += velocity.0.x * delta_secs;
        transform.translation.y += velocity.0.y * delta_secs;
    }
}
