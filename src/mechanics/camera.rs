use crate::{entities::player::Player, LevelDimensions};
use bevy::prelude::*;

pub fn move_camera(
    level_dimension: Res<LevelDimensions>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), (With<Camera2d>, Without<Player>)>,
) {
    if camera_query.is_empty() {
        return;
    }

    if player_query.is_empty() {
        return;
    }

    if level_dimension.height == 0 || level_dimension.width == 0 {
        return;
    }

    let (mut camera_transform, camera_bounds) = camera_query.get_single_mut().unwrap();
    let player_transform = player_query.get_single().unwrap();
    
    let camera_width = (camera_bounds.right - camera_bounds.left).abs() + 1.0;
    let camera_height = (camera_bounds.top - camera_bounds.bottom).abs() + 1.0;

    if camera_width > level_dimension.width as f32 {
        camera_transform.translation.x = level_dimension.width as f32 / 2.0;
    }
    else {
        camera_transform.translation.x = player_transform.translation.x.clamp(camera_width / 2.0, level_dimension.width as f32 - (camera_width / 2.0));
    }

    if camera_height > level_dimension.height as f32 {
        camera_transform.translation.y = level_dimension.height as f32 / 2.0;
    }
    else {
        camera_transform.translation.y = player_transform.translation.y.clamp(camera_height / 2.0, level_dimension.height as f32 - (camera_height / 2.0));
    }
}