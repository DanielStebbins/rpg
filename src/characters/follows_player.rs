use bevy::prelude::*;

use crate::characters::player::Player;

#[derive(Component)]
pub struct FollowsPlayer {
    pub outer_radius: f32,
    pub inner_radius: f32,
    pub speed: f32,
}

pub fn follow_player(
    mut q_followers: Query<(&FollowsPlayer, &mut Transform), Without<Player>>,
    player_transform: Single<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let player_position = player_transform.translation.truncate();
    for (follower, mut follower_transform) in q_followers.iter_mut() {
        let follower_position = follower_transform.translation.truncate();
        let distance = follower_position.distance(player_position);
        if follower.inner_radius < distance && distance < follower.outer_radius {
            let direction = (player_position - follower_position).normalize_or_zero();
            let delta = direction * follower.speed * time.delta_secs();
            follower_transform.translation += delta.extend(0.0);
        }
    }
}
