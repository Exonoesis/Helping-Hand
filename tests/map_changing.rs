mod mock_game;

use crate::mock_game::Game;
use cucumber::{given, then, when, World};

use helping_hand::{
    map::{interactions::map_changing::ChangeLevel, player::*, XyzCords},
    plugins::levels::MockLevelsPlugin,
};

#[given(regex = r"a Tiled map called (.+),")]
fn given_some_tiled_map(game: &mut Game, tiled_map_name: String) {
    game.add_plugin(MockLevelsPlugin);

    let map_path = format!("tests/test-assets/maps/{}", tiled_map_name);
    game.broadcast_event(ChangeLevel::new(&map_path));
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
    let expected_player_tile_coordinate = XyzCords::new_u32(tile_x, tile_y, 0);
    let actual_player_tile_coordinate = game.find_coordinates_of_player();

    let expected_player_tile_x = expected_player_tile_coordinate.get_x();
    let expected_player_tile_y = expected_player_tile_coordinate.get_y();

    let actual_player_tile_x = actual_player_tile_coordinate.get_x();
    let actual_player_tile_y = actual_player_tile_coordinate.get_y();

    assert_eq!(expected_player_tile_x, actual_player_tile_x);

    assert_eq!(expected_player_tile_y, actual_player_tile_y);
}

#[when("the player interacts with the tile ahead of them,")]
fn trigger_player_interaction(game: &mut Game) {
    game.broadcast_event(PlayerInteraction);

    for _ in 0..5 {
        game.tick();
    }
}

#[then(regex = r"the Player should be at ([0-9]+),([0-9]+).")]
fn verify_player_at_tile_pos(game: &mut Game, tile_x: u32, tile_y: u32) {
    let expected_player_tile_coordinate = XyzCords::new_u32(tile_x, tile_y, 0);
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

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run("tests/feature-files/map_changing.feature"));
}
