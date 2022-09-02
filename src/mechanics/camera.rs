use crate::entities::player::Player;
use bevy::prelude::*;

pub fn move_camera(
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let new_camera_position = Vec3{x:player_transform.translation.x, y:player_transform.translation.y, z:camera_transform.translation.z};
            camera_transform.translation = new_camera_position;
        }
    }
}
