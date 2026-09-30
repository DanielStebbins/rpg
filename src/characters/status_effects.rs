use bevy::prelude::*;

use crate::characters::Velocity;

#[derive(Component)]
pub struct Wading;

pub fn apply_wading(mut q_wader: Query<&mut Velocity, With<Wading>>) {
    for mut wader_velocity in q_wader.iter_mut() {
        wader_velocity.0 = wader_velocity.0 / 2.0;
    }
}
