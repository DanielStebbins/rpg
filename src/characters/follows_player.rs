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
    let player_position_2d = player_position.0.truncate();
    for (follower, follower_position, mut follower_velocity) in q_followers.iter_mut() {
        let follower_position_2d = follower_position.0.truncate();
        let distance = follower_position_2d.distance(player_position_2d);
        let mut delta = Vec2::ZERO;
        if follower.inner_radius < distance && distance < follower.outer_radius {
            let direction = (player_position_2d - follower_position_2d).normalize_or_zero();
            delta = direction * follower.speed;
        }
        follower_velocity.0 = follower_velocity.0.lerp(delta.extend(0.0), 0.5);
    }
}
