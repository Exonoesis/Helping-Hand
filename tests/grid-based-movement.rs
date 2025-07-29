mod mock_game;

use crate::mock_game::Game;

use cucumber::{given, then, when, World};

use helping_hand::{
    map::{
        interactions::map_changing::LoadLevel, movement::grid_based_movement::*, player::Player,
        XyzCords,
    },
    plugins::levels::CoreLevelsPlugin,
};

/// Converts a string into a MovementDirection
fn convert_string_to_movement_direction(movement_string: String) -> MovementDirection {
    let movement_direction_event = match movement_string.as_str() {
        "left" => MovementDirection::Left,
        "right" => MovementDirection::Right,
        "up" => MovementDirection::Up,
        "down" => MovementDirection::Down,
        _ => panic!(
            "convert_string_to_movement_direction: Invalid direction given: {}",
            movement_string
        ),
    };

    movement_direction_event
}

#[given(regex = r"a Tiled map called (.+),")]
fn given_some_tiled_map(game: &mut Game, tiled_map_name: String) {
    game.add_plugin(CoreLevelsPlugin);

    let map_path = format!("tests/test-assets/maps/{}", tiled_map_name);
    game.broadcast_event(LoadLevel::new(&map_path));
}

#[given(regex = r"the Player is at ([0-9]+),([0-9]+),([0-9]+),")]
fn verify_player_spawned_at_tile_pos(game: &mut Game, tile_x: u32, tile_y: u32, tile_z: usize) {
    let expected_player_tile_coordinate = XyzCords::new_u32(tile_x, tile_y, tile_z);
    let actual_player_tile_coordinate = game.find_coordinates_of_player();
    assert_eq!(
        expected_player_tile_coordinate,
        actual_player_tile_coordinate
    );
}

#[when(regex = r"the Player is requested to move ([a-zA-Z]+),")]
fn request_player_to_move(game: &mut Game, movement_direction: String) {
    let movement_direction_event = convert_string_to_movement_direction(movement_direction);
    game.broadcast_event(movement_direction_event);
}

#[when(regex = r"the Player moves ([a-zA-Z]+),")]
fn move_player_in_direction(game: &mut Game, movement_direction: String) {
    request_player_to_move(game, movement_direction);

    for _i in 0..255 {
        game.tick();

        let has_traveled = game.get_number_of::<Target>() == 0;
        if has_traveled {
            break;
        }
    }
}

#[then(
    regex = r"the Player's pixel coordinates are equivalent to tile ([0-9]+),([0-9]+),([0-9]+)."
)]
fn verify_player_at_tile_pixel_coordinates(
    game: &mut Game,
    tile_x: u32,
    tile_y: u32,
    tile_z: usize,
) {
    let tile_grid_coordinates = XyzCords::new_u32(tile_x, tile_y, tile_z);

    let expected_player_position = game.get_position_from_tile(&tile_grid_coordinates);
    let actual_player_position = game.get_player_position();
    assert_eq!(expected_player_position, actual_player_position);
}

#[then(regex = r"the Player's grid coordinates are set to tile ([0-9]+),([0-9]+),([0-9]+).")]
fn verify_player_at_tile_grid_coordinates(
    game: &mut Game,
    tile_x: u32,
    tile_y: u32,
    tile_z: usize,
) {
    let expected_player_tile_grid_coordinate = XyzCords::new_u32(tile_x, tile_y, tile_z);
    let actual_player_tile_grid_coordinate = game.find_coordinates_of_player();

    assert_eq!(
        expected_player_tile_grid_coordinate,
        actual_player_tile_grid_coordinate
    );
}

#[then("the Player should have a Target.")]
fn verify_player_has_target(game: &mut Game) {
    let player_has_target = game.has::<Player, Target>();
    assert!(player_has_target);
}

#[then(regex = r"the Player is facing ([a-zA-Z]+).")]
fn verify_player_facing_direction(game: &mut Game, facing_direction: String) {
    let expected_facing_direction = convert_string_to_movement_direction(facing_direction);
    let actual_facing_direction = game.get_player_facing_direction();
    assert_eq!(expected_facing_direction, actual_facing_direction);
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run(
        "tests/feature-files/in-practice/grid-based-movement.feature",
    ));
}
