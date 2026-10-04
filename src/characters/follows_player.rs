use bevy::prelude::*;

use crate::characters::{Position, Velocity, player::Player};

#[derive(Component)]
pub struct FollowsPlayer {
    pub outer_radius: f32,
    pub inner_radius: f32,
    pub speed: f32,
}

pub fn follow_player(
    mut q_followers: Query<(&FollowsPlayer, &Position, &mut Velocity), Without<Player>>,
    player_position: Single<&Position, With<Player>>,
) {
    for (follower, follower_position, mut follower_velocity) in q_followers.iter_mut() {
        let distance = follower_position.0.distance(player_position.0);
        let mut delta = Vec3::ZERO;
        if follower.inner_radius < distance && distance < follower.outer_radius {
            let mut direction = (player_position.0 - follower_position.0).normalize_or_zero();
            direction.y = 0.0; // Followers cannot levitate upwards towards the player.
            delta = direction * follower.speed;
        }
        follower_velocity.0 = follower_velocity.0.lerp(delta, 0.5);
    }
}
