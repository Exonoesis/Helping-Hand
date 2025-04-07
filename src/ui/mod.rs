use bevy::prelude::*;

pub mod menus;

use crate::map::{player::Player, GridDimensions, PxDimensions};

pub fn follow_player(
    level_query: Query<(&PxDimensions, &GridDimensions)>,
    player_query: Query<(&Transform, &PxDimensions), (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<
        (&mut Transform, &OrthographicProjection),
        (With<Camera2d>, Without<Player>),
    >,
) {
    //Check for empties
    if camera_query.is_empty() {
        return;
    }
    if player_query.is_empty() {
        return;
    }
    if level_query.is_empty() {
        return;
    }

    //Start unpacking
    let (mut camera_transform, camera_bounds) = camera_query
        .get_single_mut()
        .expect("follow_player: could not find camera");
    let (player_transform, player_tile_dimensions) = player_query
        .get_single()
        .expect("follow_player: could not find player");
    let (level_dimensions, level_grid) = level_query.single();

    //Further unpacking
    let camera_width = camera_bounds.area.width();
    let camera_height = camera_bounds.area.height();

    //Helper variables
    let player_center_position =
        get_centered_player_position(player_transform, player_tile_dimensions);
    let player_center_x = player_center_position.translation.x;
    let player_center_y = player_center_position.translation.y;

    let level_height = level_dimensions.get_height() as f32;
    let level_width = level_dimensions.get_width() as f32;
    if level_height == 0.0 || level_width == 0.0 {
        return;
    }
    let level_center_x = level_width / 2.0;
    let level_center_y = level_height / 2.0;

    let tile_x_offset = (level_width / level_grid.get_columns() as f32) / 2.0;
    let tile_y_offset = (level_height / level_grid.get_rows() as f32) / 2.0;

    //Bounding limits
    let camera_min_x = (camera_width / 2.0) - tile_x_offset;
    let camera_max_x = (level_width - (camera_width / 2.0)) - tile_x_offset;
    let camera_min_y = (camera_height / 2.0) - tile_y_offset;
    let camera_max_y = (level_height - (camera_height / 2.0)) - tile_y_offset;

    //Logic
    if camera_width > level_width {
        camera_transform.translation.x = level_center_x - tile_x_offset;
    } else {
        camera_transform.translation.x = player_center_x.clamp(camera_min_x, camera_max_x);
    }

    if camera_height > level_height {
        camera_transform.translation.y = level_center_y - tile_y_offset;
    } else {
        camera_transform.translation.y = player_center_y.clamp(camera_min_y, camera_max_y);
    }
}

/// Returns the pixel coordinates for the player's center in the game.
pub fn get_centered_player_position(
    player_position: &Transform,
    player_tile_dimensions: &PxDimensions,
) -> Transform {
    let half_tile_width = player_tile_dimensions.get_width() as f32 / 2.0;
    let half_tile_height = player_tile_dimensions.get_height() as f32 / 2.0;

    let centered_player_position = Transform::from_xyz(
        player_position.translation.x + half_tile_width,
        player_position.translation.y + half_tile_height,
        player_position.translation.z,
    );

    centered_player_position
}
