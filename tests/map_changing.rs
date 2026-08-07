mod mock_game;

use crate::mock_game::Game;
use cucumber::{given, then, when, World};

use helping_hand::{
    map::{interactions::map_changing::LoadLevel, player::*, GridCords3D, PxCords},
    plugins::levels::CoreLevelsPlugin,
};

#[given(regex = r"a Tiled map called (.+),")]
fn given_some_tiled_map(game: &mut Game, tiled_map_name: String) {
    game.add_plugin(CoreLevelsPlugin);

    let map_path = format!("tests/test_assets/maps/map_changing/{}", tiled_map_name);
    game.write_message(LoadLevel::new(&map_path));
    game.tick();
}

#[given(regex = r"a map size of ([0-9]+) x ([0-9]+) tiles,")]
fn given_some_map_size(game: &mut Game, expected_map_width: u32, expected_map_height: u32) {
    let map_dimensions = game.get_map_size();

    let actual_map_height = map_dimensions.get_rows();
    let actual_map_width = map_dimensions.get_columns();

    assert_eq!(expected_map_height, actual_map_height);
    assert_eq!(expected_map_width, actual_map_width);
}

#[given(regex = r"the Player is at ([0-9]+),([0-9]+),")]
fn verify_player_spawned_at_tile_pos(game: &mut Game, tile_x: u32, tile_y: u32) {
    let expected_player_tile_coordinate = GridCords3D::new_u32(tile_x, tile_y, 0);
    let actual_player_tile_coordinate = game.find_coordinates_of_player();

    let expected_player_tile_x = expected_player_tile_coordinate.get_x();
    let expected_player_tile_y = expected_player_tile_coordinate.get_y();

    let actual_player_tile_x = actual_player_tile_coordinate.get_x();
    let actual_player_tile_y = actual_player_tile_coordinate.get_y();

    assert_eq!(expected_player_tile_x, actual_player_tile_x);

    assert_eq!(expected_player_tile_y, actual_player_tile_y);
}

#[when("the Tiled map is loaded,")]
fn wait_for_map_load(game: &mut Game) {
    for _ in 0..5 {
        game.tick();
    }
}

#[when("the player interacts with the tile ahead of them,")]
fn trigger_player_interaction(game: &mut Game) {
    game.write_message(PlayerInteraction);

    for _ in 0..5 {
        game.tick();
    }
}

#[then(regex = r"the Player should be at ([0-9]+),([0-9]+).")]
fn verify_player_at_tile_pos(game: &mut Game, tile_x: u32, tile_y: u32) {
    let expected_player_tile_coordinate = GridCords3D::new_u32(tile_x, tile_y, 0);
    let actual_player_tile_coordinate = game.find_coordinates_of_player();

    let expected_player_tile_x = expected_player_tile_coordinate.get_x();
    let expected_player_tile_y = expected_player_tile_coordinate.get_y();

    let actual_player_tile_x = actual_player_tile_coordinate.get_x();
    let actual_player_tile_y = actual_player_tile_coordinate.get_y();

    assert_eq!(expected_player_tile_x, actual_player_tile_x);

    assert_eq!(expected_player_tile_y, actual_player_tile_y);
}

#[then(regex = r"the map size should be ([0-9]+) x ([0-9]+) tiles,")]
fn verify_map_size(game: &mut Game, expected_map_width: u32, expected_map_height: u32) {
    let map_dimensions = game.get_map_size();

    let actual_map_height = map_dimensions.get_rows();
    let actual_map_width = map_dimensions.get_columns();

    assert_eq!(expected_map_height, actual_map_height);
    assert_eq!(expected_map_width, actual_map_width);
}

#[then(regex = r"there should be ([0-9]+) tiles")]
fn verify_tile_count(game: &mut Game, expected_tile_count: usize) {
    let actual_tile_count = game.get_number_of::<GridCords3D>();

    assert_eq!(expected_tile_count, actual_tile_count);
}

#[then(
    regex = r"the tile at grid coordinate ([0-9]+),([0-9]+),([0-9]+) has a pixel coordinate of ([0-9]+),([0-9]+),([0-9]+)."
)]
fn verify_y_axis_flip(
    game: &mut Game,
    grid_cord_x: usize,
    grid_cord_y: usize,
    grid_cord_z: usize,
    px_cord_x: usize,
    px_cord_y: usize,
    px_cord_z: usize,
) {
    let given_grid_cord = GridCords3D::new(grid_cord_x, grid_cord_y, grid_cord_z);
    let expected_px_cord = PxCords::new(px_cord_x, px_cord_y, px_cord_z);

    let transform = game.get_position_from_tile(&given_grid_cord);
    let actual_px_cord = PxCords::new(
        transform.translation.x as usize,
        transform.translation.y as usize,
        transform.translation.z as usize,
    );

    assert_eq!(expected_px_cord, actual_px_cord);
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run(
        "tests/feature_files/in-practice/map_changing.feature",
    ));
}
