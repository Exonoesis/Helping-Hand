use crate::{
    entities::player::Player,
    visuals::map::{GridDimensions, PxDimensions},
};
use bevy::prelude::*;

pub fn follow_player(
    level_dimension: Query<&PxDimensions, With<GridDimensions>>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<
        (&mut Transform, &OrthographicProjection),
        (With<Camera2d>, Without<Player>),
    >,
) {
    // if camera_query.is_empty() {
    //     return;
    // }

    // if player_query.is_empty() {
    //     return;
    // }

    // if level_dimension.is_empty() {
    //     return;
    // }

    // let level_dimensions = level_dimension.single();
    // let level_height = level_dimensions.get_height();
    // let level_width = level_dimensions.get_width();
    // if level_height == 0 || level_width == 0 {
    //     return;
    // }

    // let (mut camera_transform, camera_bounds) = camera_query
    //     .get_single_mut()
    //     .expect("move_camera: could not find camera");
    // let player_transform = player_query
    //     .get_single()
    //     .expect("move_camera: could not find player");

    // let camera_width = camera_bounds.area.width();
    // let camera_height = camera_bounds.area.height();

    // // Since a tile's physical px position is the bottom left corner,
    // // to center the camera on a tile we need to offset by half a tile
    // // along the x and y axis'
    // //
    // // TO-DO: Get this value from query
    // let tile_offset = 32.0;

    // if camera_width > level_width as f32 {
    //     camera_transform.translation.x = level_width as f32 / 2.0;
    // } else {
    //     camera_transform.translation.x = player_transform.translation.x.clamp(
    //         (camera_width / 2.0) - tile_offset,
    //         level_width as f32 - (camera_width / 2.0) - tile_offset,
    //     );
    // }

    // if camera_height > level_height as f32 {
    //     camera_transform.translation.y = level_height as f32 / 2.0;
    // } else {
    //     camera_transform.translation.y = player_transform.translation.y.clamp(
    //         (camera_height / 2.0) - tile_offset,
    //         level_height as f32 - (camera_height / 2.0) - tile_offset,
    //     );
    // }
}

pub fn bound_camera() {}
