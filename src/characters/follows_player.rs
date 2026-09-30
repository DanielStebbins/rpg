use bevy::prelude::*;

use crate::characters::{Velocity, player::Player};

#[derive(Component)]
pub struct FollowsPlayer {
    pub outer_radius: f32,
    pub inner_radius: f32,
    pub speed: f32,
}

pub fn follow_player(
    mut q_followers: Query<(&FollowsPlayer, &Transform, &mut Velocity), Without<Player>>,
    player_transform: Single<&Transform, With<Player>>,
) {
    let player_position = player_transform.translation.truncate();
    for (follower, follower_transform, mut follower_velocity) in q_followers.iter_mut() {
        let follower_position = follower_transform.translation.truncate();
        let distance = follower_position.distance(player_position);
        let mut delta = Vec2::ZERO;
        if follower.inner_radius < distance && distance < follower.outer_radius {
            let direction = (player_position - follower_position).normalize_or_zero();
            delta = direction * follower.speed;
        }
        follower_velocity.0 = follower_velocity.0.lerp(delta, 0.5);
    }
}
