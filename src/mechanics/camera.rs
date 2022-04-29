use crate::Player;
use bevy::prelude::*;

pub fn move_camera(
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if !player_query.is_empty() {
        let player_transform = player_query.single();
        let mut camera_transform = camera_query.single_mut();

        camera_transform.translation = player_transform.translation;
    }
}
