mod mock_game;

use std::{path::PathBuf, time::Duration};

use crate::mock_game::Game;

use cucumber::{given, then, when, World};
use helping_hand::{
    map::GridCords3D,
    narrative::act_loading::{LoadAct, LoadNextScene},
    plugins::{acts::CoreActsPlugin, levels::CoreLevelsPlugin},
    AppState,
};

#[given(regex = r"the game is loaded with the act '(.+)',")]
fn load_plugin_and_act(game: &mut Game, act_file_name: String) {
    let fade_duration = Duration::from_secs(0);
    let maps_folder_path = PathBuf::from("tests/test_assets/maps/npc_movement/");

    game.add_plugin(CoreActsPlugin::new(fade_duration, maps_folder_path));
    game.add_plugin(CoreLevelsPlugin);

    let act_file_path_name = format!("tests/test_assets/acts/{}", act_file_name);
    let act_file_path = PathBuf::from(&act_file_path_name);

    assert!(
        act_file_path.exists(),
        "Act file does not exist at location {:?}",
        act_file_path.canonicalize().unwrap()
    );

    game.write_message(LoadAct::new(&act_file_path_name));
    game.tick();

    game.set_state(AppState::Transitioning);

    while game.get_state() == &AppState::Transitioning {
        game.tick();
    }
}

#[when(regex = r"the map cutscene '(.+)' is loaded,")]
fn load_scene(game: &mut Game, scene_name: String) {
    game.write_message(LoadNextScene::new(scene_name));
    game.tick();
}

#[when(regex = r"([0-9]+) steps have taken place,")]
fn take_number_of_steps(game: &mut Game, number_of_steps: usize) {
    for _ in 0..number_of_steps {
        game.tick();
    }
}

/*
#[then(
    regex = r"the NPC '(.+)' has pixel coordinates equivalent to tile ([0-9]+),([0-9]+),([0-9]+)."
)]
fn verify_npc_at_tile_pixel_coordinates(
    game: &mut Game,
    npc_name: String,
    tile_x: u32,
    tile_y: u32,
    tile_z: usize,
) {
    let tile_grid_coordinates = GridCords3D::new_u32(tile_x, tile_y, tile_z);

    let expected_npc_position = game.get_position_from_tile(&tile_grid_coordinates);
    let actual_npc_position = game.get_npc_position(&npc_name);
    assert_eq!(expected_npc_position, actual_npc_position);
}
*/

#[then(regex = r"the NPC '(.+)' has grid coordinates set to tile ([0-9]+),([0-9]+),([0-9]+).")]
fn verify_npc_at_tile_grid_coordinates(
    game: &mut Game,
    npc_name: String,
    tile_x: u32,
    tile_y: u32,
    tile_z: usize,
) {
    let expected_npc_tile_grid_coordinate = GridCords3D::new_u32(tile_x, tile_y, tile_z);
    let actual_npc_tile_grid_coordinate = game.get_npc_coordinate(&npc_name);
    assert_eq!(
        expected_npc_tile_grid_coordinate,
        actual_npc_tile_grid_coordinate
    );
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run(
        "tests/feature_files/in-practice/npc_movement.feature",
    ));
}
